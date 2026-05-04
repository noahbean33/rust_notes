use std::io::{self, Read};

fn read_line_manual() -> String {
    let mut buf: Vec<u8> = Vec::new();

    for byte in io::stdin().bytes() {
        let b = byte.unwrap();
        if b == b'\n' {
            break;
        }
        buf.push(b);
        println!("Buffer capacity: {} bytes", buf.capacity());
    }

    String::from_utf8(buf).unwrap()
}

fn main() {
    let text = read_line_manual();
    println!("You typed {}", text);
}
