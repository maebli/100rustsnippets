fn main() {

    let mut v =INPUT.split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    v.sort();

    let c = v.len();

    let median = match c%2 {
        1 => *v.get(c/2).unwrap() as f32,
        _ => ((v.get(c/2-1).unwrap()+v.get(c/2).unwrap()) as f32).abs() / 2f32
    };

    println!("sol1:{}",v.iter().fold(0,|acc,inst|{acc+(*inst-(median as i32)).abs()}));

    let x = (0..*(v.get(v.len()-1).unwrap()))
        .fold(i32::MAX,|acc, k|
            {(v.iter()
                .fold(0,|acc,inst|acc+find_fuel2((*inst-k).abs()))
            ).min(acc)
            }
        );

    println!("sol2:{}",x);
}

fn find_fuel2(x: i32) -> i32{
     x*(x+1)/2
}


const INPUT:&str = "16,1,2,0,4,2,7,1,2,14";

