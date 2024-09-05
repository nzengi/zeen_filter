use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use rayon::prelude::*;  // For parallel processing
use crate::hashing::hash_functions::CustomHasher;
use serde::Serialize;
use std::hash::Hasher;

/// Optimized Bloom Filter implementation.
/// This structure provides efficient memory usage with a scalable approach
/// to handle large datasets with minimum false positives.
pub struct BloomFilter {
    bit_array: Vec<u8>,  // Memory-efficient bit array stored in bytes
    size: usize,         // The size of the bit array in bits
    num_hashes: usize,   // Number of hash functions used
    inserted_items: usize, // Number of inserted items
}

impl BloomFilter {
    /// Creates a new Bloom Filter optimized for the expected number of items.
    /// 
    /// # Parameters
    /// - `expected_items`: Number of expected items.
    /// - `false_positive_rate`: The desired false positive rate (between 0 and 1).
    ///
    /// # Returns
    /// A new instance of `BloomFilter` initialized with the optimal size and number of hashes.
    pub fn new(expected_items: usize, false_positive_rate: f64) -> BloomFilter {
        let size = BloomFilter::optimal_size(expected_items, false_positive_rate);  // Calculate optimal size
        let num_hashes = BloomFilter::optimal_num_hashes(expected_items, size);     // Calculate optimal number of hash functions
        BloomFilter {
            bit_array: vec![0; size / 8 + 1],  // Initialize bit array with optimal size
            size,
            num_hashes,
            inserted_items: 0,
        }
    }

    /// Inserts an item into the Bloom Filter using Murmur3 for hashing.
    ///
    /// # Parameters
    /// - `item`: The item to be inserted, implementing `Serialize` and `Hash`.
    pub fn insert_with_murmur3<T: Hash + Serialize>(&mut self, item: T) {
        let hasher = CustomHasher;
        let size = self.size;

        // Calculate the hash and set the corresponding bit in the bit array
        let hash = (hasher.hash_with_murmur3(&item) as usize) % size;
        self.set_bit(hash);
        self.inserted_items += 1;
    }

    /// Inserts an item into the Bloom Filter using parallel processing.
    /// 
    /// # Parameters
    /// - `item`: The item to be inserted, implementing the `Hash` trait.
    pub fn insert<T: Hash + Sync>(&mut self, item: T) {
        let size = self.size;

        // Parallel hashing for efficiency
        let hashes: Vec<usize> = (0..self.num_hashes)
            .into_par_iter()
            .map(|i| self.hash_with_seed(&item, i) % size)
            .collect();

        // Set the corresponding bits in the bit array for each hash
        hashes.into_iter().for_each(|index| {
            self.set_bit(index);
        });
        self.inserted_items += 1;
    }

    /// Checks whether an item might be in the Bloom Filter.
    ///
    /// # Parameters
    /// - `item`: The item to be checked, implementing `Hash`.
    ///
    /// # Returns
    /// - `true` if the item might be in the filter, `false` if it is definitely not in the filter.
    pub fn contains<T: Hash + Sync>(&self, item: T) -> bool {
        let size = self.size;
        let hashes: Vec<usize> = (0..self.num_hashes)
            .into_par_iter()
            .map(|i| self.hash_with_seed(&item, i) % size)
            .collect();

        hashes.into_iter().all(|index| self.get_bit(index))
    }

    /// Calculates the optimal size for the bit array based on the expected items and false positive rate.
    fn optimal_size(expected_items: usize, false_positive_rate: f64) -> usize {
        let ln2 = std::f64::consts::LN_2;
        let size = -(expected_items as f64 * false_positive_rate.ln() / ln2.powi(2)).ceil();
        size as usize
    }

    /// Calculates the optimal number of hash functions to use based on the filter size and number of items.
    fn optimal_num_hashes(expected_items: usize, size: usize) -> usize {
        let k = (size as f64 / expected_items as f64 * std::f64::consts::LN_2).ceil();
        k as usize
    }

    /// Sets a specific bit in the bit array.
    fn set_bit(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_position = index % 8;
        self.bit_array[byte_index] |= 1 << bit_position;
    }

    /// Retrieves the value of a specific bit in the bit array.
    fn get_bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_position = index % 8;
        (self.bit_array[byte_index] & (1 << bit_position)) != 0
    }

    /// Hashes an item with a seed value to generate multiple hash outputs.
    fn hash_with_seed<T: Hash>(&self, item: &T, seed: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish() as usize
    }

    pub fn current_false_positive_rate(&self) -> f64 {
        let num_hashes = self.num_hashes as f64;
        let inserted_items = self.inserted_items as f64;
        let size = self.size as f64;

        // Calculate the false positive rate
        (1.0 - (-num_hashes * inserted_items / size).exp()).powi(num_hashes as i32)
    }

    pub fn resize_if_needed(&mut self) {
        let current_fp_rate = self.current_false_positive_rate();
        if current_fp_rate > 0.01 {  // Example threshold for resizing
            let new_size = BloomFilter::optimal_size(self.inserted_items, 0.01);  // Default to 1% false positive rate
            self.size = new_size;
            self.bit_array = vec![0; new_size / 8 + 1];  // Rebuild the bit array with the new size
        }
    }
}
