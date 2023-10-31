use std::fmt::{Display, Formatter};
use std::ops;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Complex {
    re: f64,
    im: f64
}


impl Complex {
    // Static functions
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    pub fn uniform(x: f64) -> Self {
        Complex { re: x, im: x }
    }

    pub fn zero() -> Self {
        Complex { re: 0.0, im: 0.0 }
    }
}


// Operators
impl ops::Add<Complex> for Complex {
    type Output = Complex;
    
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}


impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}


impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}


impl Display for Complex {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("{} + {}i", self.re, self.im);
        Ok(())
    }
}