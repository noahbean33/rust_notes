use std::env;

const PI: f64 = 3.14;

fn own_strlen(s: &str) -> usize {
    // count bytes manually
    let mut count = 0;
    for _ in s.bytes() {
        count += 1;
    }
    count
}

fn modify_str(s: &mut String) {
    s.clear();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <string-enclosed-in-quotes>", args[0]);
        return;
    }

    // Some string length counting
    let input_length = own_strlen(&args[1]);
    println!("String length is: {}", input_length);

    // copying the user input string into my own buffer
    let destination_buffer = args[1].clone();

    println!("Copied string is: {}", destination_buffer);
    println!("PI is {}", PI);
    // destination_buffer is freed automatically when it goes out of scope
}
