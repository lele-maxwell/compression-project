#!/bin/bash

# Create results directory
mkdir -p results

# Test files and algorithms
TEST_FILES=("rle-test-file.txt" "lz-test-file.txt")
ALGORITHMS=("--rle" "--lz")

# Results file
RESULTS_FILE="results/benchmark.txt"
echo "Benchmark Results" > "$RESULTS_FILE"
echo "=================" >> "$RESULTS_FILE"

# Function to convert bytes to human-readable format
human_readable() {
    local bytes=$1
    local kib=1024
    local mib=$((kib * 1024))
    local gib=$((mib * 1024))

    if (( bytes >= gib )); then
        printf "%.2f GB" "$(bc -l <<< "$bytes / $gib")"
    elif (( bytes >= mib )); then
        printf "%.2f MB" "$(bc -l <<< "$bytes / $mib")"
    elif (( bytes >= kib )); then
        printf "%.2f KB" "$(bc -l <<< "$bytes / $kib")"
    else
        printf "%d B" "$bytes"
    fi
}

# Function to run benchmark
run_benchmark() {
    local impl=$1
    local file=$2
    local algo=$3
    local operation=$4

    echo ""
    echo ">> $operation - $impl on $file using $algo"

    # Run and time the compression or decompression
    time_output=$( { time docker run -v $(pwd):/data ${impl}-compressor $operation /data/$file /data/results/${file}.${operation}.${impl}.${algo/--/} $algo; } 2>&1 )

    # Extract time
    real_time=$(echo "$time_output" | grep real | awk '{print $2}')

    # File paths and sizes
    input_path="$file"
    output_path="results/${file}.${operation}.${impl}.${algo/--/}"

    original_size=$(stat -c %s "$input_path")
    result_size=$(stat -c %s "$output_path")

    original_hr=$(human_readable "$original_size")
    result_hr=$(human_readable "$result_size")

    # Compression Ratio
    if [ "$operation" == "compress" ]; then
        ratio=$(echo "scale=2; $result_size / $original_size * 100" | bc)
    else
        ratio=$(echo "scale=2; $original_size / $result_size * 100" | bc)
    fi

    # Output to terminal
    echo "Implementation : $impl"
    echo "Operation      : $operation"
    echo "File           : $file"
    echo "Algorithm      : $algo"
    echo "Time Taken     : $real_time"
    echo "Original Size  : $original_hr"
    echo "Result Size    : $result_hr"
    echo "Ratio          : $ratio %"

    # Output to file
    {
        echo ""
        echo "----------------------------------------"
        echo "Implementation : $impl"
        echo "Operation      : $operation"
        echo "File           : $file"
        echo "Algorithm      : $algo"
        echo "Time Taken     : $real_time"
        echo "Original Size  : $original_hr"
        echo "Result Size    : $result_hr"
        echo "Ratio          : $ratio %"
        echo "----------------------------------------"
    } >> "$RESULTS_FILE"
}

# Run benchmarks
for file in "${TEST_FILES[@]}"; do
    for algo in "${ALGORITHMS[@]}"; do
        # Skip mismatched tests
        if [[ "$file" == "rle-test-file.txt" && "$algo" == "--lz" ]] || \
           [[ "$file" == "lz-test-file.txt" && "$algo" == "--rle" ]]; then
            continue
        fi

        for impl in "rust" "js"; do
            run_benchmark "$impl" "$file" "$algo" "compress"
            run_benchmark "$impl" "$file" "$algo" "decompress"
        done
    done
done

echo ""
echo "Benchmarking complete! See $RESULTS_FILE for results."
