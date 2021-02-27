use serde::{
    de::{Deserialize, Visitor},
    ser::Serialize,
    Deserializer, Serializer,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Scroll {
    Up,
    Down,
}

impl Scroll {
    pub(in crate) fn get_scroll(&self) -> autopilot::mouse::ScrollDirection {
        match self {
            &Self::Up => autopilot::mouse::ScrollDirection::Up,
            &Self::Down => autopilot::mouse::ScrollDirection::Down,
        }
    }
}

/*
 * An idiotic amount of boilerplate b/c serde can't do this automatically
 *
 */

struct ScrollVisitor;

impl<'de> Visitor<'de> for ScrollVisitor {
    type Value = Scroll;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "expected on of the strings: ['up', or 'down']")
    }

    fn visit_str<E>(self, arg: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match arg.trim() {
            "up" | "UP" => Ok(Scroll::Up),
            "down" | "DOWN" => Ok(Scroll::Down),
            x => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(x),
                &self,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Scroll {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ScrollVisitor)
    }
}

impl Serialize for Scroll {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            &Scroll::Up => s.serialize_str("up"),
            &Scroll::Down => s.serialize_str("down"),
        }
    }
}
