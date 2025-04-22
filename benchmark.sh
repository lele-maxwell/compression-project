#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create results directory
mkdir -p results

# Test files
TEST_FILES=("rle-test-file.txt" "lz-test-file.txt")
ALGORITHMS=("--rle" "--lz")

# Function to run benchmark
run_benchmark() {
    local impl=$1
    local file=$2
    local algo=$3
    local operation=$4
    
    echo -e "${BLUE}Running $operation with $impl on $file using $algo${NC}"
    
    # Run the benchmark and capture output
    if [ "$impl" == "rust" ]; then
        time_output=$( { time docker run -v $(pwd):/data rust-compressor $operation /data/$file /data/results/${file}.${operation} $algo; } 2>&1 )
    else
        time_output=$( { time docker run -v $(pwd):/data js-compressor $operation /data/$file /data/results/${file}.${operation} $algo; } 2>&1 )
    fi
    
    # Extract real time
    real_time=$(echo "$time_output" | grep real | awk '{print $2}')
    
    # Get file sizes
    original_size=$(stat -f %z "$file")
    result_size=$(stat -f %z "results/${file}.${operation}")
    
    # Calculate compression ratio
    if [ "$operation" == "compress" ]; then
        ratio=$(echo "scale=2; $result_size / $original_size * 100" | bc)
    else
        ratio=$(echo "scale=2; $original_size / $result_size * 100" | bc)
    fi
    
    # Append to results file
    echo "| $impl | $file | $algo | $operation | $real_time | $original_size | $result_size | $ratio% |" >> results/benchmark.md
}

# Create markdown table header
echo "| Implementation | File | Algorithm | Operation | Time | Original Size | Result Size | Ratio |" > results/benchmark.md
echo "|---------------|------|-----------|-----------|------|---------------|-------------|-------|" >> results/benchmark.md

# Run benchmarks
for file in "${TEST_FILES[@]}"; do
    for algo in "${ALGORITHMS[@]}"; do
        # Skip RLE test for LZ test file and vice versa
        if [[ "$file" == "rle-test-file.txt" && "$algo" == "--lz" ]] || \
           [[ "$file" == "lz-test-file.txt" && "$algo" == "--rle" ]]; then
            continue
        fi
        
        # Run compression and decompression for both implementations
        run_benchmark "rust" "$file" "$algo" "compress"
        run_benchmark "rust" "$file" "$algo" "decompress"
        run_benchmark "js" "$file" "$algo" "compress"
        run_benchmark "js" "$file" "$algo" "decompress"
    done
done

# Generate summary
echo -e "\n${GREEN}Benchmark completed! Results saved to results/benchmark.md${NC}"
echo -e "\n${BLUE}Summary:${NC}"
cat results/benchmark.md 