fn modify_array(array: &mut [i32]) {
    array[0] = 7;
    array[1] = 7;
    array[2] = 7;
}

fn print_array(array: &[i32]) {
    println!("array[0] = {}", array[0]);
    println!("array[1] = {}", array[1]);
    println!("array[2] = {}", array[2]);
}

fn main() {
    println!("Creating an array ...");
    let mut array = [0, 4, 9];
    print_array(&array);

    println!("Modifying array ...");
    modify_array(&mut array);
    print_array(&array);
}
