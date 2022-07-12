use std::sync::mpsc;
use std::{fs, io, thread};

fn start_file_reader_thread(
    documents: Vec<String>,
) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = mpsc::channel();
    let senders = vec![sender.clone(); 10];
    let handle = thread::spawn(move || {
        for filename in documents {
            for (idx, s) in senders.iter().enumerate() {
                let text = format!("Text: {} from sender: {}", filename.clone(), idx);

                if s.send(text).is_err() {
                    break;
                }
            }
        }
        Ok(())
    });

    (receiver, handle)
}

fn start_file_indexing_thread(
    texts: mpsc::Receiver<String>,
) -> (mpsc::Receiver<usize>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for (idx, text) in texts.into_iter().enumerate() {
            let r = sender.send(idx);
            if r.is_err() {
                break;
            } else {
                println!("Sent message: {} with content: {}", idx, text);
            }
        }
    });
    (receiver, handle)
}

pub fn run_pipeline() -> () {
    let documents = vec!["Hola".to_string(), "Adios".to_string()];
    let (texts, h1) = start_file_reader_thread(documents);
    let (idx, h2) = start_file_indexing_thread(texts);
    let r1 = h1.join().unwrap();
    h2.join().unwrap();
}
