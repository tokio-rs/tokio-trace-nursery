# tracing

Application-level tracing for Rust.

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![Documentation (master)][docs-master-badge]][docs-master-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Discord chat][discord-badge]][discord-url]

[crates-badge]: https://img.shields.io/crates/v/tracing.svg
[crates-url]: https://crates.io/crates/tracing
[docs-badge]: https://docs.rs/tracing/badge.svg
[docs-url]: https://docs.rs/tracing
[docs-master-badge]: https://img.shields.io/badge/docs-master-blue
[docs-master-url]: https://tracing-rs.netlify.com
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[actions-badge]: https://github.com/tokio-rs/tracing/workflows/CI/badge.svg
[actions-url]:https://github.com/tokio-rs/tracing/actions?query=workflow%3ACI
[discord-badge]: https://img.shields.io/discord/500028886025895936?logo=discord&label=discord&logoColor=white
[discord-url]: https://discord.gg/EeF3cQw

[Website](https://tokio.rs) |
[Chat](https://discord.gg/EeF3cQw) | [Documentation (master branch)](https://tracing-rs.netlify.com/)

## Overview

`tracing` is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information. `tracing` is maintained by the
Tokio project, but does _not_ require the `tokio` runtime to be used.

## Usage

(The examples below are borrowed from the `log` crate's yak-shaving
[example](https://docs.rs/log/0.4.10/log/index.html#examples), modified to
idiomatic `tracing`.)

### In Applications

In order to record trace events, executables have to use a `Subscriber`
implementation compatible with `tracing`. A `Subscriber` implements a way of
collecting trace data, such as by logging it to standard output. [`tracing_subscriber`](https://docs.rs/tracing-subscriber/)'s
[`fmt` module](https://docs.rs/tracing-subscriber/0.2.0-alpha.2/tracing_subscriber/fmt/index.html) provides reasonable defaults.
Additionally, `tracing-subscriber` is able to consume messages emitted by `log`-instrumented libraries and modules.

The simplest way to use a subscriber is to call the `set_global_default` function.

```rust
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting defualt subscriber failed");

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = yak_shave::shave_all(number_of_yaks);
    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
}
```

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.2.0-alpha.4"
```

This subscriber will be used as the default in all threads for the remainder of the duration
of the program, similar to how loggers work in the `log` crate.

In addition, you can locally override the default subscriber. For example:

```rust
use tracing::{info, Level};
use tracing_subscruber::FmtSubscriber;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        info!("This will be logged to stdout");
    });
    info!("This will _not_ be logged to stdout");
}
```

Any trace events generated outside the context of a subscriber will not be collected.

This approach allows trace data to be collected by multiple subscribers
within different contexts in the program. Note that the override only applies to the
currently executing thread; other threads will not see the change from with_default.

Once a subscriber has been set, instrumentation points may be added to the
executable using the `tracing` crate's macros.

### In Libraries

Libraries should only rely on the `tracing` crate and use the provided macros
and types to collect whatever information might be useful to downstream consumers.

```rust
use std::{error::Error, io};
use tracing::{debug, error, info, span, warn, Level};

// the `#[tracing::instrument]` attribute creates and enters a span
// every time the instrumented function is called. The span is named after the
// the function or method. Paramaters passed to the function are recorded as fields.
#[tracing::instrument]
pub fn shave(yak: usize) -> Result<(), Box<dyn Error + 'static>> {
    // this creates an event at the DEBUG level with two fields:
    // - `excitement`, with the key "excitement" and the value "yay!"
    // - `message`, with the key "message" and the value "hello! I'm gonna shave a yak."
    //
    // unlike other fields, `message`'s shorthand initialization is just the string itself.
    debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");
    if yak == 3 {
        warn!("could not locate yak!");
        // note that this is intended to demonstrate `tracing`'s features, not idiomatic
        // error handling! in a library or application, you should consider returning
        // a dedicated `YakError`. libraries like snafu or thiserror make this easy.
        return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
    } else {
        debug!("yak shaved successfully");
    }
    Ok(())
}

pub fn shave_all(yaks: usize) -> usize {
    // Constructs a new span named "shaving_yaks" at the TRACE level,
    // and a field whose key is "yaks". This is equivalent to writing:
    //
    // let span = span!(Level::TRACE, "shaving_yaks", yaks = yaks);
    //
    // local variables (`yaks`) can be used as field values
    // without an assignment, similar to struct initializers.
    let span = span!(Level::TRACE, "shaving_yaks", yaks);
    let _enter = span.enter();

    info!("shaving yaks");

    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        let res = shave(yak);
        debug!(yak, shaved = res.is_ok());

        if let Err(ref error) = res {
            // Like spans, events can also use the field initialization shorthand.
            // In this instance, `yak` is the field being initalized.
            error!(yak, error = error.as_ref(), "failed to shave yak!");
        } else {
            yaks_shaved += 1;
        }
        debug!(yaks_shaved);
    }

    yaks_shaved
}
```

```toml
[dependencies]
tracing = "0.1"
```

Note: Libraries should *NOT* call `set_global_default()`, as this will cause
conflicts when executables try to set the default later.

### In Asynchronous Code

If you are instrumenting code that make use of
[`std::future::Future`](https://doc.rust-lang.org/stable/std/future/trait.Future.html)
or async/await, be sure to use the
[`tracing-futures`](https://docs.rs/tracing-futures) crate. This is needed
because the following example _will not_ work:

```rust
async {
    let _s = span.enter();
    // ...
}
```

The span guard `_s` will not exit until the future generated by the `async` block is complete.
Since futures and spans can be entered and exited _multiple_ times without them completing,
the span remains entered for as long as the future exists, rather than being entered only when
it is polled, leading to very confusing and incorrect output.
For more details, see [the documentation on closing spans](https://tracing.rs/tracing/span/index.html#closing-spans).

There are two ways to instrument asynchronous code. The first is through the
[`Future::instrument`](https://docs.rs/tracing-futures/0.2.1/tracing_futures/trait.Instrument.html#method.instrument) combinator:

```rust
use tracing_futures::Instrument;

