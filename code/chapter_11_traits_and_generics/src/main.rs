use std::io::Write;
mod player;
mod visible;
use player::Player;
use visible::Visible;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn say_hello_generic<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
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
}
