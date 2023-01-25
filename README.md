# clf

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test][test-image]][test-link]

Flush the cpu cache line by `__builtin_clear_cache()`

This crate can be used when you do benchmarks that are not dependent on the cpu cache.

## Supports

- gcc and clang
- gnu and musl
- x86_64, aarch64, mips64el, powerpc64le ... etc
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Bugs

- armv7-unknown-linux-musleabihf: can not compile
- x86_64-pc-windows-msvc: can not compile

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
[test-image]: https://github.com/aki-akaguma/clf/actions/workflows/test.yml/badge.svg
[test-link]: https://github.com/aki-akaguma/clf/actions/workflows/test.yml
