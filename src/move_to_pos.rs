use std::error::Error;

use rand::{thread_rng,distributions::uniform::Uniform,Rng};
use autopilot::{
    geometry::{Point,Rect},
    screen::{is_point_visible,is_rect_visible},
    mouse::{location},
};

pub fn move_to_pos(target: &Point, time_slice: usize, speed: usize) -> Result<(),Box<dyn Error>> {

    if !is_point_visible(*target) {
        panic!("impossible target location");
    }

    loop {

        let loc = get_location()?;
        if loc.x == target.x && loc.y == target.y {
            return Ok(());
        }

        let retc = if loc.x >= target.x && loc.y >= target.y {
            Rect::new(*target, Size::new(loc.x-target.x,loc.y-target.y))
        } else if loc.x >= target.x && loc.y <= target.y {
            Rect::new(Point::{ x: target.x, y: loc.y }, Size::new(loc.x-target.x,target.y-loc.y))
        } else if loc.x <= target.x && loc.y >= target.y {
            Rect::new(Point::{ x: loc.x, y: target.y }, Size::new(target.x-loc.x,, loc.y - target.y))
        } else if loc.x <= target.x && loc.y <= target.y {
            Rect::new(loc.clone(), Size::new(target.x-loc.x, target.y-loc.y))
        };

        let rand_x = thread_rng().sample(Uniform::new(retc.point.x, retc.max_x()));
        let rand_y = thread_rng().sample(Uniform::new(retc.point.y, retc.max_y())));
    }
}
