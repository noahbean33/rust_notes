fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    let mut n: i32 = 1;
    for _ in 0..3_000_000_000u64 {
        let square_result = square(n);
        n = n.wrapping_add(square_result);
    }
    println!("Value: {}", n);
}
