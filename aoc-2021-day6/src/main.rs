fn main() {
    let mut fish_swarm: Vec<i32> = INPUT.split(",")
        .map(|x| { x.parse().unwrap() })
        .collect();

    for _ in 0..18 {
            fish_swarm.iter_mut()
            .for_each(|x| {
                *x = match *x {
                    -1 => 5,
                    _ => *x - 1
                }
            });
            for _ in 0..fish_swarm.iter().filter(|x| **x == -1)
            .count() {
                fish_swarm.push(8)
            }
            println!("{:?}",fish_swarm.iter.enumerate());
    }



}

fn offspring(clock: i32) -> u32 {
    1+2u32.pow(((17 - clock+1) / 6) as u32)
}


const INPUT:&str = "8";