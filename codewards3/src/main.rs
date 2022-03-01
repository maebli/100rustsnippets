
use num::bigint::{BigInt, ToBigInt};
use num::traits::{One, Zero};


fn main(){

}

fn last_digit(lst: &[u64]) -> u64 {
    if lst.len() < 3 {
        return 1;
    }else if lst[0] == lst[1] && lst[1] == lst[2] {
       return 0;
    }
    let n = 10;

    let a = (lst[0] % n).to_bigint().unwrap();
    let b = lst[1].to_bigint().unwrap();
    let c = lst[2].to_bigint().unwrap();

    let d = modpow(&a, &b, &n);

    *modpow(&b,&d,&n).to_signed_bytes_le().first().unwrap() as u64
}

pub fn modpow<T: ToBigInt, U: ToBigInt>(base: &T, exp: &T, n: &U) -> BigInt {
    let (mut base, mut exp, n) = (
        base.to_bigint().unwrap(),
        exp.to_bigint().unwrap(),
        n.to_bigint().unwrap(),
    );

    assert!(
        exp >= Zero::zero(),
        "negative exponent cannot be used in modular exponentiation"
    );

    if exp == Zero::zero() {
        return One::one();
    }

    let mut res: BigInt = One::one();
    base %= &n;

    loop {
        if &exp % 2 == One::one() {
            res *= &base;
            res %= &n;
        }

        if exp == One::one() {
            return res;
        }

        exp /= 2;
        base *= base.clone();
        base %= &n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        let tests = vec![
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6)
        ];

        for test in tests {
            assert_eq!(last_digit(&test.0), test.1);
        }
    }
}