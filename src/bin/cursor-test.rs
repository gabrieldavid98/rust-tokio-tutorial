use std::io::Cursor;

use bytes::Buf;

fn main() {
    let v: Vec<u8> = vec![0x6C, 0x81, 0xFF];
    let mut c = Cursor::new(v);
    println!("{}", c.get_u16());
    println!("{}", c.get_u8());
}
