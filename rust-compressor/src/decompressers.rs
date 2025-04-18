pub fn rle_decode(encoded: String) -> String {
    let mut decoded = String::new();
    let chars: Vec<char> = encoded.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        i += 1;

        let mut count_str = String::new();
        while i < chars.len() && chars[i].is_ascii_digit() {
            count_str.push(chars[i]);
            i += 1;
        }

        let count: usize = count_str.parse().unwrap_or(1);
        decoded.push_str(&ch.to_string().repeat(count));
    }

    decoded
}


pub fn lz77_decode(input: &String) -> String {
    let mut output = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        if chars[i] == '(' {
            i += 1;
            let mut offset_str = String::new();
            while chars[i] != ',' {
                offset_str.push(chars[i]);
                i += 1;
            }
            i += 1;

            let mut length_str = String::new();
            while chars[i] != ',' {
                length_str.push(chars[i]);
                i += 1;
            }
            i += 1;

            let offset: usize = offset_str.parse().unwrap_or(0);
            let length: usize = length_str.parse().unwrap_or(0);
            let next_char = chars[i];
            i += 2; // Skip the closing parenthesis

            if offset == 0 && length == 0 {
                output.push(next_char);
            } else {
                let start = output.len().saturating_sub(offset);
                let copied: String = output.chars().skip(start).take(length).collect();
                output.push_str(&copied);
                output.push(next_char);
            }
        }
    }

    output
}











