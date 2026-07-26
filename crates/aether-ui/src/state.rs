use crate::bidi::{BiDiEngine, LayoutDirection};
use crate::theme::{DesignSystem, ThemeMode};
use crate::toast::ToastManager;
use aether_core::{PlayState, PlaybackPosition, Track};
use serde::{Deserialize, Serialize};
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

            theme_mode: ThemeMode::Light,
            animation_speed_ms: 150,
            rounded_corners: true,
            enable_blur: true,

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
    pub design_system: DesignSystem,
    pub bidi_engine: BiDiEngine,
    pub toast_manager: ToastManager,

    pub status_text: String,
}

impl Default for AppState {
    fn default() -> Self {
        let settings = AppSettings::default();
        let design_system = DesignSystem::new(settings.theme_mode);
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
            design_system,
            bidi_engine: BiDiEngine::new(LayoutDirection::Ltr),
            toast_manager: ToastManager::default(),
            status_text: "Ready".to_string(),
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
        let guard = self.state.read().unwrap();
        f(&guard)
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut AppState),
    {
        let mut guard = self.state.write().unwrap();
        f(&mut guard);
    }
}

impl Default for StateStore {
    fn default() -> Self {
        Self::new()
    }
}
