use primes::{Sieve, PrimeSet};

fn main() {

    let mut pset = Sieve::new();
    println!("Prime {}", pset.get(10000));

}
