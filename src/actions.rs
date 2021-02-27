use std::thread::sleep_ms;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use rand::{distributions::uniform::Uniform, thread_rng, Rng};

use crate::key::Key;
use crate::mouse_button::Button;
use crate::position::Position;
use crate::scroll::Scroll;
use crate::time::{GetTime, Time};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "action")]
pub enum Actions {
    #[serde(alias = "delay")]
    Delay { wait: Time },
    #[serde(alias = "click")]
    Click { button: Button, delay: Option<Time> },
    #[serde(alias = "smooth_move_mouse")]
    SmoothMoveMouse {
        position: Position,
        duration: Option<Time>,
    },
    #[serde(alias = "move_mouse")]
    MoveMouse { position: Position },
    #[serde(alias = "scroll")]
    Scroll { direction: Scroll, clicks: u32 },
    #[serde(alias = "depress_mouse")]
    DepressButton { button: Button },
    #[serde(alias = "release_mouse")]
    ReleaseButton { button: Button },
    #[serde(alias = "tap_key")]
    TapKey { key: Key, delay: Option<Time> },
    #[serde(alias = "depress_key")]
    PressKey { key: Key },
    #[serde(alias = "release_key")]
    ReleaseKey { key: Key },
    #[serde(alias = "or")]
    Or { options: Vec<Actions> },
    #[serde(alias = "in_order")]
    InOrder { todo: Vec<Actions> },
    #[serde(alias = "repeat")]
    Repeat { todo: Vec<Actions>, times: usize },
    #[serde(alias = "ensure")]
    Ensure {
        task: Box<Actions>,
        time: Option<Time>,
    },
}

impl Actions {
    /// run the actions
    pub fn do_it(&self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        match self {
            &Self::Delay { ref wait } => {
                let time: u32 = wait.get_time(None);
                sleep_ms(time);
                Ok(())
            }
            &Self::Click {
                ref button,
                ref delay,
            } => {
                let delay: u64 = delay.get_time(Some(50.0));
                autopilot::mouse::click(button.get_button(), Some(delay));
                Ok(())
            }
            &Self::SmoothMoveMouse {
                ref position,
                ref duration,
            } => {
                let duration: f64 = duration.get_time(Some(1.0));
                let target_point = position.get_point();
                to_err(autopilot::mouse::smooth_move(target_point, Some(duration)))
            }
            &Self::MoveMouse { ref position } => {
                to_err(autopilot::mouse::move_to(position.get_point()))
            }
            &Self::Scroll {
                ref direction,
                ref clicks,
            } => {
                autopilot::mouse::scroll(direction.get_scroll(), *clicks);
                Ok(())
            }
            &Self::DepressButton { ref button } => {
                autopilot::mouse::toggle(button.get_button(), true);
                Ok(())
            }
            &Self::ReleaseButton { ref button } => {
                autopilot::mouse::toggle(button.get_button(), false);
                Ok(())
            }
            &Self::TapKey { ref key, ref delay } => {
                let delay: u64 = delay.get_time(None);

                match key.get_character() {
                    Option::None => {
                        match key.get_key_code() {
                            Option::None => {
                                panic!("impossible condition, illegal key press specified")
                            }
                            Option::Some(k) => {
                                autopilot::key::tap(&k, &[], delay, 0);
                            }
                        };
                    }
                    Option::Some(c) => {
                        autopilot::key::tap(&c, &[], delay, 0);
                    }
                };
                Ok(())
            }
            &Self::PressKey { ref key } => {
                match key.get_character() {
                    Option::None => {
                        match key.get_key_code() {
                            Option::None => {
                                panic!("impossible condition, illegal key press specified")
                            }
                            Option::Some(k) => {
                                autopilot::key::toggle(&k, true, &[], 0);
                            }
                        };
                    }
                    Option::Some(c) => {
                        autopilot::key::toggle(&c, true, &[], 0);
                    }
                };
                Ok(())
            }
            &Self::ReleaseKey { ref key } => {
                match key.get_character() {
                    Option::None => {
                        match key.get_key_code() {
                            Option::None => {
                                panic!("impossible condition, illegal key press specified")
                            }
                            Option::Some(k) => {
                                autopilot::key::toggle(&k, false, &[], 0);
                            }
                        };
                    }
                    Option::Some(c) => {
                        autopilot::key::toggle(&c, false, &[], 0);
                    }
                };
                Ok(())
            }
            &Self::Or { ref options } => {
                let index = thread_rng().sample(Uniform::new(0, options.len()));
                options[index].do_it()
            }
            &Self::InOrder { ref todo } => {
                for item in todo.iter() {
                    item.do_it()?;
                }
                Ok(())
            }
            &Self::Repeat {
                ref todo,
                ref times,
            } => {
                for _ in 0..times.clone() {
                    for do_it in todo {
                        do_it.do_it()?;
                    }
                }
                Ok(())
            }
            &Self::Ensure { ref task, ref time } => {
                let time: u32 = time.get_time(None);
                let now = Instant::now();
                task.do_it()?;
                let total_time = now.elapsed().as_millis() as u32;
                if total_time < time {
                    sleep_ms(time - total_time);
                }
                Ok(())
            }
        }
    }
}

/// rust can't figure this out on its own
#[inline(always)]
fn to_err<E: std::error::Error + 'static>(
    r: Result<(), E>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    match r {
        Ok(()) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

#[test]
fn test_todo_many() {
    use serde_json::from_str;

    let sample = r#"
{
    "action": "repeat",
    "times": 10,
    "todo": [
        {
            "action": "delay",
            "wait": {
                "max": 1000,
                "min": 650
            }
        },
        {
            "action": "click",
            "button": "left",
            "delay": 25
        }
    ]
}
"#;

    let action = from_str(sample).unwrap();
    let dut = Actions::Repeat {
        times: 10,
        todo: vec![
            Actions::Delay {
                wait: Time::Uniform {
                    max: 1000,
                    min: 650,
                },
            },
            Actions::Click {
                button: Button::Left,
                delay: Some(Time::Constant(25)),
            },
        ],
    };

    assert_eq!(dut, action);
}
