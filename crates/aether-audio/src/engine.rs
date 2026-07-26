use crate::decoder::AudioDecoder;
use crate::dsp::{Equalizer, VolumeController};
use crate::output::AudioOutputDevice;
use crate::ring_buffer::create_audio_ring_buffer;
use aether_core::{AetherError, PlayState, PlayerCommand, PlayerEvent, Result};
use crossbeam_channel::{Receiver, Sender};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

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
}

pub struct HeadlessAudioEngine {
    command_receiver: Receiver<PlayerCommand>,
    event_sender: Sender<PlayerEvent>,
    volume: VolumeController,
    equalizer: Equalizer,
    state: PlayState,
    current_file: Option<PathBuf>,
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
        let mut pcm_batch = Vec::with_capacity(4096);

        while is_running.load(Ordering::Relaxed) {
            while let Ok(cmd) = self.command_receiver.try_recv() {
                self.handle_command(cmd, &mut active_decoder);
            }

            if self.state == PlayState::Playing {
                if let Some(decoder) = active_decoder.as_mut() {
                    pcm_batch.clear();
                    match decoder.decode_next(&mut pcm_batch) {
                        Ok(has_more) => {
                            if !has_more {
                                self.state = PlayState::Stopped;
                                active_decoder = None;
                                let _ = self.event_sender.send(PlayerEvent::TrackEnded);
                                let _ = self
                                    .event_sender
                                    .send(PlayerEvent::StateChanged(PlayState::Stopped));
                            } else {
                                self.equalizer.process(&mut pcm_batch);
                                self.volume.process(&mut pcm_batch);

                                for sample in pcm_batch.drain(..) {
                                    while producer.is_full() {
                                        thread::sleep(std::time::Duration::from_millis(1));
                                    }
                                    let _ = producer.push(sample);
                                }
                            }
                        }
                        Err(e) => {
                            let _ = self
                                .event_sender
                                .send(PlayerEvent::ErrorOccurred(e.to_string()));
                            self.state = PlayState::Stopped;
                            active_decoder = None;
                        }
                    }
                } else {
                    self.state = PlayState::Stopped;
                }
            } else {
                thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }

    fn handle_command(&mut self, cmd: PlayerCommand, active_decoder: &mut Option<AudioDecoder>) {
        match cmd {
            PlayerCommand::LoadTrack(path) => match AudioDecoder::open(&path) {
                Ok(decoder) => {
                    self.current_file = Some(path);
                    *active_decoder = Some(decoder);
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
                let _ = self
                    .event_sender
                    .send(PlayerEvent::StateChanged(PlayState::Stopped));
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
