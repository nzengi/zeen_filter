use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zeen_filter::filters::bloom_filter::BloomFilter;
use zeen_filter::utils::logging::log_insertion; // Log function for better monitoring

fn benchmark_bloom_filter(c: &mut Criterion) {
    let mut filter = BloomFilter::new(1_000_000, 0.01);

    // Benchmark insertions for different dataset sizes
    c.bench_function("insert 1 million items", |b| {
        b.iter(|| {
            for i in 0..1_000_000 {
                log_insertion(&format!("Inserting item_{}", i));  // Log each insertion
                filter.insert(black_box(format!("item_{}", i)));
            }
        })
    });

    // Additional test for smaller dataset
    c.bench_function("insert 100k items", |b| {
        b.iter(|| {
            for i in 0..100_000 {
                filter.insert(black_box(format!("item_{}", i)));
            }
        })
    });
}

criterion_group!(benches, benchmark_bloom_filter);
criterion_main!(benches);
