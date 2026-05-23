/*!
Flush the data cache line.

This crate can be used when you do benchmarks that are not dependent on the cpu cache.

# Supports

- x86_64, aarch64 (native implementation)
- mips64el, powerpc64le ... etc (fallback to `__builtin_clear_cache`)
- minimum support rustc 1.70.0 (due to `core::arch` and `asm!`)

# Examples
Easy to use:

```rust
let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
clf::cache_line_flush_with_slice(&a);
```

or

```rust
let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
let begin_ptr = a.as_ptr();
let end_ptr = unsafe { begin_ptr.add(a.len()) };
unsafe { clf::cache_line_flush_with_ptr(begin_ptr, end_ptr) };
```

# References

[CPU cache](https://en.wikipedia.org/wiki/CPU_cache)

# Benchmarking

To measure the effectiveness of the cache flushing, you can run the included benchmarks:

```text
make bench
```

This will compare the access time of "warm" data versus "flushed" data.

*/

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_clflush;

#[link(name = "clf")]
extern "C" {
    #[allow(dead_code)]
    fn clf_fallback_clear_cache(begin_ptr: *const u8, end_ptr: *const u8);
}

///
/// get the cpu cache line size.
///
fn get_cache_line_size() -> usize {
    use core::sync::atomic::{AtomicUsize, Ordering};
    static CACHE_LINE_SIZE: AtomicUsize = AtomicUsize::new(0);

    let size = CACHE_LINE_SIZE.load(Ordering::Relaxed);
    if size != 0 {
        return size;
    }

    let detected_size = detect_cache_line_size();
    CACHE_LINE_SIZE.store(detected_size, Ordering::Relaxed);
    detected_size
}

fn detect_cache_line_size() -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        // CPUID leaf 1, EBX bits 15:8 contains the CLFLUSH line size in 8-byte increments
        #[allow(unused_unsafe)]
        let cpuid = unsafe { core::arch::x86_64::__cpuid(1) };
        let size = ((cpuid.ebx >> 8) & 0xff) as usize * 8;
        if size != 0 {
            return size;
        }
    }

    // AArch64 usually has 64-byte lines.
    // We avoid using `mrs ctr_el0` as it's restricted on some platforms (like macOS).

    // Default value (most common on modern CPUs)
    64
}

///
/// flush the data cache line, this parameters are pointers.
///
/// # Safety
/// This function is unsafe because it dereferences raw pointers and
/// performs low-level CPU cache operations.
pub unsafe fn cache_line_flush_with_ptr(begin_ptr: *const u8, end_ptr: *const u8) {
    #[cfg(target_arch = "x86_64")]
    {
        let size = get_cache_line_size();
        let mut ptr = begin_ptr;
        while ptr < end_ptr {
            _mm_clflush(ptr);
            ptr = ptr.add(size);
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        let size = get_cache_line_size();
        let mut ptr = begin_ptr as usize;
        let end = end_ptr as usize;
        while ptr < end {
            core::arch::asm!("dc civac, {0}", in(reg) ptr, options(nostack, preserves_flags));
            ptr += size;
        }
        core::arch::asm!("dsb ish", options(nostack, preserves_flags));
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        clf_fallback_clear_cache(begin_ptr, end_ptr);
    }
}

///
/// flush the data cache line, this parameter is a slice.
///
pub fn cache_line_flush_with_slice<T>(slice: &[T]) {
    let begin_ptr = slice.as_ptr() as *const u8;
    let end_ptr = unsafe { begin_ptr.add(core::mem::size_of_val(slice)) };
    unsafe { cache_line_flush_with_ptr(begin_ptr, end_ptr) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_1() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe {
            super::cache_line_flush_with_ptr(a.as_ptr(), a.as_ptr().add(a.len()));
        }
    }
    #[test]
    fn it_works_2() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        super::cache_line_flush_with_slice(&a);
    }
    #[test]
    fn large_slice() {
        let a = vec![0u8; 1024 * 1024]; // 1MB
        super::cache_line_flush_with_slice(&a);
    }
}
