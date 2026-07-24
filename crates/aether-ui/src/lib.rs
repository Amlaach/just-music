pub mod bidi;
pub mod state;
pub mod theme;
pub mod virtual_list;

pub use bidi::{BiDiEngine, LayoutDirection, TextAlignment};
pub use state::{AppState, StateStore};
pub use theme::{ColorPalette, DesignSystem, ThemeMode};
pub use virtual_list::{VirtualListCalculator, VisibleWindow};

pub fn ui_init() {
    tracing::info!("Aether UI Layer (Design System & RTL/LTR BiDi Engine) initialized");
}
