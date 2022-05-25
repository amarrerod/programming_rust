mod player;
mod visible;
use player::Player;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use visible::Visible;
mod html;
use html::WriteHtml;
use std::iter;
use std::ops::{Add, Mul};
use std::vec::IntoIter;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn say_hello_generic<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
    let writer = File::create("example.txt")?;
    let mut serializer = serde_json::Serializer::new(writer);
    config.serialize(&mut serializer)?;
    Ok(())
}

fn cyclical_zip<T: Clone>(v: Vec<T>, u: Vec<T>) -> impl Iterator<Item = T> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn dot<N>(v1: &[N], v2: &[N]) -> Result<N, String>
where
    N: Add<Output = N> + Mul<Output = N> + Copy + Default,
{
    if v1.len() != v2.len() {
        return Err("v1 and v2 have different sizes".to_string());
    }

    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    Ok(total)
}

fn main() {
    use std::fs::File;
    let mut local_file = File::create("example.txt").expect("Error file not created");
    let mut buf: Vec<u8> = vec![];
    say_hello(&mut local_file);
    say_hello_generic(&mut local_file);
    say_hello_generic(&mut buf);
    println!("{:?}", buf);

    let mut player = Player::new(100, 80);
    player.draw();
    println!("{}", player.in_origin());

    let doc = html::HtmlDocument::new("test".to_string());
    local_file.write_html(&doc);

    let mut my_data: HashMap<String, String> = HashMap::new();
    my_data.insert("Name".to_string(), "Testing".to_string());
    save_configuration(&my_data);

    let v: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let u: Vec<i32> = vec![0, 10, 100, 1000, 10000, 100000];
    match dot(&v, &u) {
        Ok(x) => println!("{}", x),
        Err(err) => println!("Error {}", err),
    }
}
