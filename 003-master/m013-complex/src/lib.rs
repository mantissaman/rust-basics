use std::ops::Add;
use std::convert::From;
use std::fmt::{Formatter, Display, Result};

#[derive(Default, Debug,PartialEq, Copy, Clone)]
pub struct Complex<T>{
    pub re: T,
    pub im: T
}

impl<T> Complex<T>{
    pub fn new(re: T, im: T) -> Self {
        Complex {re, im}
    }
}

// Implementing Add for a generic type T, where T implements Add <T, Output=t>
// <T, Output = T> says that implementation must have the same inpuy and Output types
impl<T: Add<T, Output= T>> Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T> From<(T,T)> for Complex<T> {
    fn from(val: (T, T)) -> Self{
        Self {re : val.0, im: val.1}
    }
}

impl<T: Display> Display for Complex<T>{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;
    #[test]
    fn compex_basics() {
        let first = Complex::new(3, 5);
        let second: Complex<i32> = Complex::default();

        assert_eq!(first.re, 3);
        assert_eq!(first.im, 5);
        assert_eq!(second.re, second.im);
    }
    #[test]
    fn complex_addition(){
        let a = Complex::new(3, 5);
        let b = Complex::default();
        let res = a +b;
        assert_eq!(res, a);
    }

    #[test]
    fn complex_from(){
        let a = (2345, 456);
        let b = Complex::from(a);

        assert_eq!(b.re, 2345);
        assert_eq!(b.im, 456);
    }

    #[test]
    fn complex_display(){
        let a = Complex::from((2345,456));
        println!("{}", a);
    }
}
