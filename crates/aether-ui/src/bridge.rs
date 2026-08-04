use slint::{Timer, TimerMode, SharedString, ModelRc, VecModel};
use std::rc::Rc;
use std::time::Duration;
use std::path::PathBuf;
use aether_audio::AudioEngineHandle;
use aether_core::{PlayerCommand, PlayerEvent, PlayState, Track, AudioFormat};
use crate::state::{AppState, ThemeMode};

// Import the generated Slint module — this brings in MainWindow, Theme, Track (Slint struct), etc.
slint::include_modules!();

/// Main application struct that owns the Slint window and bridges to the audio engine.
pub struct JostMusicApp {
    window: MainWindow,
    pub state: AppState,
    _event_timer: Timer,
    pub audio_handle: Option<AudioEngineHandle>,
}

impl JostMusicApp {
    pub fn new(audio_handle: Option<AudioEngineHandle>) -> Self {
        let window = MainWindow::new().unwrap();
        let mut state = AppState::load_saved();

        // Apply saved theme settings to Slint globals
        apply_theme(&window, &state);

        // Apply saved settings to window properties
        apply_settings(&window, &state);

        // Setup all callbacks from Slint -> Rust
        setup_playback_callbacks(&window, audio_handle.clone());
        setup_file_callbacks(&window, audio_handle.clone());
        setup_window_callbacks(&window);
        setup_settings_callbacks(&window);

        // Start polling audio events (16ms timer)
        let event_timer = start_event_poller(&window, audio_handle.clone());

        // Auto-scan music folder if playlist empty
        if state.playlist.is_empty() {
            scan_default_music_folder(&window, &mut state);
        }

        Self {
            window,
            state,
            _event_timer: event_timer,
            audio_handle,
        }
    }

    /// Load a file from CLI argument or drag-and-drop.
    pub fn load_file(&self, path: PathBuf) {
        if let Some(handle) = &self.audio_handle {
            let _ = handle.send_command(PlayerCommand::LoadTrack(path));
        }
    }

    /// Run the Slint event loop (blocking).
    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.window.run()
    }
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

fn apply_theme(window: &MainWindow, state: &AppState) {
    let theme = window.global::<Theme>();

    match state.settings.theme_mode {
        ThemeMode::Light => {
            theme.set_is_dark(false);
            theme.set_is_deep_night_red(false);
            window.set_active_theme(0);
        }
        ThemeMode::Dark => {
            theme.set_is_dark(true);
            theme.set_is_deep_night_red(false);
            window.set_active_theme(1);
        }
        ThemeMode::DeepNightRed => {
            theme.set_is_dark(true);
            theme.set_is_deep_night_red(true);
            window.set_active_theme(2);
        }
    }

    theme.set_is_rtl(state.settings.is_rtl);
    theme.set_language(SharedString::from(if state.settings.is_rtl { "he" } else { "en" }));

    window.set_is_rtl(state.settings.is_rtl);
    window.set_language(SharedString::from(if state.settings.is_rtl { "he" } else { "en" }));
}

fn apply_settings(window: &MainWindow, state: &AppState) {
    window.set_start_with_windows(state.settings.start_with_windows);
    window.set_remember_volume(state.settings.remember_volume);
    window.set_restore_playlist(state.settings.restore_last_playlist);
    window.set_minimize_to_tray(state.settings.minimize_to_tray);
    window.set_playback_speed(state.settings.playback_speed);
    window.set_buffer_size(state.settings.buffer_size_ms as i32);
    window.set_crossfade(state.settings.crossfade_sec as i32);
    window.set_gapless_playback(state.settings.gapless_playback);
    window.set_volume(state.volume);
}

// ---------------------------------------------------------------------------
// Playback Callbacks
// ---------------------------------------------------------------------------

