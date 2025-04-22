# Rust Compression Tool

A command-line tool for compressing and decompressing files using RLE (Run-Length Encoding) and LZ77 algorithms, written in Rust. Includes both Rust and JavaScript implementations for performance comparison.

## Features

- **RLE Compression**: Efficient compression for files with repeated sequences
- **LZ77 Compression**: Advanced compression using sliding window and dictionary-based encoding
- **Dual Implementation**: Both Rust and JavaScript versions for performance comparison
- **Automated Benchmarking**: Comprehensive performance testing and metrics collection
- **File Operations**: Support for compressing and decompressing any file type

## Installation

1. Ensure you have Rust installed (https://rustup.rs/)
2. Ensure you have Node.js installed (https://nodejs.org/)
3. Clone this repository
4. Build the Rust project:
```bash
cargo build --release
```

## Project Structure

```
compression-project/
├── rust-compressor/         # Rust implementation
│   ├── src/                # Source code
│   ├── Cargo.toml          # Rust dependencies
│   └── benchmark.sh        # Benchmarking script
├── js-compressor/          # JavaScript implementation
│   ├── compress.js         # Compression implementation
│   └── decompress.js       # Decompression implementation
├── rle-test-file.txt       # Test file for RLE algorithm
├── lz-test-file.txt        # Test file for LZ77 algorithm
└── results/                # Benchmark results directory
```

## Usage

### Command Line Interface

The program accepts the following command-line arguments:

```bash
cargo run <operation> <input_file> <output_file> <algorithm>
```

Where:
- `operation`: Either "compress" or "decompress"
- `input_file`: Path to the input file
- `output_file`: Path where the output will be saved
- `algorithm`: Either "--rle" for Run-Length Encoding or "--lz" for LZ77

### Examples

Compress a file using RLE:
```bash
cargo run compress input.txt compressed.rle --rle
```

Decompress an RLE-compressed file:
```bash
cargo run decompress compressed.rle decompressed.txt --rle
```

Compress a file using LZ77:
```bash
cargo run compress input.txt compressed.lz --lz
```

Decompress an LZ77-compressed file:
```bash
cargo run decompress compressed.lz decompressed.txt --lz
```

## Benchmarking

The project includes an automated benchmarking system that tests both Rust and JavaScript implementations.

### Test Files

The benchmarking system uses two specific test files:
1. `rle-test-file.txt`: Contains repeated patterns (e.g., "AAAABBBBCCCC")
2. `lz-test-file.txt`: Contains repeated patterns and unique sequences (e.g., "ABABABAB")

These files are automatically created if they don't exist.

### Running Benchmarks

To run the benchmarks:

```bash
cd rust-compressor
chmod +x benchmark.sh
./benchmark.sh
```

### Benchmark Results

The script generates detailed results for each test in the `results` directory. Each result file contains:
- File name and size
- Implementation used (Rust/JavaScript)
- Algorithm used (RLE/LZ77)
- Operation performed (compress/decompress)
- Compression ratio (for compression operations)
- Processing time
- Throughput (bytes/second)

Result files are named in the format:
```
results/<implementation>_<algorithm>_<operation>_<filename>.txt
```

Example: `results/rust_rle_compress_rle-test-file.txt`

### Performance Metrics

The benchmarking system measures:
1. **Compression Ratio**: Original size / Compressed size
2. **Processing Time**: Time taken for the operation
3. **Throughput**: Bytes processed per second
4. **Memory Usage**: Memory consumption during operations

## Algorithms

### RLE (Run-Length Encoding)
- Simple and efficient for files with repeated sequences
- Each repeated sequence is encoded as (count, value) pairs
- Maximum run length of 255 bytes
- Best for files with many repeated bytes

### LZ77
- More sophisticated compression algorithm
- Uses a sliding window of 32KB for finding matches
- Encodes matches as (offset, length) pairs
- Minimum match length of 3 bytes
- Best for files with repeated patterns

## Testing

Run the test suite:
```bash
cargo test
```

The test suite includes:
- Round-trip tests for both algorithms
- Edge case handling
- Performance benchmarks

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 