//!
//! Flush the cpu cache line by `__builtin_clear_cache()`
//!
//! This crate can be used when you do benchmarks that are not dependent on the cpu cache.
//!
//! # Examples
//! Easy to use:
//! 
//! ```text
//! let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
//! clf::cache_line_flush_with_slice(&a);
//! ```
//! 
//! or
//! 
//! ```text
//! let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
//! let begin_ptr = a.as_ptr();
//! let end_ptr = unsafe { begin_ptr.add(a.len()) };
//! clf::cache_line_flush_with_ptr(begin_ptr, end_ptr);
//! ```
//!

#[link(name = "clf")]
extern "C" {
    fn _cache_line_flush(begin_ptr: *const u8, end_ptr: *const u8);
}

///
/// flush the cpu cache line, this parameters are pointers.
///
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn cache_line_flush_with_ptr(begin_ptr: *const u8, end_ptr: *const u8) {
    unsafe { _cache_line_flush(begin_ptr, end_ptr) };
}

///
/// flush the cpu cache line, this parameter is a slice.
///
pub fn cache_line_flush_with_slice<T>(slice: &[T]) {
    let begin_ptr = slice.as_ptr();
    let end_ptr = unsafe { begin_ptr.add(slice.len()) };
    unsafe { _cache_line_flush(begin_ptr as *const u8, end_ptr as *const u8) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_1() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        super::cache_line_flush_with_ptr(a.as_ptr(), unsafe { a.as_ptr().add(a.len()) });
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_works_2() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        super::cache_line_flush_with_slice(&a);
        assert_eq!(2 + 2, 4);
    }
}
