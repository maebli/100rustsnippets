extern crate core;
use std::env;


struct Flags{
    meterNoResponse:bool,
    meterEcoFrameError:bool,
    meterRollError:bool,
    brokenPipe:bool,
    lowBattery:bool,
    backFlow:bool,
    continuousFlow:bool,
    noUsage:bool,
    LinkError:bool
}

impl Flags{
    fn from(input:u8) -> Flags{
        let mut flags = Flags {
            meterNoResponse: false,
            meterEcoFrameError: false,
            meterRollError: false,
            brokenPipe: false,
            lowBattery: false,
            backFlow: false,
            continuousFlow: false,
            noUsage: false,
            LinkError: false
        };

        flags.meterNoResponse = (input & 1<<3) != 0;
        flags.meterEcoFrameError = (input & 1<<4) != 0;
        flags.meterRollError = (input & 1<<5) != 0;
        flags.brokenPipe = (input & 1<<6) != 0;
        flags.lowBattery = (input & 1<<7) != 0;
        flags.backFlow = (input & 1<<8) != 0;

        flags
    }

}

fn main() {

    let args: Vec<String> = env::args().collect();

    if let Some(input)=args.get(1){
        let input:Vec<&str> = input.split(";").collect();
        let mut payload = input.get(1).unwrap();
        let mut timestamp = input.get(0).unwrap();

        let mut meter_volume = u32::from_str_radix(&payload[0..2], 16).unwrap();
        meter_volume+=u32::from_str_radix(&payload[2..4],16).unwrap()<<8;
        meter_volume+=u32::from_str_radix(&payload[4..6],16).unwrap()<<16;
        meter_volume+=(u32::from_str_radix(&payload[6..8],16).unwrap()&0b111)<<24;

        let flags = Flags::from(u8::from_str_radix(&payload[15..17],16).unwrap());

        println!("{};{}", timestamp,meter_volume);

    }else{
        panic!("No arguments provided");
    }


}