pub fn rle(input: String) -> String {
    let mut encoded = String::new();
    let mut chars = input.chars();
    let mut prev = match chars.next() {
        Some(c) => c,
        None => return encoded, 
    };

    let mut count = 1;

    for current in chars {
        if current == prev {
            count += 1;
        } else {
            encoded.push(prev);
            encoded.push_str(&count.to_string());

            prev = current;
            count = 1;
        }
    }

    encoded.push(prev);
    encoded.push_str(&count.to_string());

    encoded
}

pub fn lz77_encode(input: &str) -> String {
    const WINDOW_SIZE: usize = 20; 
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut cursor = 0;

    while cursor < chars.len() {
        let mut max_match_length = 0;
        let mut match_offset = 0;
        let mut match_found = false;

        let start = if cursor >= WINDOW_SIZE { cursor - WINDOW_SIZE } else { 0 };

        for look_back in start..cursor {
            let mut length = 0;

            while look_back + length < cursor &&
                  cursor + length < chars.len() &&
                  chars[look_back + length] == chars[cursor + length] {
                length += 1;
            }

            if length > max_match_length {
                max_match_length = length;
                match_offset = cursor - look_back;
                match_found = true;
            }
        }

        if match_found && cursor + max_match_length < chars.len() {
            let next_char = chars[cursor + max_match_length];
            result.push_str(&format!("({},{},{})", match_offset, max_match_length, next_char));
            cursor += max_match_length + 1;
        } else {
            result.push_str(&format!("(0,0,{})", chars[cursor]));
            cursor += 1;
        }
    }

    result
}
