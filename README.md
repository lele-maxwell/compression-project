# Compression Tool 

This is compression too for both compressing and decompressing files using **Run-Length Encoding (RLE)** and **LZ77** algorithms, available in both **Rust** and **JavaScript** . It is accompanied with bechmarking and automation via docker and CI/CD pipelines

## main features :
- Dual Implementation**: Rust (high-performance) and JavaScript (easy to follow).
-** Dockerized:**
-  **Benchmarking** : comparing performance (size time and compression ratio)


## How to install 

### locally:

**Rust:**

```bash
cd rust-compressor
cargo build --release
```

**JavaScript:**

```bash
cd js-compressor
npm install
```
### Docker:

# Rust
docker pull ghcr.io/lele-maxwell/rust-compressor:latest

# JavaScript
```
docker pull ghcr.io/lele-maxwell/js-compressor:latest
```

## How to  use :

### locally:
     
**Rust:**

```bash
cargo run -- compress input.txt output.txt --rle
```

**JavaScript:**

```bash
node index.js compress input.txt output.txt --rle
```

Replace `compress` with `decompress` as needed.

---


### Docker :

> Place `input.txt` in your working directory with its contents before running. 

**Rust Compressor:**

```bash
sudo docker run -v $(pwd):/data ghcr.io/lele-maxwell/rust-compressor compress /data/input.txt /data/output.txt --rle
```

**JavaScript Compressor:**

```bash
sudo docker run -v $(pwd):/data ghcr.io/lele-maxwell/js-compressor compress /data/input.txt /data/output.txt --rle
```



**the output will be created as output.txt** in the current working directory


## Testing :

**Rust :**
```bash
cd rust-compressor
cargo test
```


** JavaScript **

```bash
cd js-compressor
npm install
npx mocha test/*.test.js
```


## Benchmarking :

**run the command **
```bash
./benchmark.sh
```
##### properties:
- compression time
- decompression time
- original / compressed file size 
- compression ratio

**node:**

The result of the bench is stored and saved in the results/benchmark.md file 

## CI/CD with GitHub Actions:
**properties**:
 - checkout code 
 - builds both js and rust docker images 
 - runs test 
 - pushes both images to GHCR 


 ```text
ghcr.io/lele-maxwell/rust-compressor
ghcr.io/lele-maxwell/js-compressor
```

