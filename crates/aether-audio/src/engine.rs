use crate::decoder::AudioDecoder;
use crate::dsp::{Equalizer, VolumeController};
use crate::output::AudioOutputDevice;
use crate::ring_buffer::create_audio_ring_buffer;
use aether_core::{AetherError, PlayState, PlaybackPosition, PlayerCommand, PlayerEvent, Result};
use crossbeam_channel::{Receiver, Sender};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

pub struct AudioEngineHandle {
    command_sender: Sender<PlayerCommand>,
    event_receiver: Receiver<PlayerEvent>,
    is_running: Arc<AtomicBool>,
}

impl AudioEngineHandle {
    pub fn send_command(&self, cmd: PlayerCommand) -> Result<()> {
        self.command_sender
            .send(cmd)
            .map_err(|e| AetherError::AudioEngine(format!("Failed to send command: {}", e)))
    }

    pub fn event_receiver(&self) -> &Receiver<PlayerEvent> {
        &self.event_receiver
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }
}

pub struct HeadlessAudioEngine {
    command_receiver: Receiver<PlayerCommand>,
    event_sender: Sender<PlayerEvent>,
    volume: VolumeController,
    equalizer: Equalizer,
    state: PlayState,
    current_file: Option<PathBuf>,
    decoded_samples_count: u64,
    total_duration_ms: u64,
}

impl HeadlessAudioEngine {
    pub fn spawn() -> Result<AudioEngineHandle> {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();
        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_clone = is_running.clone();

        thread::Builder::new()
            .name("aether-audio-engine".into())
            .spawn(move || {
                let mut engine = HeadlessAudioEngine {
                    command_receiver: cmd_rx,
                    event_sender: event_tx,
                    volume: VolumeController::new(),
                    equalizer: Equalizer::new(44100.0),
                    state: PlayState::Stopped,
                    current_file: None,
                    decoded_samples_count: 0,
                    total_duration_ms: 0,
                };
                engine.run_loop(is_running_clone);
            })
            .map_err(|e| {
                AetherError::AudioEngine(format!("Failed to spawn audio thread: {}", e))
            })?;

        Ok(AudioEngineHandle {
            command_sender: cmd_tx,
            event_receiver: event_rx,
            is_running,
        })
    }

