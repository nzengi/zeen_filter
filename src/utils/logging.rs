use log::{info, warn};

/// Logging utility to track operations within Zeen Filter.
/// This system logs important operations such as insertions and lookups.
pub fn log_insertion(item: &str) {
    info!("Inserting item into Zeen Filter: {}", item);
}

pub fn log_lookup(item: &str, found: bool) {
    if found {
        info!("Item '{}' might be in the filter.", item);
    } else {
        warn!("Item '{}' is definitely not in the filter.", item);
    }
}

pub fn log_murmur3_hash<T: std::fmt::Debug>(item: &T) {
    println!("Murmur3 hash computed for item: {:?}", item);
}
