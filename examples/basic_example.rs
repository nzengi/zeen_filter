use zeen_filter::filters::bloom_filter::BloomFilter;
use zeen_filter::utils::logging::log_murmur3_hash;

fn main() {
    // Create a Bloom Filter for 10,000 expected items with a 1% false positive rate
    let mut filter = BloomFilter::new(10_000, 0.01);

    // Insert some data into the Bloom Filter
    filter.insert("Blockchain");
    log_murmur3_hash(&"Blockchain");

    // Check if the data is in the filter
    if filter.contains("Blockchain") {
        println!("Blockchain might be in the filter");
    }

    // Check the current false positive rate
    println!("Current false positive rate: {}", filter.current_false_positive_rate());

    // Resize the filter if needed
    filter.resize_if_needed();
}
