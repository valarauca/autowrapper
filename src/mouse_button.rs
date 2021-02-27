use autopilot::mouse::Button as AutoPilotButton;
use serde::{
    de::{Deserialize, Visitor},
    ser::Serialize,
    Deserializer, Serializer,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Button {
    Left,
    Right,
    Middle,
}

impl Button {
    #[allow(dead_code)]
    pub(in crate) fn get_button(&self) -> AutoPilotButton {
        match self {
            &Self::Left => AutoPilotButton::Left,
            &Self::Right => AutoPilotButton::Right,
            &Self::Middle => AutoPilotButton::Middle,
        }
    }
}

/*
 * An idiotic amount of boilerplate b/c serde can't do this automatically
 *
 */

struct ButtonVisitor;

impl<'de> Visitor<'de> for ButtonVisitor {
    type Value = Button;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "expected on of the strings: ['left', 'right', or 'middle']"
        )
    }

    fn visit_str<E>(self, arg: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match arg.trim() {
            "left" | "LEFT" => Ok(Button::Left),
            "right" | "RIGHT" => Ok(Button::Right),
            "middle" | "MIDDLE" => Ok(Button::Middle),
            x => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(x),
                &self,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ButtonVisitor)
    }
}

impl Serialize for Button {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            &Button::Left => s.serialize_str("left"),
            &Button::Right => s.serialize_str("right"),
            &Button::Middle => s.serialize_str("middle"),
        }
    }
}
