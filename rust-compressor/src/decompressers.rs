#![allow(warnings)]

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        let count = data[i];
        let value = data[i + 1];
        
        for _ in 0..count {
            decompressed.push(value);
        }
        
        i += 2;
    }
    
    decompressed
}

pub fn lz77_decode(data: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        let flag = data[i];
        i += 1;
        
        if flag == 0x00 {
            // Copy literal
            decompressed.push(data[i]);
            i += 1;
        } else {
            // Copy from previous output
            let offset = ((data[i] as u16) << 8) | (data[i + 1] as u16);
            let length = data[i + 2];
            i += 3;
            
            let start = decompressed.len() - offset as usize;
            for j in 0..length {
                decompressed.push(decompressed[start + j as usize]);
            }
        }
    }
    
    decompressed
}












