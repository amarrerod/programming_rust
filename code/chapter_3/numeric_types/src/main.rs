mod aliases_and_strs;
mod floating;
mod overflow;
mod sequences;
mod tuples;

fn main() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);

    //overflow::overflow_example();
    overflow::checking_example();
    overflow::wrapping_example();

    floating::limits();

    tuples::tuples_example();

    sequences::arrays();
    sequences::vectors();
    aliases_and_strs::decode(&aliases_and_strs::Bytes::new());
    aliases_and_strs::print_str("Hola Mundo, Rust!");
}
