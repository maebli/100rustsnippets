use serde::{Deserialize, Serialize};
use aes::Aes128;
use cmac::{Cmac, Mac, NewMac};
use std::result::Result;

fn main() {

    let payload = PHYPayload{
        mhdr: 0,
        mac_payload: JoinRequest{
            join_eui: 0,
            dev_eui: 0,
            dev_nonce: 0
        },
        mic: 0
    };

    let key = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    println!("{:?}",payload.calculate_mic(&key[..]));
}


#[derive(Serialize, Deserialize)]
struct JoinRequest{
    join_eui:u64,
    dev_eui:u64,
    dev_nonce:u16
}

#[derive(Serialize, Deserialize)]
struct PHYPayload<T>{
    mhdr:u8,
    mac_payload:T,
    mic:u32
}

impl PHYPayload<JoinRequest> {
    fn calculate_mic(&self, key:&[u8]) -> Result<Vec<u8>, &str>{
        let mut serializedPayload = bincode::serialize(&self.mhdr)
            .expect("Failed to serialize MHDR.");
        serializedPayload.append(&mut bincode::serialize(&self.mac_payload)
            .expect("Failed to serialize MAC Payload."));
        let mut mac = Cmac::<Aes128>::new_from_slice(key)
            .expect("Bad AES128 Key.");
        mac.update(&serializedPayload[..]);
        let result = mac.finalize();
        let tag_bytes = result.into_bytes();
        Ok(Vec::from(&tag_bytes[0..4]))
    }
}


