mod compressors;
mod decompressers;

use std::collections::HashMap; 

fn main() {
    let input_string = "maxwell maxwell ";

let mut char_frequency = HashMap::new();
    for ch in input_string.chars() {
        let count = char_frequency.entry(ch).or_insert(0);
        *count += 1;
    }

    println!("Character Frequencies: {:?}", char_frequency);
}
