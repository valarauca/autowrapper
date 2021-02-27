use serde::{Deserialize, Serialize};

use autopilot::key::{Character, Code, KeyCode, KeyCodeConvertible};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged, rename_all = "lowercase")]
pub enum SpecialKey {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    #[serde(alias = "left_arrow")]
    LeftArrow,
    #[serde(alias = "right_arrow")]
    RightArrow,
    #[serde(alias = "down_arrow")]
    DownArrow,
    #[serde(alias = "up_arrow")]
    UpArrow,
    End,
    #[serde(alias = "page_up")]
    PageUp,
    #[serde(alias = "page_down")]
    PageDown,
    Delete,
    Home,
    Escape,
    Backspace,
    Tab,
    Space,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Key {
    Spec(SpecialKey),
    Char(char),
}
impl Key {
    /// autopilot does the weirdest shit
    pub(in crate) fn get_character(&self) -> Option<Character> {
        match self.clone() {
            Self::Char(c) => Some(Character(c)),
            _ => None,
        }
    }

    pub(in crate) fn get_key_code(&self) -> Option<Code> {
        match self.clone() {
            Self::Spec(SpecialKey::F1) => Some(Code(KeyCode::F1)),
            Self::Spec(SpecialKey::F2) => Some(Code(KeyCode::F2)),
            Self::Spec(SpecialKey::F3) => Some(Code(KeyCode::F3)),
            Self::Spec(SpecialKey::F4) => Some(Code(KeyCode::F4)),
            Self::Spec(SpecialKey::F5) => Some(Code(KeyCode::F5)),
            Self::Spec(SpecialKey::F6) => Some(Code(KeyCode::F6)),
            Self::Spec(SpecialKey::F7) => Some(Code(KeyCode::F7)),
            Self::Spec(SpecialKey::F8) => Some(Code(KeyCode::F8)),
            Self::Spec(SpecialKey::F9) => Some(Code(KeyCode::F9)),
            Self::Spec(SpecialKey::F10) => Some(Code(KeyCode::F10)),
            Self::Spec(SpecialKey::F11) => Some(Code(KeyCode::F11)),
            Self::Spec(SpecialKey::F12) => Some(Code(KeyCode::F12)),
            Self::Spec(SpecialKey::LeftArrow) => Some(Code(KeyCode::LeftArrow)),
            Self::Spec(SpecialKey::RightArrow) => Some(Code(KeyCode::RightArrow)),
            Self::Spec(SpecialKey::DownArrow) => Some(Code(KeyCode::DownArrow)),
            Self::Spec(SpecialKey::UpArrow) => Some(Code(KeyCode::UpArrow)),
            Self::Spec(SpecialKey::End) => Some(Code(KeyCode::End)),
            Self::Spec(SpecialKey::PageUp) => Some(Code(KeyCode::PageUp)),
            Self::Spec(SpecialKey::PageDown) => Some(Code(KeyCode::PageDown)),
            Self::Spec(SpecialKey::Delete) => Some(Code(KeyCode::Delete)),
            Self::Spec(SpecialKey::Home) => Some(Code(KeyCode::Home)),
            Self::Spec(SpecialKey::Escape) => Some(Code(KeyCode::Escape)),
            Self::Spec(SpecialKey::Backspace) => Some(Code(KeyCode::Backspace)),
            Self::Spec(SpecialKey::Tab) => Some(Code(KeyCode::Tab)),
            Self::Spec(SpecialKey::Space) => Some(Code(KeyCode::Space)),
            _ => None,
        }
    }
}