let my_future = async {
    // ...
};

my_future
    .instrument(tracing::info_span!("my_future"))
    .await
```

`Future::instrument` attaches a span to the future, ensuring that the span's lifetime
is as long as the future's.

The second, and preferred, option is through the
[`#[instrument]`](https://docs.rs/tracing/0.1.11/tracing/attr.instrument.html)
attribute:

```rust
use tracing::{info, instrument};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use std::io;

#[instrument]
async fn write(stream: &mut TcpStream) -> io::Result<usize> {
    let result = stream.write(b"hello world\n").await;
    info!("wrote to stream; success={:?}", result.is_ok());
    result
}
```

Under the hood, the `#[instrument]` macro performs same the explicit span
attachment that `Future::instrument` does.

Note: the [`#[tracing::instrument]`](https://github.com/tokio-rs/tracing/issues/399)` macro does not work correctly with the [async-trait](https://github.com/dtolnay/async-trait) crate. This bug is tracked in [#399](https://github.com/tokio-rs/tracing/issues/399).

## Getting Help

First, see if the answer to your question can be found in the API documentation.
If the answer is not there, there is an active community in
the [Tracing Discord channel][chat]. We would be happy to try to answer your
question.  Last, if that doesn't work, try opening an [issue] with the question.

[chat]: https://discord.gg/EeF3cQw
[issue]: https://github.com/tokio-rs/tracing/issues/new

## Contributing

:balloon: Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][guide] to help you get involved in the Tracing
project.

[guide]: CONTRIBUTING.md

## Project layout

The [`tracing`] crate contains the primary _instrumentation_ API, used for
instrumenting libraries and applications to emit trace data. The [`tracing-core`]
crate contains the _core_ API primitives on which the rest of `tracing` is
instrumented. Authors of trace subscribers may depend on `tracing-core`, which
guarantees a higher level of stability.

Additionally, this repository contains several compatibility and utility
libraries built on top of `tracing`. Some of these crates are in a pre-release
state, and are less stable than the `tracing` and `tracing-core` crates.

The crates included as part of Tracing are:

* [`tracing-futures`]: Utilities for instrumenting `futures`.
  ([crates.io][fut-crates]|[docs][fut-docs])

