extern crate core;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if let Some(input)=args.get(1){
        if input.len() !=22 {
            panic!("Wrong input length, 22 expected, {} provided",input.len());
        }
        let mut meter_volume = u32::from_str_radix(&input[0..2], 16).unwrap();
        meter_volume+=u32::from_str_radix(&input[2..4],16).unwrap()<<8;
        meter_volume+=u32::from_str_radix(&input[4..6],16).unwrap()<<16;
        meter_volume+=(u32::from_str_radix(&input[6..8],16).unwrap()&0b111)<<24;
        println!("{}", meter_volume);
    }else{
        panic!("No arguments provided");
    }


}