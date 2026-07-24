use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub bg_primary: String,
    pub bg_secondary: String,
    pub bg_elevated: String,
    pub accent_primary: String,
    pub accent_hover: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub border_subtle: String,
    pub glass_glow: String,
}

impl ColorPalette {
    pub fn dark() -> Self {
        Self {
            bg_primary: "#0D0F12".into(),
            bg_secondary: "#16191E".into(),
            bg_elevated: "#1E232B".into(),
            accent_primary: "#00E5FF".into(),
            accent_hover: "#33EBFF".into(),
            text_primary: "#F0F4F8".into(),
            text_secondary: "#94A3B8".into(),
            text_muted: "#64748B".into(),
            border_subtle: "rgba(255, 255, 255, 0.08)".into(),
            glass_glow: "rgba(0, 229, 255, 0.15)".into(),
        }
    }

    pub fn light() -> Self {
        Self {
            bg_primary: "#F8FAFC".into(),
            bg_secondary: "#FFFFFF".into(),
            bg_elevated: "#F1F5F9".into(),
            accent_primary: "#0284C7".into(),
            accent_hover: "#0369A1".into(),
            text_primary: "#0F172A".into(),
            text_secondary: "#475569".into(),
            text_muted: "#94A3B8".into(),
            border_subtle: "rgba(0, 0, 0, 0.08)".into(),
            glass_glow: "rgba(2, 132, 199, 0.15)".into(),
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
            ThemeMode::Dark => ColorPalette::dark(),
            ThemeMode::Light => ColorPalette::light(),
        };
        Self { mode, palette }
    }
}

impl Default for DesignSystem {
    fn default() -> Self {
        Self::new(ThemeMode::Dark)
    }
}
