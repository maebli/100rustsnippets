use std::ops;
use std::fmt;


struct ComplexNumber {
    re: i32,
    im: i32
}

fn main() {
    let a = ComplexNumber { re: 1, im: 2 };
    let b = ComplexNumber { re: 3, im: 4 };

    println!("({})+({}) = {}",a,b,&a+&b);
    println!("({})-({}) = {}",a,b,&a-&b);
    println!("({})*({}) = {}",a,b,&a*&b);
}

impl ops::Add<& ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, b: &ComplexNumber) -> ComplexNumber {
        let mut x:ComplexNumber = ComplexNumber { re: 0, im: 0 };
        x.im = &self.im + &b.im;
        x.re = &self.re + &b.im;
        x
    }
}

impl ops::Sub<& ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, b: &ComplexNumber) -> ComplexNumber {
        let mut x:ComplexNumber = ComplexNumber { re: 0, im: 0 };
        x.im = &self.im - &b.im;
        x.re = &self.re - &b.im;
        x
    }
}

impl ops::Mul<& ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, b: &ComplexNumber) -> ComplexNumber {
        let mut x:ComplexNumber = ComplexNumber { re: 0, im: 0 };
        x.im = &self.im * &b.re + &self.re * &b.im;
        x.re = &self.re * &b.re - &self.im * &b.im;
        x
    }
}

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i", self.re,self.im)
    }
}