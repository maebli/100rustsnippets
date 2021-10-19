fn main() {
    let hex: String = String::from("0123456789ABCDEFabcdef");

    for b in hex.bytes(){
        println!("ascii hex byte {} => integer {}", b, ascii_hex_to_u8(b));
    }
}

fn ascii_hex_to_u8(c: u8) -> u8 {
    match c {
        c if c >= b'0' && c <= b'9' => c - b'0',
        c if c >= b'a' && c <= b'f' => c - b'a' + 10,
        c if c >= b'A' && c <= b'F' => c - b'A' + 10,
        _ => panic!()
    }
}
