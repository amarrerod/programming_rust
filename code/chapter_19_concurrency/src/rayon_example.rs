use crate::fork_join::Data;
use rayon::prelude::*;
use std::sync::Arc;

pub fn do_n_thing(data: &[u8]) {
    data.par_iter().for_each(|value| {
        println!("{}", value);
    });
}

fn process_files(files: &u8, data: &Data) -> std::io::Result<()> {
    println!(
        "Hello from a child thread with numbers: {:?} and data: {:?}",
        files, data
    );
    Ok(())
}

pub fn process_files_in_parallel(filenames: &Vec<u8>, data: &Data) -> std::io::Result<()> {
    filenames
        .par_iter()
        .map(|filename| process_files(filename, data))
        .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
        .unwrap_or(Ok(()))
}
