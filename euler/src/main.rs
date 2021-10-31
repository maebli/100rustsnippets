fn main() {
    euler1();
    euler2();
    euler3();
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