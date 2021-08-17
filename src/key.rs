use serde::{Deserialize, Serialize};

use autopilot::key::{Character, Code, KeyCode, KeyCodeConvertible};


    pub fn get_key_code(arg: &String) -> Option<Code> {
        match arg.to_lowercase().trim() {
            "f1" => Some(Code(KeyCode::F1)),
            "f2" => Some(Code(KeyCode::F2)),
            "f3" => Some(Code(KeyCode::F3)),
            "f4" => Some(Code(KeyCode::F4)),
            "f5" => Some(Code(KeyCode::F5)),
            "f6" => Some(Code(KeyCode::F6)),
            "f7" => Some(Code(KeyCode::F7)),
            "f8" => Some(Code(KeyCode::F8)),
            "f9" => Some(Code(KeyCode::F9)),
            "f10" => Some(Code(KeyCode::F10)),
            "f11" => Some(Code(KeyCode::F11)),
            "f12" => Some(Code(KeyCode::F12)),
            "leftArrow" => Some(Code(KeyCode::LeftArrow)),
            "rightArrow" => Some(Code(KeyCode::RightArrow)),
            "downArrow" => Some(Code(KeyCode::DownArrow)),
            "upArrow" => Some(Code(KeyCode::UpArrow)),
            "end" => Some(Code(KeyCode::End)),
            "pageUp" => Some(Code(KeyCode::PageUp)),
            "pageDown" => Some(Code(KeyCode::PageDown)),
            "delete" => Some(Code(KeyCode::Delete)),
            "home" => Some(Code(KeyCode::Home)),
            "escape" | "esc" => Some(Code(KeyCode::Escape)),
            "backspace" => Some(Code(KeyCode::Backspace)),
            "tab" => Some(Code(KeyCode::Tab)),
            "space" => Some(Code(KeyCode::Space)),
            _ => None,
        }
    }
