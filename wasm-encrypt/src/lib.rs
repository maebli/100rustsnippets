mod utils;
use std::str;

use wasm_bindgen::prelude::*;
use aes::Aes128;
use block_modes::{BlockMode, BlockModeError, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

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
pub fn encrypt(plaintext: &str)  -> String {
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());
    hex::encode(ciphertext)
}


#[wasm_bindgen]
pub fn decrypt(encrypted: &str)  -> String {
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted = cipher.decrypt_vec(hex::decode(&encrypted).unwrap().as_ref());
    match decrypted{
        Ok(_) => String::from_utf8(decrypted.unwrap()).unwrap(),
        Err(x) => format!("{}",x)
    }

}
