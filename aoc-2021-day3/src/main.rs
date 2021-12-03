fn main() {

    let x:Vec<Vec<_>> = DIAGNOSTIC_REPORT.lines()
        .map(|l| {
            l.as_bytes()
                .into_iter()
                .map(|x|*x == b'1')
                .collect()
        })
        .collect();

    let length = x.len() as u32;

    let bit_count = x
        .iter()
        .fold(vec![0; length as usize], |acc:Vec<u32>, inst|{
        acc
            .iter().zip(inst.iter())
            .map(|x| {match x.1
                {
                    true =>{ *x.0 + 1},
                    false => *x.0
            }})
            .collect()
    });

    println!("{:?}",bit_count);

    let gamma= bit_count.iter()
        .map(|x|{ (*x > length/2) as u32})
        .rev()
        .enumerate()
        .fold(0,|acc, inst|{
            acc | (inst.1 << (inst.0 as u32))
            }
        );

    let epsilon = (!gamma) & 0b111111111111;
    println!("{}*{} = {}",gamma,epsilon,gamma*epsilon);

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