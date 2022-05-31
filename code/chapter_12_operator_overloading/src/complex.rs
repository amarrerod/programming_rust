use std::cmp::PartialEq;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::Not;

#[derive(Debug)]
pub struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex { re, im }
    }
}

impl<L, R> Add<Complex<R>> for Complex<L>
where
    L: Add<R>,
{
    type Output = Complex<L::Output>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> Not for Complex<T>
where
    T: Not<Output = T> + Neg<Output = T>,
{
    type Output = Complex<T>;
    fn not(self) -> Complex<T> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, other: Complex<T>) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}
