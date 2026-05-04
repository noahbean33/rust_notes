use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sum = 0;

    // Bug 1: starts at index 0 (program name is not a number — will crash)
    // Bug 2: should start at 1 to skip the program name
    for i in 0..args.len() {
        let number: i32 = args[i].parse().unwrap();
        sum = sum + number;
    }

    println!("The sum is {}", sum);
}
