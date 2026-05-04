use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sum = 0;

    // Start at index 1 to skip the program name
    for i in 1..args.len() {
        let number: i32 = args[i].parse().unwrap();
        sum = sum + number;
    }

    println!("The sum is {}", sum);
}
