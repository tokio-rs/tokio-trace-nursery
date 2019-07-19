# tracing-fmt

**Warning: Until `tracing-fmt` has a 0.1.0 release on crates.io, please treat every release as potentially breaking.**

A subscriber for [tracing], a collection of libraries designed for application-level tracing for Rust. tracing-fmt tastefully formats and colors tracing's [spans]/[events] before chucking them to the wastes of `stdout`.

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][travis-badge]][travis-url]
[![Gitter chat][gitter-badge]][gitter-url]

[Documentation][docs-url] |
[Chat][gitter-url]

[tracing]: https://github.com/tokio-rs/tracing-fmt
[crates-badge]: https://img.shields.io/crates/v/tracing-fmt.svg
[crates-url]: https://crates.io/crates/tracing-fmt
[docs-badge]: https://docs.rs/tracing-fmt/badge.svg
[docs-url]: https://docs.rs/tracing-fmt
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[travis-badge]: https://travis-ci.org/tokio-rs/tracing.svg?branch=master
[travis-url]: https://travis-ci.org/tokio-rs/tracing/branches
[gitter-badge]: https://img.shields.io/gitter/room/tokio-rs/tracing.svg
[gitter-url]: https://gitter.im/tokio-rs/tracing
[spans]: https://docs.rs/tracing/0.1.3/tracing/span/index.html
[events]: https://docs.rs/tracing/0.1.3/tracing/struct.Event.html

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tracing by you, shall be licensed as MIT, without any additional
terms or conditions.
