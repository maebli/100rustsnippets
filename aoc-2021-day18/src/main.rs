use crate::SnailFishNumber::{Pair, Regular};
use std::{ops};
use std::borrow::Borrow;

fn main() {

    let a = Pair(Box::new(Regular(1)), Box::new(Regular(2)));
    let b = Pair(Box::new(Regular(1)), Box::new(Regular(2)));
     println!("{:?}",a+b);
}


#[test]
fn test_parse(){
    let input:&str = "[[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4]";
    let expected_output =
       Pair(
            Box::new( Pair(
                Box::new(Pair(
                    Box::new(Pair(
                        Box::new(Pair(
                            Box::new(Regular(9)),
                            Box::new(Regular(8)))),
                        Box::new(Regular(1)))),
                    Box::new(Regular(2)))),
            Box::new(Regular(3)), )),
            Box::new(Regular(4)));

    assert_eq!(expected_output,SnailFishNumber::from("[[[[[9,8],1],2],3],4]"));
}

#[test]
fn test_explode(){
    let mut x = Box::new(SnailFishNumber::from("[[[[[9,8],1],2],3],4]"));
    x.explode_once();
    println!("{:?}",SnailFishNumber::from("[[[[0,9],2],3],4]"));
    println!("{:?}",x);
}

#[derive(Clone,Debug,PartialEq)]
enum SnailFishNumber{
    Pair(Box<SnailFishNumber>,Box<SnailFishNumber>),
    Regular(i32)
}

impl SnailFishNumber{

    fn from(s:& str) -> SnailFishNumber{

        let mut comma_index = 0;
        let mut bracket_count = 1;

        while bracket_count > 0 {
            comma_index+=1;
            match s.as_bytes().iter().nth(comma_index).unwrap() {
                b'[' => bracket_count+=1,
                b']' => bracket_count-=1,
                _ => ()
            }
        }

        comma_index-=1;

        let left = &s[1..comma_index-1];
        let right = &s[comma_index..s.len()-1];

        Pair(
            if left.contains(",") {
                Box::new(SnailFishNumber::from(left))
            } else {
                Box::new( Regular(left.parse().unwrap()))
            }
            ,
            if right.contains(",") {
                Box::new(SnailFishNumber::from(right))
            } else {
                Box::new( Regular(right.parse().unwrap()))
            }
        )

    }

    fn explode_once(&mut self){

        let mut num = self;
        let mut depth = 1;
        let mut last_left = None;
        let mut last_right = None;
        loop {
            if let Pair(x,y) = num {
                if let Regular( k) = **x {
                    if let Some(z) = last_left {
                        *x = Box::new(Regular(k+z));
                    }else{
                        *x = Box::new(Regular(0));
                    }
                    if let Regular (k) = **y {
                        return;
                    }else{
                        last_right = Some(*y);
                        y.explode_once();
                    }
                    last_left = Some(k);
                }
                println!("{:?}, {:?}",depth,*x);
                num = x;
                depth += 1;
            }
        }

    }
}

impl ops::Add<SnailFishNumber> for SnailFishNumber {
    type Output = SnailFishNumber;

    fn add(self, b: SnailFishNumber) -> SnailFishNumber {
        Pair(Box::new(self), Box::from(b))
    }
}