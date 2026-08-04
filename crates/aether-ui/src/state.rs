use aether_core::{PlayState, PlaybackPosition, Track};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavTab {
    Home,
    Playlist,
    Recent,
    Settings,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsTab {
    General,
    Playback,
    Appearance,
    Associations,
    Updates,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    DeepNight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub start_with_windows: bool,
    pub remember_volume: bool,
    pub restore_last_playlist: bool,
    pub minimize_to_tray: bool,
    pub auto_check_updates: bool,

    pub playback_speed: f32,
    pub buffer_size_ms: u32,
    pub crossfade_sec: u32,
    pub gapless_playback: bool,

    pub theme_mode: ThemeMode,
    pub animation_speed_ms: u32,
    pub rounded_corners: bool,
    pub enable_blur: bool,

    pub is_rtl: bool,
    pub file_associations_registered: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            start_with_windows: false,
            remember_volume: true,
            restore_last_playlist: true,
            minimize_to_tray: false,
            auto_check_updates: true,

            playback_speed: 1.0,
            buffer_size_ms: 100,
            crossfade_sec: 2,
            gapless_playback: true,

            theme_mode: ThemeMode::Dark,
            animation_speed_ms: 150,
            rounded_corners: true,
            enable_blur: true,

            is_rtl: true, // Default to RTL for Hebrew users
            file_associations_registered: false,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub current_tab: NavTab,
    pub current_settings_tab: SettingsTab,

    pub current_track: Option<Track>,
    pub playlist: Vec<Track>,
    pub recent_tracks: Vec<Track>,

    pub play_state: PlayState,
    pub position: PlaybackPosition,
    pub volume: f32,
    pub is_muted: bool,

    pub settings: AppSettings,

    pub status_text: String,
}

impl AppState {
    fn config_path() -> PathBuf {
        let dir = dirs_next::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("JustMusic");
        let _ = fs::create_dir_all(&dir);
        dir.join("config.json")
    }

    pub fn load_saved() -> Self {
        let mut state = AppState::default();
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(saved_settings) = serde_json::from_str::<AppSettings>(&content) {
                    state.settings = saved_settings;
                }
            }
        }
        state
    }

    pub fn save_config(&self) {
        let path = Self::config_path();
        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            let _ = fs::write(path, json);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let settings = AppSettings::default();
        Self {
            current_tab: NavTab::Home,
            current_settings_tab: SettingsTab::General,
            current_track: None,
            playlist: Vec::new(),
            recent_tracks: Vec::new(),
            play_state: PlayState::Stopped,
            position: PlaybackPosition {
                current_ms: 0,
                total_ms: 0,
                progress_ratio: 0.0,
            },
            volume: 0.85,
            is_muted: false,
            settings,
            status_text: "מוכן לנגינה".to_string(),
        }
    }
}

pub struct StateStore {
    state: Arc<RwLock<AppState>>,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::default())),
        }
    }

    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&AppState) -> R,
    {
        if let Ok(guard) = self.state.read() {
            f(&guard)
        } else {
            let fallback = AppState::default();
            f(&fallback)
        }
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut AppState),
    {
        if let Ok(mut guard) = self.state.write() {
            f(&mut guard);
        }
    }
}

impl Default for StateStore {
    fn default() -> Self {
        Self::new()
    }
}
