use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: Color32,
    pub primary_hover: Color32,
    pub primary_pressed: Color32,
    pub accent: Color32,

    pub background: Color32,
    pub cards: Color32,
    pub borders: Color32,

    pub text_primary: Color32,
    pub text_secondary: Color32,
}

impl ColorPalette {
    pub fn light() -> Self {
        Self {
            primary: Color32::from_rgb(217, 74, 74),         // #D94A4A
            primary_hover: Color32::from_rgb(227, 92, 92),   // #E35C5C
            primary_pressed: Color32::from_rgb(197, 61, 61), // #C53D3D
            accent: Color32::from_rgb(241, 122, 122),        // #F17A7A

            background: Color32::from_rgb(250, 250, 250), // #FAFAFA
            cards: Color32::from_rgb(255, 255, 255),      // #FFFFFF
            borders: Color32::from_rgb(231, 231, 231),    // #E7E7E7

            text_primary: Color32::from_rgb(34, 34, 34), // #222222
            text_secondary: Color32::from_rgb(102, 102, 102), // #666666
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: Color32::from_rgb(217, 74, 74),         // #D94A4A
            primary_hover: Color32::from_rgb(227, 92, 92),   // #E35C5C
            primary_pressed: Color32::from_rgb(197, 61, 61), // #C53D3D
            accent: Color32::from_rgb(241, 122, 122),        // #F17A7A

            background: Color32::from_rgb(20, 20, 22), // Dark #141416
            cards: Color32::from_rgb(30, 30, 34),      // Dark Cards #1E1E22
            borders: Color32::from_rgb(45, 45, 52),    // Dark Borders #2D2D34

            text_primary: Color32::from_rgb(240, 240, 240), // Light Text #F0F0F0
            text_secondary: Color32::from_rgb(160, 160, 170), // Muted Text #A0A0AA
        }
    }
}

#[derive(Debug, Clone)]
pub struct DesignSystem {
    pub mode: ThemeMode,
    pub palette: ColorPalette,
}

impl DesignSystem {
    pub fn new(mode: ThemeMode) -> Self {
        let palette = match mode {
            ThemeMode::Light => ColorPalette::light(),
            ThemeMode::Dark | ThemeMode::System => ColorPalette::dark(),
        };
        Self { mode, palette }
    }
}

impl Default for DesignSystem {
    fn default() -> Self {
        Self::new(ThemeMode::Light)
    }
}
