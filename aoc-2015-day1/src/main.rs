fn main() {
    let ans1: i32 = INPUT
        .chars()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();

    println!("{}", ans1);

    let ans2 = INPUT
        .chars()
        .enumerate()
        .fold((-1i32, 0i32), |mut acc, inst| {
            match inst.1 {
                '(' => acc.1 = acc.1 as i32 + 1,
                ')' => acc.1 = acc.1 as i32 - 1,
                _ => {}
            }
            if acc.1 < 0 && acc.0 == -1 {
                acc.0 = inst.0 as i32 + 1
            }
            acc
        })
        .0;

    println!("{}", ans2);
}

const INPUT:&str=
"()(((()))(())
";
