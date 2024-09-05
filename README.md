# Zeen Filter

## Overview

Zeen Filter is a highly optimized Bloom filter implementation designed for high-performance applications. It provides a memory-efficient, scalable solution for large datasets while minimizing false positives.

### Key Features

- **Parallel Processing**: Zeen Filter uses `rayon` to handle large datasets efficiently.
- **Memory-Efficient**: The bit array is stored in bytes, reducing memory footprint.
- **Real-time Logging**: The library includes a logging system to track operations such as insertions and lookups.
- **Customizable False Positive Rate**: Automatically adjusts the size of the filter if the false positive rate exceeds a defined threshold.

### Installation

Add Zeen Filter to your `Cargo.toml` file:

```toml
[dependencies]
zeen_filter = "0.1.0"
```

## Example Usage

```rust
use zeen_filter::filters::bloom_filter::BloomFilter;
use zeen_filter::utils::logging::log_insertion;

fn main() {
    // Create a Bloom Filter for 10,000 expected items with a 1% false positive rate
    let mut filter = BloomFilter::new(10_000, 0.01);

    // Insert data into the filter
    filter.insert("Blockchain");
    log_insertion("Blockchain");

    // Check if data is in the filter
    if filter.contains("Blockchain") {
        println!("Blockchain might be in the filter");
    }

    if !filter.contains("Ethereum") {
        println!("Ethereum is definitely not in the filter");
    }
}
```

## Advanced Usage

```bash
carga bench
```