fn setup_playback_callbacks(window: &MainWindow, audio_handle: Option<AudioEngineHandle>) {
    // Play / Pause
    {
        let handle = audio_handle.clone();
        window.on_play_pause(move || {
            if let Some(h) = &handle {
                let _ = h.send_command(PlayerCommand::TogglePlayPause);
            }
        });
    }

    // Stop
    {
        let handle = audio_handle.clone();
        window.on_stop(move || {
            if let Some(h) = &handle {
                let _ = h.send_command(PlayerCommand::Stop);
            }
        });
    }

    // Next Track
    {
        let handle = audio_handle.clone();
        window.on_next_track(move || {
            if let Some(h) = &handle {
                let _ = h.send_command(PlayerCommand::NextTrack);
            }
        });
    }

    // Previous Track
    {
        let handle = audio_handle.clone();
        window.on_prev_track(move || {
            if let Some(h) = &handle {
                let _ = h.send_command(PlayerCommand::PreviousTrack);
            }
        });
    }

    // Seek — receives progress ratio 0.0-1.0
    {
        let handle = audio_handle.clone();
        let window_weak = window.as_weak();
        window.on_seek(move |progress| {
            if let Some(h) = &handle {
                // We need total_ms to compute the seek target.
                // Read it from the window's total-time or keep it in state.
                // For now, store total_ms in a side channel or approximate.
                // The MainWindow doesn't expose total_ms directly, but we can
                // parse the total-time string or keep state. We'll approximate:
                if let Some(w) = window_weak.upgrade() {
                    let total_str = w.get_total_time().to_string();
                    let total_ms = parse_duration_string(&total_str);
                    let seek_ms = (progress as f64 * total_ms as f64) as u64;
                    let _ = h.send_command(PlayerCommand::SeekTo(Duration::from_millis(seek_ms)));
                }
            }
        });
    }

    // Set Volume
    {
        let handle = audio_handle.clone();
        window.on_set_volume(move |volume| {
            if let Some(h) = &handle {
                let _ = h.send_command(PlayerCommand::SetVolume(volume));
            }
        });
    }

    // Toggle Mute
    {
        let handle = audio_handle.clone();
        let window_weak = window.as_weak();
        window.on_toggle_mute(move || {
            if let Some(h) = &handle {
                if let Some(w) = window_weak.upgrade() {
                    let currently_muted = w.get_is_muted();
                    let _ = h.send_command(PlayerCommand::SetMute(!currently_muted));
                    w.set_is_muted(!currently_muted);
                }
            }
        });
    }
}

// ---------------------------------------------------------------------------
// File Callbacks
// ---------------------------------------------------------------------------

fn setup_file_callbacks(window: &MainWindow, audio_handle: Option<AudioEngineHandle>) {
    // Open File dialog
    {
        let handle = audio_handle.clone();
        window.on_open_file(move || {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter(
                    "Audio Files",
                    &["mp3", "flac", "wav", "aac", "ogg", "opus", "m4a", "aiff"],
                )
                .pick_file()
            {
                if let Some(h) = &handle {
                    let _ = h.send_command(PlayerCommand::LoadTrack(path));
                }
            }
        });
    }

    // Scan Folder
    {
        let window_weak = window.as_weak();
        window.on_scan_folder(move || {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                if let Some(w) = window_weak.upgrade() {
                    let tracks = scan_music_folder(&folder);
                    let model = tracks_to_slint_model(&tracks);
                    w.set_tracks(model);
                }
            }
        });
    }

    // Play specific track by index
    {
        let handle = audio_handle.clone();
        let window_weak = window.as_weak();
        window.on_play_track(move |index| {
            // This would need to look up the file path from the track model.
            // For now, we signal intent — the actual file path lookup
            // requires maintaining a parallel Vec<Track> in Rust.
            tracing::info!("Play track request: index {}", index);
        });
    }
}

// ---------------------------------------------------------------------------
// Window Control Callbacks
// ---------------------------------------------------------------------------

fn setup_window_callbacks(window: &MainWindow) {
    let window_weak = window.as_weak();
    window.on_minimize_window(move || {
        if let Some(w) = window_weak.upgrade() {
            w.window().set_minimized(true);
        }
    });

    let window_weak = window.as_weak();
    window.on_maximize_window(move || {
        if let Some(w) = window_weak.upgrade() {
            let is_max = w.window().is_maximized();
            w.window().set_maximized(!is_max);
        }
    });

    let window_weak = window.as_weak();
    window.on_close_window(move || {
        if let Some(w) = window_weak.upgrade() {
            // Save config before closing
            let _ = w.hide();
            slint::quit_event_loop().ok();
        }
    });
}

// ---------------------------------------------------------------------------
// Settings Callbacks
// ---------------------------------------------------------------------------

fn setup_settings_callbacks(window: &MainWindow) {
    // Theme changes are handled by the Slint two-way binding on active-theme.
    // The Rust side can observe changes if needed for saving config.
    // For now, the settings view directly modifies the Theme global properties.
}

// ---------------------------------------------------------------------------
// Event Polling
// ---------------------------------------------------------------------------

