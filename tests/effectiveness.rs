use std::time::Instant;
use std::hint::black_box;

#[test]
#[ignore] // Ignored by default due to potential flakiness in CI environments
fn test_flush_effectiveness() {
    // 32MB - large enough to exceed most L2/L3 caches
    let size = 1024 * 1024 * 32;
    let data = vec![1u8; size];

    // Generate random indices to avoid prefetcher optimization
    // We use a simple LCG to avoid external dependencies like `rand`
    let mut indices = Vec::with_capacity(10000);
    let mut seed = 0xDEADBEEFu64;
    for _ in 0..10000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        indices.push((seed as usize) % (size - 1));
    }

    // 1. Warm-up: Access the random indices to pull them into cache
    let mut _unused_sum = 0u64;
    for &i in &indices {
        _unused_sum += data[i] as u64;
    }
    black_box(_unused_sum);

    // 2. Measure "warm" access
    let iters = 5;
    let start_warm = Instant::now();
    for _ in 0..iters {
        let mut sum = 0u64;
        for &i in &indices {
            sum += data[i] as u64;
        }
        black_box(sum);
    }
    let duration_warm = start_warm.elapsed();
    let avg_warm = duration_warm / iters;

    // 3. Flush the cache
    clf::cache_line_flush_with_slice(&data);

    // 4. Measure "flushed" access
    let start_flushed = Instant::now();
    let mut sum = 0u64;
    for &i in &indices {
        sum += data[i] as u64;
    }
    black_box(sum);
    let duration_flushed = start_flushed.elapsed();

    println!("Warm duration (avg of {}): {:?}", iters, avg_warm);
    println!("Flushed duration: {:?}", duration_flushed);

    // Assert that the flushed access is slower than a warm access.
    // Random access should show a much more significant difference.
    assert!(
        duration_flushed > avg_warm,
        "Flushed access ({:?}) should be slower than warm access ({:?})",
        duration_flushed,
        avg_warm
    );
}
