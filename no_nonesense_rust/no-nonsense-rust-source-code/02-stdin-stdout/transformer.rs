use std::io::{self, Read, Write};

fn main() {
    let key: u8 = 42;

    // u8 -> 8 bit number (values 0..255)
    for byte in io::stdin().bytes() {
        let b = byte.unwrap();
        // write ANY transformation here -> converts ANY input to ANY output
        io::stdout().write_all(&[b ^ key]).unwrap();
    }
}
