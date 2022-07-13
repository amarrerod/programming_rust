mod complex;
mod interval;

use interval::Interval;
fn main() {
    let mut c1 = complex::Complex::new(32, 100);
    let mut c2 = complex::Complex::new(33, 101);
    let mut c3 = c1 + c2;
    c3 = -c3;
    println!("{:?}", c3);

    let c4 = complex::Complex::new(32, 50);
    let mut c5 = complex::Complex::new(10, 80);

    assert_eq!(c5 + c4, complex::Complex::new(42, 130));
}
