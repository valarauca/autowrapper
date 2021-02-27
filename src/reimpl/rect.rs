use std::error::Error;

use rand::{distributions::uniform::Uniform, thread_rng, Rng};

use crate::reimpl::point::Point;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Rect {
    pub point: Point,
    pub x_width: isize,
    pub y_height: isize,
}

impl Rect {
    pub fn into_autopilot_rect(&self) -> autopilot::geometry::Rect {
        autopilot::geometry::Rect::new(
            self.point.into_autopilot_point(),
            autopilot::geometry::Size::new(self.x_width as f64, self.y_height as f64),
        )
    }

    pub fn max_x(&self) -> isize {
        self.point.get_x() + self.x_width
    }

    pub fn max_y(&self) -> isize {
        self.point.get_y() + self.y_height
    }

    pub fn random_point_inside(&self) -> Result<Point, Box<dyn Error>> {
        Point::new(
            thread_rng().sample(Uniform::new(self.point.get_x(), self.max_x())),
            thread_rng().sample(Uniform::new(self.point.get_y(), self.max_y())),
        )
    }
}
