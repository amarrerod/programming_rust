pub struct Sporangium;

use cells::{Cell, Gene};

pub struct Spore {
    size: f64,
}

pub fn produce_spore(factory: &mut i32) -> Spore {
    Spore { size: 1.0 }
}

pub(crate) fn genes(spore: &Spore) -> Vec<Gene> {
    todo!()
}

fn recombine(parent: &mut Cell) {
    todo!()
}

mod cells {
    pub struct Cell {
        x: f64,
        y: f64,
    }
    impl Cell {
        pub fn distance_from_origin(&self) -> f64 {
            f64::hypot(self.x, self.y)
        }
    }

    pub struct Gene;
}
