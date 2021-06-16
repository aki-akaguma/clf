# clf
Flush the cpu cache line by `__builtin_clear_cache()`

This crate can be used when you do benchmarks that are not dependent on the cpu cache.

## Supports

+ gcc and clang
+ gnu and musl
+ x86_64, aarch64, mips64el, powerpc64le ... etc

## Bugs

+ armv7-unknown-linux-musleabihf: can not compile

## Examples

Easy to use:

```
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    clf::cache_line_flush_with_slice(&a);
```

or

```
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let begin_ptr = a.as_ptr();
    let end_ptr = unsafe { begin_ptr.add(a.len()) };
    clf::cache_line_flush_with_ptr(begin_ptr, end_ptr);
```

## References

[CPU cache](https://en.wikipedia.org/wiki/CPU_cache)
