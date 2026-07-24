use crate::bidi::{BiDiEngine, LayoutDirection};
use crate::theme::DesignSystem;
use aether_core::{PlayState, PlaybackPosition, Track};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub current_track: Option<Track>,
    pub play_state: PlayState,
    pub position: PlaybackPosition,
    pub volume: f32,
    pub is_muted: bool,
    pub search_query: String,
    pub search_results: Vec<Track>,
    pub design_system: DesignSystem,
    pub bidi_engine: BiDiEngine,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_track: None,
            play_state: PlayState::Stopped,
            position: PlaybackPosition {
                current_ms: 0,
                total_ms: 0,
                progress_ratio: 0.0,
            },
            volume: 0.85,
            is_muted: false,
            search_query: String::new(),
            search_results: Vec::new(),
            design_system: DesignSystem::default(),
            bidi_engine: BiDiEngine::new(LayoutDirection::Rtl),
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
