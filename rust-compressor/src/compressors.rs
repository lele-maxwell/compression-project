#![allow(warnings)]




// RLE compression
pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        let mut count = 1u8;
        while (i + (count as usize)) < data.len() && data[i] == data[i + (count as usize)] && count < 255 {
            count += 1;
        }
        
        compressed.push(count);
        compressed.push(data[i]);
        i += count as usize;
    }
    
    compressed
}

// LZ77 compression 
pub fn lz77_encode(data: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        let mut best_match = (0u16, 0u8);
        
        
        let start = if i > 32768 { i - 32768 } else { 0 };
        for j in start..i {
            let mut match_len = 0u8;
            while (i + (match_len as usize)) < data.len() && 
                  (j + (match_len as usize)) < i && 
                  data[j + (match_len as usize)] == data[i + (match_len as usize)] && 
                  match_len < 255 {
                match_len += 1;
            }
            
            if match_len > best_match.1 {
                best_match = ((i - j) as u16, match_len);
            }
        }
        
        if best_match.1 >= 3 {
            
            compressed.push(0x01);
            compressed.push((best_match.0 >> 8) as u8);
            compressed.push(best_match.0 as u8);
            compressed.push(best_match.1);
            i += best_match.1 as usize;
        } else {
            
            compressed.push(0x00);
            compressed.push(data[i]);
            i += 1;
        }
    }
    
    compressed
}
