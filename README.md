# Rust Experiments - Fibonacci Calculator

Naive implementation to calculate the first n numbers of the Fibonacci series.

## Build

Rust and Cargo are required to build this project. Either install them via [rustup](https://rustup.rs/) or use the build_binary.sh script to build using the rust docker image.

### Build using Cargo
```bash
cargo build --release
```

### Build using Docker
**NOTE**: Requires Docker 
```bash
./build_binary.sh
```

The built binary will be present in ./target/release/

## Usage

The tool takes a single integer as the argument as the limit and will print out all numbers in the Fibonacci series until the limit.

```bash
./rust-experiments-fibonacci [integer > 0]
```

## Docker Image
Included dockerfile can be used to create a docker image with the tool as the entrypoint.

### Building Docker Image
```bash
docker build -t fibonacci-calculator:latest .
```

### Running Docker Image
```bash
docker run --rm fibonacci-calculator:latest [integer > 0]
```
