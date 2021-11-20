fn main() {
    let answer = INPUT.lines()
        .filter(|&x| check_line(x))
        .count();

    println!("{}",answer);
}

fn check_line(line:&str) -> bool {

    let mut s:Vec<&str> = line
        .split(' ')
        .collect();

    let range = s
        .remove(0)
        .split_once('-')
        .unwrap();

    let min:usize = range
        .0
        .parse()
        .unwrap();

    let max:usize = range
        .1
        .parse()
        .unwrap();

    let letter = s
        .remove(0)
        .trim_end_matches(":")
        .chars()
        .nth(0)
        .unwrap();

    let password = s.remove(0);

    let num = password
        .chars()
        .filter(|c| c.eq(&letter))
        .count();


    num >= min && num <= max

}

const INPUT:&str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";