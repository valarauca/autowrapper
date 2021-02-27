#[allow(unused_imports)]
use rand::{distributions::uniform::Uniform, distributions::Normal, thread_rng, Rng};
use serde::{Deserialize, Serialize};

/// Time describes the amount of milliseconds something should
/// wait for.
///
/// It can either be a fixed millisecond value, or a random
/// millisecond value, which will use a standard distribution
/// and the user can provide cut-off values optionally.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Time {
    Constant(u32),
    Uniform { min: u32, max: u32 },
}

pub trait GetTime {
    fn get_time<F: Into<Option<f64>>, T: From<u32>>(&self, default: F) -> T;
}

impl GetTime for Time {
    /// returns the time in question
    #[allow(dead_code)]
    fn get_time<F: Into<Option<f64>>, T: From<u32>>(&self, _default: F) -> T {
        match self {
            Self::Constant(ref arg) => T::from(arg.clone()),
            Self::Uniform { min, max } => T::from(thread_rng().sample(Uniform::new(min, max))),
        }
    }
}

impl GetTime for Option<Time> {
    #[allow(dead_code)]
    fn get_time<F: Into<Option<f64>>, T: From<u32>>(&self, default: F) -> T {
        match self {
            &Option::None => match default.into() {
                Option::None => T::from(1u32),
                Option::Some(default) => T::from(default as u32),
            },
            &Option::Some(ref time) => time.get_time(default),
        }
    }
}
