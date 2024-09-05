use zeen_filter::filters::bloom_filter::BloomFilter;
use zeen_filter::utils::logging::log_insertion;  // Import logging for better monitoring

#[test]
fn test_large_insertion() {
    // Create a Bloom Filter for 1 million items with a 1% false positive rate
    let mut filter = BloomFilter::new(1_000_000, 0.01);
    
    // Insert 1 million items into the Bloom Filter
    for i in 0..1_000_000 {
        log_insertion(&format!("Inserting item_{}", i));  // Log each insertion
        filter.insert(format!("item_{}", i));
    }

    // Check if specific items are correctly identified
    assert!(filter.contains("item_999999"));  // Verify that the last inserted item is in the filter
    assert!(!filter.contains("item_1000001"));  // Verify that an uninserted item is not in the filter
}

#[test]
fn test_false_positive_rate() {
    // Create a Bloom Filter for 10,000 items with a 1% false positive rate
    let mut filter = BloomFilter::new(10_000, 0.01);
    
    // Insert 10,000 items into the Bloom Filter
    for i in 0..10_000 {
        filter.insert(format!("test_item_{}", i));
    }

    // Test if the false positive rate is within acceptable limits
    let false_positives = (10_000..20_000)
        .filter(|i| filter.contains(format!("test_item_{}", i)))
        .count();
    
    // Calculate the actual false positive rate
    let actual_false_positive_rate = false_positives as f64 / 10_000.0;
    println!("Actual False Positive Rate: {}", actual_false_positive_rate);

    assert!(actual_false_positive_rate <= 0.01);  // Assert that the false positive rate is within 1%
}
