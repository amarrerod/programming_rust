// Declaramos los módulos

pub mod leaves;
pub mod roots;
pub mod stems;

pub use self::leaves::Leaf;
pub use self::roots::Root;

use self::roots::RootSet;
use stems::StemSet;

pub enum FernType {
    Fiddlehead,
}

pub struct Fern {
    pub roots: RootSet,
    pub stems: StemSet,
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    pub fn new(_type: FernType) -> Fern {
        Fern {
            roots: vec![],
            stems: vec![stems::Stem { furled: true }],
            size: 1.0,
            growth_rate: 0.001,
        }
    }

    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }

    pub fn is_furled(&self) -> bool {
        !self.is_fully_unfurled()
    }

    pub fn is_fully_unfurled(&self) -> bool {
        self.stems.iter().all(|s| !s.furled)
    }
}
