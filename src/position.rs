use autopilot::geometry::Point as AutoPilotPoint;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use rand::{distributions::uniform::Uniform, thread_rng, Rng};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Position {
    Point {
        x: usize,
        y: usize,
    },
    Rectangle {
        top_left_x: usize,
        top_left_y: usize,
        bottom_right_x: usize,
        bottom_right_y: usize,
    },
}

impl Position {
    /// returns the point
    #[allow(dead_code)]
    pub(in crate) fn get_point(&self) -> AutoPilotPoint {
        match self.clone() {
            Position::Point { x, y } => AutoPilotPoint {
                x: x as f64,
                y: y as f64,
            },
            Position::Rectangle {
                top_left_x,
                top_left_y,
                bottom_right_x,
                bottom_right_y,
            } => {
                let rng_x = thread_rng().sample(Uniform::new(top_left_x, bottom_right_x)) as usize;
                let rng_y = thread_rng().sample(Uniform::new(top_left_y, bottom_right_y)) as usize;
                AutoPilotPoint {
                    x: rng_x as f64,
                    y: rng_y as f64,
                }
            }
        }
    }
}

#[test]
fn test_deserialize_point() {
    use serde_json::from_str;

    let text: &'static str = r#"{ "x": 15, "y": 16 }"#;
    assert_eq!(
        from_str::<Position>(&text).unwrap(),
        Position::Point { x: 15, y: 16 }
    );
}

#[test]
fn test_deserialize_rectange() {
    use serde_json::from_str;

    let text: &'static str =
        r#"{ "top_left_x": 100, "top_left_y": 200, "bottom_right_x": 200, "bottom_right_y": 300 }"#;
    assert_eq!(
        from_str::<Position>(&text).unwrap(),
        Position::Rectangle {
            top_left_x: 100,
            top_left_y: 200,
            bottom_right_x: 200,
            bottom_right_y: 300
        }
    );
}
