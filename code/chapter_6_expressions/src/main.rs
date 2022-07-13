use rand::{distributions::Uniform, Rng};
mod quicksort;

fn error_messages() -> Result<Vec<String>, ()> {
    Ok(vec!["error 1".to_string(), "error 2".to_string()])
}

fn main() {
    println!("Hello, Rust Chapter 6!");

    let code: u32 = 0;
    match code {
        0 => println!("OK"),
        1 => println!("Wires tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", code),
    }

    for rs in &error_messages().unwrap() {
        println!("Error is: {}", rs);
    }

    for rs in &mut error_messages().unwrap() {
        rs.push('#');
        println!("After modfiying it is: {}", rs);
    }
    let mut integers: Vec<i32> = rand::thread_rng()
        .sample_iter(Uniform::from(0..101))
        .take(10)
        .collect();
    println!("Initial vector is: {:?}", integers);
    quicksort::quicksort(&mut integers);
    println!("Sorted vector is: {:?}", integers);
}
