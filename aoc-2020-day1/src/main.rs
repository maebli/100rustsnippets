
fn main() {
    let mut x:Vec<u32> = INPUT.split('\n')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    x.sort();

    while x.len()>1 {
        let mut next = x.pop().unwrap();
        let mut i = x.len() - 1;
        let mut current: u32 = 2020;
        while next + current >= 2020 {

            current = *x.get(i).unwrap();

            if next + current == 2020 {
                println!("{:?}*{:?}={:?}", next, current, next * current);
            }
            if i == 0 {
                break;
            }

            i = i - 1;
        }
    }


}

const INPUT:&str="1721
979
366
299
675
1456
";
