fn main() {

    let mut input: Vec<Vec<u32>> = DIAGNOSTIC_REPORT.lines()
        .map(|l| {
            l.as_bytes()
                .into_iter()
                .map(|x| (*x == b'1') as u32)
                .collect()
        })
        .collect();

    let mut a:usize = 0;

    let mut x = input.clone();

    while (*x).len()>1 {
        let bit_count = find_most_common_bits_per_row(x.clone());
        x.retain(|x|{x[a] == bit_count[a]});
        a+= 1;
    }

    println!("{:?}",x);

    let mut a:usize = 0;
    while (*input).len()>1 {
        let bit_count = find_most_common_bits_per_row(input.clone());
        input.retain(|x|{x[a] != bit_count[a]});
        a+= 1;
    }

    println!("{:?}",input);

}


fn find_most_common_bits_per_row(x: Vec<Vec<u32>>) -> Vec<u32> {
    let length = x.len() as u32;
    let x = transpose(x);
    let bit_count: Vec<u32> = x.iter()
        .map(|x| {
            x.iter().fold(0, |acc, inst| { acc + inst })
        })
        .map(|x| { (x as f32 >= (length as f32 / 2.0)) as u32 })
        .collect();
    bit_count
}

/// Source : https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}


const DIAGNOSTIC_REPORT:&str ="00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";