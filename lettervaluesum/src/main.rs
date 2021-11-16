fn main() {

    let f= |k:&str| k.bytes().fold(0,|acc,x|acc + ((x - b'a' + 1) as u32));
    println!("{}",f("microspectrophotometries"));

}