* [`tracing-macros`]: Experimental macros for emitting trace events (unstable).

* [`tracing-attributes`]: Procedural macro attributes for automatically
    instrumenting functions. ([crates.io][attr-crates]|[docs][attr-docs])

* [`tracing-log`]: Compatibility with the `log` crate (unstable).

* [`tracing-serde`]: A compatibility layer for serializing trace data with
    `serde` (unstable).

* [`tracing-subscriber`]: Subscriber implementations, and utilities for
  implementing and composing `Subscriber`s.
  ([crates.io][sub-crates]|[docs][sub-docs])

* [`tracing-tower`]: Compatibility with the `tower` ecosystem (unstable).

[`tracing`]: tracing
[`tracing-core`]: tracing
[`tracing-futures`]: tracing-futures
[`tracing-macros`]: tracing-macros
[`tracing-attributes`]: tracing-attributes
[`tracing-log`]: tracing-log
[`tracing-serde`]: tracing-serde
[`tracing-subscriber`]: tracing-subscriber
[`tracing-tower`]: tracing-tower

[fut-crates]: https://crates.io/crates/tracing-futures
[fut-docs]: https://docs.rs/tracing-futures

[attr-crates]: https://crates.io/crates/tracing-attributes
[attr-docs]: https://docs.rs/tracing-attributes

[sub-crates]: https://crates.io/crates/tracing-subscriber
[sub-docs]: https://docs.rs/tracing-subscriber

## Related Crates

In addition to this repository, here are also several third-party crates which
are not maintained by the `tokio` project. These include:

- [`tracing-timing`] implements inter-event timing metrics on top of `tracing`.
  It provides a subscriber that records the time elapsed between pairs of
  `tracing` events and generates histograms.
- [`tracing-opentelemetry`] provides a subscriber for emitting traces to
  [OpenTelemetry]-compatible distributed tracing systems.
- [`tracing-honeycomb`] implements a subscriber for reporting traces to
  [honeycomb.io].
- [`tracing-actix`] provides `tracing` integration for the `actix` actor
  framework.
- [`tracing-gelf`] implements a subscriber for exporting traces in Greylog
  GELF format.
- [`tracing-coz`] provides integration with the [coz] causal profiler
  (Linux-only).

(if you're the maintainer of a `tracing` ecosystem crate not in this list,
please let us know!)

[`tracing-timing`]: https://crates.io/crates/tracing-timing
[`tracing-opentelemetry`]: https://crates.io/crates/tracing-opentelemetry
[OpenTelemetry]: https://opentelemetry.io/
[`tracing-honeycomb`]: https://crates.io/crates/honeycomb-tracing
[honeycomb.io]: https://www.honeycomb.io/
[`tracing-actix`]: https://crates.io/crates/tracing-actix
[`tracing-gelf`]: https://crates.io/crates/tracing-gelf
[`tracing-coz`]: https://crates.io/crates/tracing-coz
[coz]: https://github.com/plasma-umass/coz

**Note:** that some of the ecosystem crates are currently unreleased and
undergoing active development. They may be less stable than `tracing` and
`tracing-core`.

## External Resources

This is a list of links to blog posts, conference talks, and tutorials about
Tracing.

#### Blog Posts

* [Diagnostics with Tracing][tokio-blog-2019-08] on the Tokio blog, August 2019

[tokio-blog-2019-08]: https://tokio.rs/blog/2019-08-tracing/

#### Talks

* [Bay Area Rust Meetup talk and Q&A][bay-rust-2018-03], March 2018
* [RustConf 2019 talk][rust-conf-2019-08-video] and [slides][rust-conf-2019-08-slides], August 2019

[bay-rust-2018-03]: https://www.youtube.com/watch?v=j_kXRg3zlec
[rust-conf-2019-08-video]: https://www.youtube.com/watch?v=JjItsfqFIdo
[rust-conf-2019-08-slides]: https://www.elizas.website/slides/rustconf-8-2019.pdf

Help us expand this list! If you've written or spoken about Tracing, or
know of resources that aren't listed, please open a pull request adding them.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tracing by you, shall be licensed as MIT, without any additional
terms or conditions.
