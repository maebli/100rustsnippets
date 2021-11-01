use std::ops::Range;

fn main() {
    euler1();
    euler2();
    euler3();
    euler4();
}

fn euler1(){
    let x= (0..1000)
        .filter(|x| (x%3 == 0 )| (x%5 == 0))
        .sum::<u32>();

    println!("sum of all multiples {:?}",x);
}

fn euler2(){

    let mut sum = 2;
    let mut sum_temp ;
    let mut sum_prev = 1;
    let mut sum_even = 0;

    while sum < 4000000 {

        if sum % 2 == 0 {
            sum_even += sum;
        }

        sum_temp = sum;
        sum = sum_prev + sum;
        sum_prev = sum_temp;

    }

    println!("sum of all even fibonacci numbers up to {} is {:?}",40000000,sum_even);

}

fn euler3(){

    const NUM:u64 = 600851475143;
    let factors = trial_division(NUM);
    println!("biggest primal factor:{:?}",factors.last().unwrap());
}

fn trial_division(n:u64) -> Vec<u64>{
    let mut v = Vec::new();
    let mut n = n;
    let mut factor = 2;
    while n > 1 {
        if n % factor == 0 {
            n = n / factor as u64;
            v.push(factor);
        } else {
            factor += 1;
        }
    }
    v
}


fn euler4(){

    let mut largest_prod = 0;
    let mut prod ;

    const RANGE: Range<u64> = 100..1000;

    for x in RANGE {
        for y in RANGE {
            prod =x*y;

            if prod > largest_prod && is_palindrome(prod){
                largest_prod = prod;

            }
        }
    }

    println!("largest palindrome which is product of two three digit numbers = {}",largest_prod)
}

#[test]
fn test_is_palindrome(){
    assert!(!is_palindrome(99002275));
    assert!(is_palindrome(332233));
    assert!(is_palindrome(332233332233));
    assert!(is_palindrome(99999999));
    assert!(!is_palindrome(94));
    assert!(!is_palindrome(994));
    assert!(!is_palindrome(12345678));
    assert!(!is_palindrome(971198));
    assert!(!is_palindrome(963342));

}

fn is_palindrome(x:u64) -> bool{
    let mut s = x.to_string();

    while s.len() >= 2 {
        if s.ends_with(s.chars().nth(0).unwrap()) {
            if s.len() == 2 {
                return s.chars().nth(0) == s.chars().nth(1)
            }
            s.remove(0);
            s.remove(s.len()-1);
            println!("{}",s)
        }else{
            return false
        }

    }

    false
}