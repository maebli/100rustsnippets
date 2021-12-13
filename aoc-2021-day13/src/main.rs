use std::cmp::Ordering;

fn main() {

    let points:Vec<(u32,u32)> = DOTS.lines()
        .map(|x|{
            let k = x.split_once(',').unwrap();
            (k.0.parse().unwrap(), k.1.parse().unwrap())
        }).collect();

    let n = points
        .iter()
        .fold(0,|max,inst| inst.0.max(max).max(inst.1))+1;

    let folds:Vec<(Fold,u32)>= FOLDS.lines()
        .map(|x|{
            let k = x.split_once("=").unwrap();
            match k.0 {
                "fold along y" => (Fold::Up, k.1.parse::<u32>().unwrap()),
                _ => (Fold::Left,k.1.parse().unwrap())
            }
        }).collect();

    let mut i: Vec<u32> = points
        .iter()
        .map(|x| x.0 + (n) * x.1)
        .collect();
    i.sort();

    i = match folds.get(0).unwrap() {
        (Fold::Up, y) => {
            points
                .iter()
                .map(|x| x.0 + (n) * x.1)
                .map(|i| match i.cmp(&((n * n) / 2)) {
                    Ordering::Greater => n * (n - (i - 1) / n) - (n - i % n),
                    _ => i
                }).collect()
        },
        (Fold::Left, x) => {
            points.iter()
                .filter(|i| i.0 != n/2)
                .map(|i| i.0 + n*i.1)
                .map(|i| match (i % n).cmp(&(n / 2)) {
                    Ordering::Greater => (n-i%n-1) + (i / n) * n,
                    _ => i
                }).collect()
        },
        _ => vec![0]
    };

    i.sort();
    i.dedup();

    println!("{:?}",i.len());
}

#[derive(Debug)]
enum Fold{
    Up,
    Left
}


const DOTS:&str = "0,0
2,0
3,0
6,0
9,0
1,1
4,1
6,2
10,2
0,3
4,3
1,4
3,4
6,4
8,4
9,4
10,4
";

const FOLDS:&str ="fold along x=5";