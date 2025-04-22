#![allow(warnings)]

use std::env;
use std::fs;
use std::process;
use std::time::Instant;
use compressors::compress;

mod compressors;
mod decompressers;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 5 {
        eprintln!("Usage: {} compress|decompress <input_file> <output_file> --rle|--lz", args[0]);
        process::exit(1);
    }

    let operation = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];
    let algorithm = &args[4];

    // Read input file
    let input_data = match fs::read(input_file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            process::exit(1);
        }
    };

    // Process data
    let result = match (operation.as_str(), algorithm.as_str()) {
        ("compress", "--rle") => compressors::compress(&input_data),
        ("compress", "--lz") => compressors::lz77_encode(&input_data),
        ("decompress", "--rle") => decompressers::decompress(&input_data),
        ("decompress", "--lz") => decompressers::lz77_decode(&input_data),
        _ => {
            eprintln!("Invalid operation or algorithm. Use compress|decompress with --rle|--lz");
            process::exit(1);
        }
    };

    // Write output file
    if let Err(e) = fs::write(output_file, result) {
        eprintln!("Error writing output file: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compressors::compress(input);
        let decompressed = decompressers::decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
        let compressed = compressors::lz77_encode(input);
        let decompressed = decompressers::lz77_decode(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
