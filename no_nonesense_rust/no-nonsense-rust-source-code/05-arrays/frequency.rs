use std::io::{self, Read};

// Requires a character_counts array of size 256 (all possible bytes)
fn print_all_character_counts(counts: &[u32; 256]) {
    for i in 32..126u8 {
        let freq = counts[i as usize];
        if freq != 0 {
            println!("{} -> {}", i as char, freq);
        }
    }
}

// A simple character frequency analyzer
fn main() {
    let mut counts: [u32; 256] = [0; 256];
    // counts[0]  -> value at the first "slot"
    // counts[59] -> value at the 60th "slot"

    for byte in io::stdin().bytes() {
        let b = byte.unwrap();
        counts[b as usize] += 1;
    }

    print_all_character_counts(&counts);
}
