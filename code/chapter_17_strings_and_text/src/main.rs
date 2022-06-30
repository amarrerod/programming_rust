use std::fmt;

struct Complex {
    pub re: f64,
    pub im: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let (re, im) = (self.re, self.im);
        if dest.alternate() {
            let abs = f64::sqrt(re * re + im * im);
            let angle = f64::atan2(im, re) / std::f64::consts::PI * 100.0;
            write!(dest, "{} < {}º", abs, angle)
        } else {
            let im_sign = if self.im < 0.0 { '-' } else { '+' };
            write!(dest, "{} {} {}i", self.re, im_sign, f64::abs(self.im))
        }
    }
}

fn main() {
    let c = Complex { re: 0.5, im: 0.866 };
    println!(
        "{:.3}us: relocated {} at {:#x} to {:#x}, {} bytes",
        0.84391, "object", 140737488346304_usize, 6299664_usize, 64
    );
    println!("The complex is: {}", c);
    println!("The complex is: {:#}", c);
}
