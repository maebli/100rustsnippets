//Problem 5
// [](https://projecteuler.net/problem=5)
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
fn main() {

    let x =(1..)
        .filter(|x| {(1..20).fold(0,|total,next|{total+x%next}) ==0} )
        .take(1)
        .last();
    println!("{}",x.unwrap());

}

