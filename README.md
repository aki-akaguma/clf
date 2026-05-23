# clf

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

Flush the data cache line.

This crate can be used when you do benchmarks that are not dependent on the cpu cache.

## Supports

- x86_64, aarch64 (native implementation)
- mips64el, powerpc64le ... etc (fallback to `__builtin_clear_cache`)
- minimum support rustc 1.70.0 (due to `core::arch` and `asm!`)

## Examples
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

## References

[CPU cache](https://en.wikipedia.org/wiki/CPU_cache)

## Benchmarking

To measure the effectiveness of the cache flushing, you can run the included benchmarks:

```
make bench
```

This will compare the access time of "warm" data versus "flushed" data.


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/clf/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/clf.svg
[crate-link]: https://crates.io/crates/clf
[docs-image]: https://docs.rs/clf/badge.svg
[docs-link]: https://docs.rs/clf/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/clf/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/clf/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/clf/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/clf/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/clf/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/clf/actions/workflows/test-windows.yml
