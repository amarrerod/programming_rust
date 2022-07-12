use std::sync::Arc;
use std::{io, thread};

#[derive(Default, Debug)]
pub struct Data {
    name: String,
    idx: usize,
    info: Vec<u32>,
}

impl Data {
    pub fn new(name: String, idx: usize) -> Data {
        Data {
            name,
            idx,
            ..Default::default()
        }
    }
}

fn process_files(files: &Vec<u8>, data: &Data) -> io::Result<()> {
    println!(
        "Hello from a child thread with numbers: {:?} and data: {:?}",
        files, data
    );
    Ok(())
}

fn split_vector_in_chunks(data: &Vec<u8>, n: usize) -> Vec<Vec<u8>> {
    data.chunks(n).map(|s| s.into()).collect()
}

pub fn process_files_in_parallel(filenames: &Vec<u8>, data: Arc<Data>) -> io::Result<()> {
    const NTHREADS: usize = 8;
    let mut thread_handles = vec![];
    let worklists = split_vector_in_chunks(filenames, NTHREADS);
    for worklist in worklists {
        let data_for_child_thread = data.clone();
        thread_handles.push(thread::spawn(move || {
            process_files(&worklist, &data_for_child_thread)
        }));
    }

    for handle in thread_handles {
        handle.join().unwrap()?;
    }
    Ok(())
}
