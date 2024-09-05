use mur3::Hasher128;  // Correct import for Murmur3 Hasher
use std::hash::{Hash, Hasher};  // Import Hash trait and Hasher for DefaultHasher
use serde::Serialize;  // Import Serialize for bincode
use bincode;  // Bincode for serialization

/// CustomHasher struct to manage both Murmur3 and default hashing mechanisms.
pub struct CustomHasher;

impl CustomHasher {
    /// This function uses Murmur3 hashing for optimal performance on large datasets.
    /// It serializes the item and then hashes it using Murmur3.
    pub fn hash_with_murmur3<T: Hash + Serialize>(&self, item: &T) -> u128 {
        let mut hasher = Hasher128::with_seed(0);  // Initialize Murmur3 hasher with a seed
        let mut buffer = vec![];

        // Serialize the item to bytes and feed it into the Murmur3 hasher
        bincode::serialize_into(&mut buffer, item).expect("Failed to serialize item for hashing");
        hasher.write(&buffer);

        // Get the 128-bit hash value as a tuple (u64, u64)
        let (high, low) = hasher.finish128();
        // Convert the tuple to a single u128 value
        ((high as u128) << 64) | (low as u128)
    }

    /// This function uses the DefaultHasher as a fallback.
    /// It is useful for smaller datasets or when Murmur3 is not needed.
    pub fn hash_with_default<T: Hash>(&self, item: &T, seed: usize) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish() as usize
    }
}
