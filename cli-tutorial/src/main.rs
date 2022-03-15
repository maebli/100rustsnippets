use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("This program takes exactly one argument, the file name.")
    }

    let filname = &args[1];
    let contents = fs::read_to_string(filname)
        .expect("Something went wrong reading the file");

    print!("{}", contents);
}
