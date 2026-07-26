pub mod app;
pub mod associations;
pub mod bidi;
pub mod state;
pub mod theme;
pub mod toast;
pub mod views;
pub mod virtual_list;

pub use app::JustMusicApp;
pub use associations::AssociationManager;
pub use bidi::{BiDiEngine, LayoutDirection, TextAlignment};
pub use state::{AppState, StateStore};
pub use theme::{ColorPalette, DesignSystem, ThemeMode};
pub use toast::ToastManager;
pub use virtual_list::{VirtualListCalculator, VisibleWindow};

pub fn ui_init() {
    tracing::info!("Just Music GUI Layer initialized");
}
