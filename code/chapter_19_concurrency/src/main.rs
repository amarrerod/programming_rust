mod channels;
mod fork_join;
mod rayon_example;
mod mutex;

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
/*    println!("Hello, world!");
    let strings = vec!["Hello, world!".as_bytes()];
    let numbers = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 63, 64,
    ];
    let data: Arc<fork_join::Data> =
        Arc::new(fork_join::Data::new("Example of data".to_string(), 1));
    fork_join::process_files_in_parallel(&numbers, data.clone());
    rayon_example::do_n_thing(&numbers);
    rayon_example::process_files_in_parallel(&numbers, &data);
    channels::run_pipeline();*/

    let mut handlers = vec![];
    for i in 0..10 {
        handlers.push(
            thread::spawn(move || {
                let game = Arc::new(mutex::UltimateGame{
                    waiting_list: Mutex::new(vec![]),
                });
                for j in i..i*10{
                    game.join_waiting_list(j as u32);
                }
        })
        );
    }
    for handler in handlers {
        handler.join();
    }
}
