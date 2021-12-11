fn main() {

    const MAX_ITERATIONS:i32 = 10000;

    let mut energy:Vec<i32> = INPUT.lines()
        .map(|x|{x.as_bytes()})
        .fold(vec![0u8;0],|mut acc:Vec<u8>, inst|{
            acc.extend(inst.to_vec());
            acc
        })
        .iter()
        .map(|x|{(x - b'0') as i32})
        .collect();

    let len = energy.len() as i32;
    let n = (len as f64).sqrt() as i32;
    let coordinates:Vec<(i32,i32)>  = (0..len).map(|x|{((x%n),(x/n) )}).collect();

    let mut flashes = 0;

    for i in 0..MAX_ITERATIONS {

        /* step one: increment */
        energy
            .iter_mut()
            .for_each(|x|*x=*x+1);

        /* step two : increment neighbours*/
        while energy.contains(&10){

            let mut neighbours = Vec::new();

            energy
                .iter_mut()
                .enumerate()
                .map(|x|{(coordinates.get(x.0).unwrap(),x.1)})
                .filter(|x|*x.1 == 10)
                .for_each(|x|{
                    *x.1 = *x.1+1;
                    neighbours.push((( x.0.0 - 1 ) , ( x.0.1 - 1) ));
                    neighbours.push((( x.0.0     ) , ( x.0.1 - 1) ));
                    neighbours.push((( x.0.0 + 1 ) , ( x.0.1 - 1) ));
                    neighbours.push((( x.0.0 - 1 ) , ( x.0.1    ) ));
                    neighbours.push((( x.0.0 + 1 ) , ( x.0.1    ) ));
                    neighbours.push((( x.0.0 - 1 ) , ( x.0.1 + 1) ));
                    neighbours.push((( x.0.0     ) , ( x.0.1 + 1) ));
                    neighbours.push((( x.0.0 + 1 ) , ( x.0.1 + 1) ));
                });

            energy
                .iter_mut()
                .zip(coordinates.iter())
                .for_each(|x|{
                    let count = neighbours.iter()
                        .filter(|n|n.0 == (x.1.0) && n.1 == (x.1.1))
                        .count();
                    /* Th next two lines are essential and caused me a lot of trouble */
                    if *x.0 < 10 {
                        *x.0 = (*x.0 + (count as i32)).min(10);
                    }

                });

        }

        /* reset: to zero*/
         energy
            .iter_mut()
            .filter(|x|**x > 9)
            .for_each(|x|{*x=0; flashes+=1});

        if i == 100 {
            display_energy(&energy);
            println!("sol1={}",flashes);
        }

        if energy.iter().filter(|x|**x == 0).count() == len as usize {
            display_energy(&energy);
            println!("sol2={}",i+1);
            break;
        }



    }


    
}


fn display_energy(energy: &Vec<i32>){
    println!("current energy level and glowing octopuses:");
    let n = (energy.len() as f64).sqrt() as i32;
    for i in 0..n {
        for j in 0..n {
            match energy[(i*n+j) as usize] {
                0 => print!("*"),
                n => print!("{}",n),
            }
        }
        println!();
    }
    println!();
    println!();
}


const INPUT:&str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";