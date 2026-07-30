use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    DeepNightRed,
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
            primary: Color32::from_rgb(139, 0, 0),          // Crimson #8B0000
            primary_hover: Color32::from_rgb(155, 17, 30),   // #9B111E
            primary_pressed: Color32::from_rgb(96, 0, 8),    // #600008
            accent: Color32::from_rgb(0, 229, 255),         // Cyan #00E5FF

            background: Color32::from_rgb(250, 250, 250), // #FAFAFA
            cards: Color32::from_rgb(255, 255, 255),      // #FFFFFF
            borders: Color32::from_rgb(231, 231, 231),    // #E7E7E7

            text_primary: Color32::from_rgb(34, 34, 34), // #222222
            text_secondary: Color32::from_rgb(102, 102, 102), // #666666
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: Color32::from_rgb(139, 0, 0),          // Primary Crimson #8B0000
            primary_hover: Color32::from_rgb(155, 17, 30),   // Crimson Bright #9B111E
            primary_pressed: Color32::from_rgb(96, 0, 8),    // Dark Crimson #600008
            accent: Color32::from_rgb(0, 229, 255),         // Accent Cyan #00E5FF

            background: Color32::from_rgb(10, 8, 9),      // Deep Main Dark #0A0809
            cards: Color32::from_rgb(22, 10, 14),         // Crimson Card Tint
            borders: Color32::from_rgb(60, 10, 20),       // Crimson Border Tint

            text_primary: Color32::from_rgb(241, 245, 249), // Off-white #F1F5F9
            text_secondary: Color32::from_rgb(148, 163, 184), // Light Grey #94A3B8
        }
    }

    pub fn deep_night_red() -> Self {
        Self {
            primary: Color32::from_rgb(139, 0, 0),          // Primary Crimson #8B0000
            primary_hover: Color32::from_rgb(155, 17, 30),   // Crimson Bright #9B111E
            primary_pressed: Color32::from_rgb(96, 0, 8),    // Dark Crimson #600008
            accent: Color32::from_rgb(0, 229, 255),         // Accent Cyan #00E5FF

            background: Color32::from_rgb(5, 2, 3),        // Ultra Deep Night Red #050203
            cards: Color32::from_rgb(14, 5, 8),           // Ultra Dark Cards
            borders: Color32::from_rgb(40, 5, 10),        // Borders

            text_primary: Color32::from_rgb(241, 245, 249), // Off-white #F1F5F9
            text_secondary: Color32::from_rgb(148, 163, 184), // Light Grey #94A3B8
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
            ThemeMode::DeepNightRed => ColorPalette::deep_night_red(),
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