    fn run_loop(&mut self, is_running: Arc<AtomicBool>) {
        const RING_BUFFER_SIZE: usize = 65536;
        let (mut producer, consumer) = create_audio_ring_buffer(RING_BUFFER_SIZE);

        let output_device = match AudioOutputDevice::new(consumer) {
            Ok(dev) => dev,
            Err(e) => {
                let _ = self
                    .event_sender
                    .send(PlayerEvent::ErrorOccurred(e.to_string()));
                return;
            }
        };

        self.equalizer
            .set_sample_rate(output_device.sample_rate() as f32);

        let mut active_decoder: Option<AudioDecoder> = None;
        let mut pcm_batch: Vec<f32> = Vec::with_capacity(4096);
        let mut pcm_offset: usize = 0;
        let mut last_position_report = Instant::now();

        while is_running.load(Ordering::Relaxed) {
            // Process commands non-blocking
            while let Ok(cmd) = self.command_receiver.try_recv() {
                self.handle_command(cmd, &mut active_decoder, &mut pcm_batch, &mut pcm_offset);
            }

            if self.state == PlayState::Playing {
                if let Some(decoder) = active_decoder.as_mut() {
                    // If current batch is completely pushed, decode next block
                    if pcm_offset >= pcm_batch.len() {
                        pcm_batch.clear();
                        pcm_offset = 0;
                        match decoder.decode_next(&mut pcm_batch) {
                            Ok(has_more) => {
                                if !has_more {
                                    self.state = PlayState::Stopped;
                                    active_decoder = None;
                                    let _ = self.event_sender.send(PlayerEvent::TrackEnded);
                                    let _ = self
                                        .event_sender
                                        .send(PlayerEvent::StateChanged(PlayState::Stopped));
                                    continue;
                                } else {
                                    self.equalizer.process(&mut pcm_batch);
                                    self.volume.process(&mut pcm_batch);
                                }
                            }
                            Err(e) => {
                                let _ = self
                                    .event_sender
                                    .send(PlayerEvent::ErrorOccurred(e.to_string()));
                                self.state = PlayState::Stopped;
                                active_decoder = None;
                                continue;
                            }
                        }
                    }

                    // Push as many samples from batch to ring buffer as capacity allows
                    while pcm_offset < pcm_batch.len() && !producer.is_full() {
                        if producer.push(pcm_batch[pcm_offset]).is_ok() {
                            pcm_offset += 1;
                            self.decoded_samples_count += 1;
                        } else {
                            break;
                        }
                    }

                    // Emit PlaybackPosition periodically (every ~100ms)
                    if last_position_report.elapsed() >= Duration::from_millis(100) {
                        last_position_report = Instant::now();
                        let sample_rate = decoder.sample_rate().max(1) as u64;
                        let channels = decoder.channels().max(1) as u64;
                        let current_ms =
                            (self.decoded_samples_count * 1000) / (sample_rate * channels);
                        let total_ms = self.total_duration_ms.max(1);
                        let progress_ratio = (current_ms as f32 / total_ms as f32).clamp(0.0, 1.0);

                        let _ = self.event_sender.send(PlayerEvent::PositionUpdated(
                            PlaybackPosition {
                                current_ms,
                                total_ms,
                                progress_ratio,
                            },
                        ));
                    }

                    // Short sleep if ring buffer full to prevent 100% CPU core spinning
                    if producer.is_full() {
                        thread::sleep(Duration::from_millis(5));
                    }
                } else {
                    self.state = PlayState::Stopped;
                }
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
    }

    fn handle_command(
        &mut self,
        cmd: PlayerCommand,
        active_decoder: &mut Option<AudioDecoder>,
        pcm_batch: &mut Vec<f32>,
        pcm_offset: &mut usize,
    ) {
        match cmd {
            PlayerCommand::LoadTrack(path) => match AudioDecoder::open(&path) {
                Ok(decoder) => {
                    self.total_duration_ms = decoder.duration_ms();
                    self.decoded_samples_count = 0;
                    self.current_file = Some(path);
                    *active_decoder = Some(decoder);
                    pcm_batch.clear();
                    *pcm_offset = 0;
                    self.state = PlayState::Playing;
                    let _ = self
                        .event_sender
                        .send(PlayerEvent::StateChanged(PlayState::Playing));
                }
                Err(e) => {
                    let _ = self
                        .event_sender
                        .send(PlayerEvent::ErrorOccurred(e.to_string()));
                }
            },
            PlayerCommand::Play => {
                if active_decoder.is_some() {
                    self.state = PlayState::Playing;
                    let _ = self
                        .event_sender
                        .send(PlayerEvent::StateChanged(PlayState::Playing));
                }
            }
            PlayerCommand::Pause => {
                self.state = PlayState::Paused;
                let _ = self
                    .event_sender
                    .send(PlayerEvent::StateChanged(PlayState::Paused));
            }
            PlayerCommand::TogglePlayPause => {
                self.state = match self.state {
                    PlayState::Playing => PlayState::Paused,
                    _ => PlayState::Playing,
                };
                let _ = self
                    .event_sender
                    .send(PlayerEvent::StateChanged(self.state));
            }
            PlayerCommand::Stop => {
                self.state = PlayState::Stopped;
                *active_decoder = None;
                pcm_batch.clear();
                *pcm_offset = 0;
                self.decoded_samples_count = 0;
                let _ = self
                    .event_sender
                    .send(PlayerEvent::StateChanged(PlayState::Stopped));
            }
            PlayerCommand::SeekTo(target_duration) => {
                if let Some(decoder) = active_decoder.as_mut() {
                    if let Ok(_actual_ts) = decoder.seek(target_duration) {
                        pcm_batch.clear();
                        *pcm_offset = 0;
                        let sr = decoder.sample_rate().max(1) as u64;
                        let ch = decoder.channels().max(1) as u64;
                        self.decoded_samples_count =
                            (target_duration.as_millis() as u64 * sr * ch) / 1000;
                    }
                }
            }
            PlayerCommand::SetVolume(vol) => {
                self.volume.set_volume(vol);
                let _ = self.event_sender.send(PlayerEvent::VolumeChanged(vol));
            }
            PlayerCommand::SetMute(muted) => {
                self.volume.set_mute(muted);
                let _ = self.event_sender.send(PlayerEvent::MuteChanged(muted));
            }
            PlayerCommand::SetEqualizerBand {
                band_index,
                gain_db,
            } => {
                self.equalizer.set_band_gain(band_index, gain_db);
            }
            PlayerCommand::SetEqualizerEnabled(enabled) => {
                self.equalizer.set_enabled(enabled);
            }
            _ => {}
        }
    }
}
