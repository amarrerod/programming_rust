#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3d {
    pub fn new(x: f32, y: f32, z: f32) -> Point3d {
        Point3d { x, y, z }
    }

    pub fn center(&self) -> &Point3d {
        &self
    }
}

pub fn print_shape(p: &Point3d) {
    println!("{:?}", p);
}

pub const ORIGIN: Point3d = Point3d {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

impl Shape {
    pub fn create_unit_sphere() -> Shape {
        Shape::Sphere {
            center: ORIGIN,
            radius: 1.0,
        }
    }
}
