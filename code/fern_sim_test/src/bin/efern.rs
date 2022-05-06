use fern_sim_test::{run_simulation, Fern};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
        roots: Vec::new(),
        stems: Vec::new(),
    };
    run_simulation(&mut fern, 1000);
    println!("Simulation done with final fern size: {}", fern.size);
}
