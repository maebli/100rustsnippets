fn main() {

    let a = (1..101).fold(0,|acc,x|{acc+x*x});
    let b = (1..101).fold(0,|acc,x|{acc+x});

    let c = b*b-a ;

    println!("{}",c);
}
