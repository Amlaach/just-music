use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutDirection {
    Ltr,
    Rtl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone)]
pub struct BiDiEngine {
    direction: LayoutDirection,
}

impl BiDiEngine {
    pub fn new(direction: LayoutDirection) -> Self {
        Self { direction }
    }

    pub fn direction(&self) -> LayoutDirection {
        self.direction
    }

    pub fn is_rtl(&self) -> bool {
        self.direction == LayoutDirection::Rtl
    }

    pub fn detect_text_direction(text: &str) -> LayoutDirection {
        for ch in text.chars() {
            if ('\u{0590}'..='\u{05FF}').contains(&ch)
                || ('\u{0600}'..='\u{06FF}').contains(&ch)
                || ('\u{0750}'..='\u{077F}').contains(&ch)
            {
                return LayoutDirection::Rtl;
            } else if ch.is_alphabetic() {
                return LayoutDirection::Ltr;
            }
        }
        LayoutDirection::Ltr
    }

    pub fn resolve_alignment(&self, align: TextAlignment) -> TextAlignment {
        if self.is_rtl() {
            match align {
                TextAlignment::Start => TextAlignment::End,
                TextAlignment::End => TextAlignment::Start,
                TextAlignment::Center => TextAlignment::Center,
            }
        } else {
            align
        }
    }
}

impl Default for BiDiEngine {
    fn default() -> Self {
        Self::new(LayoutDirection::Rtl)
    }
}
