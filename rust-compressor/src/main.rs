mod compressors;
mod decompressers;

use std::fs;
use std::process;
use std::io::{self, Write};
use std::path::Path;

fn print_usage() {
    eprintln!("Usage: compress|decompress <input_file> <output_file> --rle|--lz");
    eprintln!("Example: compress input.txt output.compressed --rle");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        println!("compress|decompress <input_file> <output_file> --rle|--lz");
        return;
    }
    fs::create_dir_all("compressed").expect("Failed to create compressed directory");
    fs::create_dir_all("decompressed").expect("Failed to create decompressed directory");

    let input_file = &args[2];
    let output_file = &args[3];
    let compressor_type = &args[4];

    let output_path = if args[1] == "compress" {
        Path::new("compressed").join(output_file)
    } else {
        Path::new("decompressed").join(output_file)
    };

    let input_string = fs::read_to_string(input_file).expect("Failed to read input file");

    if args[1] == "compress" && compressor_type == "--rle" {
        let compressed = compressors::rle(input_string.clone());
        fs::write(&output_path, compressed).expect("Failed to write output file");
    }

    if args[1] == "compress" && compressor_type == "--lz" {
        let compressed = compressors::lz77_encode(&input_string);
        fs::write(&output_path, compressed).expect("Failed to write output file");
    }

    if args[1] == "decompress" && compressor_type == "--rle" {
        let decompressed = decompressers::rle_decode(input_string.clone());
        fs::write(&output_path, decompressed).expect("Failed to write output file");
    }

    if args[1] == "decompress" && compressor_type == "--lz" {
        let decompressed = decompressers::lz77_decode(&input_string);
        fs::write(&output_path, decompressed).expect("Failed to write output file");
    }

    println!("Operation completed successfully!");
    println!("Input file: {}", input_file);
    println!("Output file: {}", output_path.display());
    println!("Algorithm: {}", compressor_type);
}