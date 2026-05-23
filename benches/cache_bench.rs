use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::{Duration, Instant};

fn bench_effectiveness(c: &mut Criterion) {
    let size = 1024 * 1024 * 32; // 32MB
    let data = vec![1u8; size];
    let mut group = c.benchmark_group("Cache Effectiveness");

    // Warm access: The data is likely to be in cache after the first few iterations.
    group.bench_function("sum_warm", |b| {
        b.iter(|| {
            let sum: u64 = black_box(&data).iter().map(|&x| x as u64).sum();
            black_box(sum);
        })
    });

    // Flushed access: We flush the cache before each iteration.
    // We use iter_custom to measure only the summation time, excluding the flush time.
    group.bench_function("sum_after_flush", |b| {
        b.iter_custom(|iters| {
            let mut total = Duration::from_secs(0);
            for _ in 0..iters {
                // Ensure data is in a known state (warm) before flushing to be consistent
                black_box(&data).iter().map(|&x| x as u64).sum::<u64>();
                
                // Flush the data cache
                clf::cache_line_flush_with_slice(&data);
                
                let start = Instant::now();
                let sum: u64 = black_box(&data).iter().map(|&x| x as u64).sum();
                total += start.elapsed();
                black_box(sum);
            }
            total
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_effectiveness);
criterion_main!(benches);
