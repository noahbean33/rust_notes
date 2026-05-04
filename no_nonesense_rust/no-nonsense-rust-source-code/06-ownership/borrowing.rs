// this doesn't work, because number is passed by value (copied)
fn square_broken(mut number: i32) {
    number = number * number;
    println!("Inside the broken function: Number = {}", number);
}

// this works, because number is passed by MUTABLE REFERENCE
fn square_working(number: &mut i32) {
    *number = *number * *number;
    println!("Inside the working function: Number = {}", *number);
}

fn main() {
    let mut number = 6;
    println!("Trying to square the number {}", number);

    square_broken(number);
    println!("After passing by value: Number = {}", number);

    square_working(&mut number);
    println!("After passing by reference: Number = {}", number);
}
