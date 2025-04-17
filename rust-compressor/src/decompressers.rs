fn rle_decode(encoded: String) -> String {
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






fn lz77_decode(triples: &[(usize, usize, char)]) -> String {
    let mut output = String::new();

    for &(offset, length, next_char) in triples {
        if offset == 0 && length == 0 {
            output.push(next_char);
        } else {
            let start = output.len().saturating_sub(offset);
            let copied: String = output.chars().skip(start).take(length).collect();
            output.push_str(&copied);
            output.push(next_char);
        }
    }

    output
}











