use crate::shape;

pub fn number_of_rabbits(r: u32) {
    match r {
        0 => {}
        1 => println!("There's one rabbit"),
        n => println!("There are {} rabbits", n),
    }
}

#[derive(Debug)]
pub enum Calendar {
    Gregorian,
    Chinese,
    Korea,
    Ethiopian,
}

pub fn calendar_type(calendar: &str) -> Option<Calendar> {
    let new_calendar = match calendar {
        "gregorian" => Calendar::Gregorian,
        "chinese" => Calendar::Chinese,
        "ethiopian" => Calendar::Ethiopian,
        other => return None,
    };
    Some(new_calendar)
}

pub fn describe_point(x: i32, y: i32) -> String {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "At the origin".to_string(),
        (_, Equal) => "On the x axis".to_string(),
        (Equal, _) => "On the y axis".to_string(),
        (Greater, Greater) => "In the first quadrant".to_string(),
        (Less, Greater) => "In the second quadrant".to_string(),
        _ => "Who cares".to_string(),
    }
}

pub fn balloon_location(location: shape::Point3d) {
    match location {
        shape::Point3d { x, y, .. } => {
            println!("Balloon in 2D: {:?}", location);
        }
        shape::Point3d { x, y, z } => {
            println!("Balloon at this point: {:?}", location);
        }
    }
}

pub fn hsl_2_rgb(hsl: [u8; 3]) -> [u8; 3] {
    match hsl {
        [_, _, 0] => [0, 0, 0],
        [_, _, 255] => [255, 255, 255],
        [h, s, l] => [h, s, l],
    }
}

pub fn greet_peopple(names: &[&str]) {
    match names {
        [] => println!("Hello world"),
        [a] => println!("Hello, {}", a),
        [a, b] => println!("Hello, {} and {}.", a, b),
        [a, .., b] => println!("Hello, everyone from {} to {}", a, b),
    }
}

pub fn get_center(sphere: &shape::Point3d) {
    match sphere.center() {
        &shape::Point3d { x, y, z: 0.0 } => {
            println!("This is the center: {}, {}", x, y);
        }

        &shape::Point3d { x, y, z } => {
            println!("This is the center: {}, {}, {}", x, y, z);
        }
    }
}
