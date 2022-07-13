use crate::plant_structures::{Fern, FernType};
use std::fs::File;
use std::time::Duration;

pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}

pub struct Terrarium {
    ferns: Vec<Fern>,
}

impl Terrarium {
    pub fn new() -> Terrarium {
        Terrarium { ferns: vec![] }
    }

    pub fn load(filename: &str) -> Terrarium {
        File::open(filename).unwrap();
        Terrarium {
            ferns: vec![Fern::new(FernType::Fiddlehead)],
        }
    }

    pub fn fern(&self, index: usize) -> &Fern {
        &self.ferns[index]
    }
    /// Let the sun shine in and run the simulation for a given amount of time.
    ///
    ///     # use fern_sim::Terrarium;
    ///     # use std::time::Duration;
    ///     # let mut tm = Terrarium::new();
    ///     tm.apply_sunlight(Duration::from_secs(60));
    ///
    pub fn apply_sunlight(&mut self, time: Duration) {
        for f in &mut self.ferns {
            for s in &mut f.stems {
                s.furled = false;
            }
        }
    }
}
