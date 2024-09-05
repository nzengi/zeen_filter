use zeen_filter::filters::bloom_filter::BloomFilter;
use zeen_filter::utils::logging::log_insertion;  // Import logging for insertion monitoring

#[test]
fn test_insert_and_lookup() {
    // Create a Bloom Filter for 100 items with a 1% false positive rate
    let mut filter = BloomFilter::new(100, 0.01);

    // Insert "Hello" and "World" into the Bloom Filter
    log_insertion("Hello");  // Log the insertion process for "Hello"
    filter.insert("Hello");
    log_insertion("World");  // Log the insertion process for "World"
    filter.insert("World");

    // Check if the inserted items are present in the filter
    assert!(filter.contains("Hello"));  // "Hello" should be in the filter
    assert!(filter.contains("World"));  // "World" should be in the filter

    // Check if a non-inserted item is correctly identified as absent
    assert!(!filter.contains("NotInFilter"));  // "NotInFilter" should not be in the filter
}
