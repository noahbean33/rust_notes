fn main() {
    let numbers = [10, 20, 30, 40, 50];

    // Iterate over the array using a for..in loop
    // The for loop borrows the array immutably by default
    for num in numbers {
        println!("{}", num);
    }

    // Iterate with index using enumerate
    for (index, num) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", index, num);
    }
}