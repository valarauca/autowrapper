use std::error::Error;

/// Wraps point primative b/c autopilot uses floats.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Point {
    x: isize,
    y: isize,
}
impl Point {
    pub fn get_x(&self) -> isize {
        self.x
    }

    pub fn get_y(&self) -> isize {
        self.y
    }

    pub fn new(x: isize, y: isize) -> Result<Self, Box<dyn Error>> {
        Self::try_from_point(autopilot::geometry::Point::new(x as f64, y as f64))
    }

    pub fn get_mouse_location() -> Result<Point, Box<dyn Error>> {
        Self::try_from_point(autopilot::mouse::location())
    }

    pub fn into_autopilot_point(&self) -> autopilot::geometry::Point {
        autopilot::geometry::Point {
            x: self.x as f64,
            y: self.y as f64,
        }
    }

    pub fn try_from_point(point: autopilot::geometry::Point) -> Result<Point, Box<dyn Error>> {
        if !autopilot::screen::is_point_visible(point.clone()) {
            Err(build_err(format!(
                "Point: {:?} is not visible on main monitor",
                point.clone()
            )))
        } else {
            Ok(Point {
                x: point.x as isize,
                y: point.y as isize,
            })
        }
    }

    pub fn move_mouse(&self) -> Result<(), Box<dyn Error>> {
        Ok(autopilot::mouse::move_to(self.into_autopilot_point())?)
    }

    pub fn euclidian_dist(&self, other: &Point) -> usize {
        ((self.x as f64 - other.x as f64).powi(2) + (self.y as f64 - other.y as f64).powi(2)).sqrt()
            as usize
    }
}

fn build_err(msg: String) -> Box<dyn Error> {
    use std::io::ErrorKind;
    Box::new(std::io::Error::new(ErrorKind::Other, msg))
}
