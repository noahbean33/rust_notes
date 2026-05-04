use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <in-file> <out-file>", args[0]);
        std::process::exit(1);
    }

    let key: u8 = 69;

    let mut in_file = File::open(&args[1]).unwrap();
    let mut out_file = File::create(&args[2]).unwrap();

    let mut buffer = Vec::new();
    in_file.read_to_end(&mut buffer).unwrap();

    for byte in &buffer {
        out_file.write_all(&[byte ^ key]).unwrap();
    }
}
