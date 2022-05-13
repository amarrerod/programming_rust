#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

const ORIGIN: Point3d = Point3d {
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
