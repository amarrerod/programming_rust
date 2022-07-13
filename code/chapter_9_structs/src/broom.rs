#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

pub struct Broom {
    pub name: String,
    pub height: u32,
    pub health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

impl Broom {
    pub fn new(name: String) -> Broom {
        Broom {
            name: name,
            height: 180,
            health: 100,
            position: (0.0, 0.0, 0.0),
            intent: BroomIntent::FetchWater,
        }
    }
}

pub fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}