fn start_event_poller(window: &MainWindow, audio_handle: Option<AudioEngineHandle>) -> Timer {
    let timer = Timer::default();
    let window_weak = window.as_weak();

    timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
        let Some(window) = window_weak.upgrade() else {
            return;
        };
        let Some(handle) = &audio_handle else {
            return;
        };

        while let Ok(event) = handle.event_receiver().try_recv() {
            match event {
                PlayerEvent::StateChanged(play_state) => {
                    window.set_is_playing(play_state == PlayState::Playing);
                }
                PlayerEvent::TrackStarted(track) => {
                    window.set_current_track_title(SharedString::from(&track.title));
                    window.set_current_track_artist(SharedString::from(&track.artist));
                    window.set_total_time(SharedString::from(format_duration(track.duration_ms)));
                    window.set_is_playing(true);
                }
                PlayerEvent::TrackEnded => {
                    window.set_is_playing(false);
                    window.set_current_time(SharedString::from("0:00"));
                    window.set_progress(0.0);
                }
                PlayerEvent::PositionUpdated(pos) => {
                    window.set_current_time(SharedString::from(format_duration(pos.current_ms)));
                    window.set_progress(pos.progress_ratio);
                }
                PlayerEvent::VolumeChanged(vol) => {
                    window.set_volume(vol);
                }
                PlayerEvent::MuteChanged(muted) => {
                    window.set_is_muted(muted);
                }
                PlayerEvent::QueueUpdated(tracks) => {
                    let model = tracks_to_slint_model(&tracks);
                    window.set_tracks(model);
                }
                PlayerEvent::ErrorOccurred(err) => {
                    tracing::error!("Audio engine error: {}", err);
                }
            }
        }
    });

    timer
}

// ---------------------------------------------------------------------------
// Music Folder Scanning
// ---------------------------------------------------------------------------

fn scan_default_music_folder(window: &MainWindow, state: &mut AppState) {
    if let Ok(home) = std::env::var("USERPROFILE") {
        let music_dir = PathBuf::from(home).join("Music");
        if music_dir.exists() {
            tracing::info!("Auto-scanning music directory: {:?}", music_dir);
            let tracks = scan_music_folder(&music_dir);
            if !tracks.is_empty() {
                state.playlist = tracks.clone();
                let model = tracks_to_slint_model(&tracks);
                window.set_tracks(model);
            }
        }
    }
}

fn scan_music_folder(dir: &PathBuf) -> Vec<Track> {
    let audio_extensions = ["mp3", "flac", "wav", "aac", "ogg", "opus", "m4a", "aiff", "alac"];
    let mut tracks = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if audio_extensions.contains(&ext.to_lowercase().as_str()) {
                        let filename = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("Unknown")
                            .to_string();

                        tracks.push(Track {
                            id: aether_core::TrackId::new(),
                            file_path: path.clone(),
                            title: filename,
                            artist: "Unknown Artist".to_string(),
                            album: "Unknown Album".to_string(),
                            genre: None,
                            year: None,
                            track_number: None,
                            duration_ms: 0,
                            bitrate: None,
                            sample_rate: 44100,
                            channels: 2,
                            format: AudioFormat::from_extension(ext),
                            replaygain_track_gain: None,
                            replaygain_track_peak: None,
                            play_count: 0,
                            rating: 0,
                        });
                    }
                }
            }
        }
    }

    tracks
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Format milliseconds into "m:ss" display string.
fn format_duration(ms: u64) -> String {
    let total_secs = ms / 1000;
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;
    format!("{}:{:02}", minutes, seconds)
}

/// Parse a "m:ss" string back to milliseconds (for seek calculations).
fn parse_duration_string(s: &str) -> u64 {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() == 2 {
        let minutes: u64 = parts[0].parse().unwrap_or(0);
        let seconds: u64 = parts[1].parse().unwrap_or(0);
        (minutes * 60 + seconds) * 1000
    } else {
        0
    }
}

/// Convert a Vec<Track> (core domain) into a Slint ModelRc for the UI.
fn tracks_to_slint_model(tracks: &[Track]) -> ModelRc<home_view::Track> {
    let items: Vec<home_view::Track> = tracks
        .iter()
        .map(|track| home_view::Track {
            title: SharedString::from(&track.title),
            artist: SharedString::from(&track.artist),
            album: SharedString::from(&track.album),
            duration: SharedString::from(format_duration(track.duration_ms)),
            is_selected: false,
            is_playing: false,
        })
        .collect();

    ModelRc::from(Rc::new(VecModel::from(items)))
}
