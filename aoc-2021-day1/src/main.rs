use itertools::multizip;

fn main() {

    let a= INPUT.lines()
        .map(|x|{x.parse().unwrap()})
        .collect::<Vec<u32>>();

    let c:Vec<u32> = multizip((a.iter(),a.iter().skip(1),a.iter().skip(2)))
        .map(|x|{x.0+x.1+x.2})
        .collect();

    println!("sol1:{}, sol2:{}",count_bigger(&a),count_bigger(&c));

}

fn count_bigger(a: &Vec<u32>) -> usize {
    a.iter()
        .zip(a.iter().skip(1))
        .filter(|x| { x.0 < x.1 })
        .count()
}


const INPUT:&str = "199
200
208
210
200
207
240
269
260
263";