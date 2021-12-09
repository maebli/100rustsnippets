fn main() {

    const N:usize = 10;
    let mut tunnel = INPUT.lines()
        .map(|x|{x.as_bytes()})
        .fold(vec![b'9';N+2],|mut acc:Vec<u8>, inst|{
            acc.push(b'9');
            acc.extend(inst.to_vec());
            acc.push(b'9');
            acc
        });
    tunnel.extend(vec![b'9';(N+2)*2]);

    let x:Vec<(bool,u8)> = tunnel
        .windows((N+2)*3).collect::<Vec<&[u8]>>()
        .iter()
        .enumerate()
        .map(|x|{(
                    x.1[1] > x.1[N+3]
                    && x.1[(N+2)*2+1] > x.1[N+3]
                    && x.1[N+2] > x.1[N+3]
                    && x.1[N+4] > x.1[N+3]
                , x.1[N+3]- 48)
            })
        .collect();

    let sol1:u32=x.iter()
        .filter(|x|x.0)
        .map(|x|x.1)
        .fold(0,|acc,inst|{acc+inst as u32 +1});

    println!("{:?}",sol1);

}


const INPUT:&str = "2199943210
3987894921
9856789892
8767896789
9899965678";