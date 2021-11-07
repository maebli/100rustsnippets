mod utils;

use wasm_bindgen::prelude::*;
use crc16::{EN_13757, State};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn crc16_en_13757(s: &str) -> String {
    let a= hex::decode(&s);
    match a {
        Ok(_) => {format!("{:#04x}",State::<EN_13757>::calculate(&a.unwrap()))},
        Err(_) => String::from("Error calculating CRC16, invalid input.")
    }
}
