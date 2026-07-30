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

    pub fn process(&self, text: &str) -> String {
        if self.is_rtl() {
            process_bidi_text(text)
        } else {
            text.to_string()
        }
    }
}

impl Default for BiDiEngine {
    fn default() -> Self {
        Self::new(LayoutDirection::Rtl)
    }
}

pub fn bidi(text: impl AsRef<str>) -> String {
    process_bidi_text(text.as_ref())
}

pub fn process_bidi_text(input: &str) -> String {
    if !input.chars().any(is_hebrew_char) {
        return input.to_string();
    }

    let mut result = String::with_capacity(input.len());
    let mut current_segment = String::new();
    let mut is_current_hebrew = false;

    for ch in input.chars() {
        let is_heb = is_hebrew_char(ch) || (is_current_hebrew && is_hebrew_punct(ch));

        if is_heb == is_current_hebrew {
            current_segment.push(ch);
        } else {
            if !current_segment.is_empty() {
                if is_current_hebrew {
                    result.push_str(&reverse_hebrew_segment(&current_segment));
                } else {
                    result.push_str(&current_segment);
                }
                current_segment.clear();
            }
            is_current_hebrew = is_heb;
            current_segment.push(ch);
        }
    }

    if !current_segment.is_empty() {
        if is_current_hebrew {
            result.push_str(&reverse_hebrew_segment(&current_segment));
        } else {
            result.push_str(&current_segment);
        }
    }

    result
}

fn is_hebrew_char(ch: char) -> bool {
    ('\u{0590}'..='\u{05FF}').contains(&ch)
}

fn is_hebrew_punct(ch: char) -> bool {
    matches!(ch, ' ' | '.' | ',' | '!' | '?' | '"' | '\'' | '-' | '(' | ')')
}

fn reverse_hebrew_segment(segment: &str) -> String {
    segment
        .chars()
        .map(|ch| match ch {
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            '<' => '>',
            '>' => '<',
            c => c,
        })
        .rev()
        .collect()
}
