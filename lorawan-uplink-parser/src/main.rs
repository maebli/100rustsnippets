use std::fmt;
use byteorder::{BigEndian, ByteOrder};

struct JoinRequest{
    join_eui:u64,
    dev_eui:u64,
    dev_nonce:u16
}

impl fmt::Display for JoinRequest{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "deveui:{:#8x},joineui:{:#8x},devnonce:{:#2x}", self.dev_eui, self.join_eui, self.dev_nonce)
    }
}

enum MType{
    JoinRequest{mhdr:u8,message:JoinRequest,mic:u32},
    _JoinAccept(),
    _UnconfirmedDataUp(),
    _UnconfirmedDataDown(),
    _ConfirmedDataUp(),
    _ConfirmedDataDown(),
    _RejoinRequest(),
    _Proprietary()
}

impl MType{
    fn get_major(&self) -> Option<u8> {
        match self{
            MType::JoinRequest{mhdr,..} => Some(3 & mhdr),
            _ => {println!("not implemented!"); None}
        }
    }
}

impl fmt::Display for MType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            MType::JoinRequest{message, mic,.. } =>{
                write!(f, "major = {:#02x}, mic = {:#04x}, joinrequest = {}",
                       self.get_major().unwrap(), *mic, *message
                )
            }
            _ => write!(f, "Error")
        }
    }
}


fn main() {

    /*
        Message with MHDR = 01,
        join eui = 01,02,03,04,05,,06,07,08
        dev eui = 01,02,03,04,05,06,07,08
        dev nuance = 00,01
        mic = 01,02,03,04
     */

    let message:Vec<u8> = vec![1,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,0,1,1,2,3,4];

    let m=parse(&message);


    match m {
        None => println!("Failed to parse!"),
        _ => {
            println!("{}",m.unwrap())
        }
    }


}


fn parse(message:&Vec<u8>) -> Option<MType> {
    let mhdr = message.get(0);

    match mhdr.unwrap()>>5 {
        0 => Some(MType::JoinRequest {
            mhdr: *mhdr.unwrap(),
            message: JoinRequest {
                join_eui: BigEndian::read_u64(&message[1..9]),
                dev_eui: BigEndian::read_u64(&message[9..17]),
                dev_nonce: BigEndian::read_u16(&message[17..19])
            },
            mic: BigEndian::read_u32(&message[19..23])
        }),
        _ => None
    }
}
