use std::cmp::Ordering;
use std::time::Instant;

fn main() {
    let num = 150;
    let mut before = Instant::now();
    println!("Nth fib = {}",nth_fib_recursive(num));
    let mut after = Instant::now();
    println!("{:?}", after.checked_duration_since(before));

    before = Instant::now();
    println!("Nth fib recursive = {}",nth_fib(num));
    after = Instant::now();
    println!("{:?}", after.checked_duration_since(before));

}

fn nth_fib_recursive(n:u128) ->  u128 {
    let x:u128 = 2;
    match n.cmp(& x){
        Ordering::Less => 1,
        Ordering::Equal => 2,
        Ordering::Greater => nth_fib(n-1)+nth_fib(n-2),
    }
}

fn nth_fib(n:u128) -> u128 {
    if n == 1 {
        1
    }else if n==2 {
        2
    }else {
        let mut xn_1:u128 = 2;
        let mut xn_2:u128 = 1;
        let mut temp:u128;
        for _num in 2..n {
            temp = xn_1;
            xn_1 = xn_1 + xn_2;
            xn_2 = temp;
        }
        xn_1
    }
}
