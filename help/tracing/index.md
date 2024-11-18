# Tracing

We use the crate `tracing` and implementation crate `tracing-subscriber`.

We use these macros for verbosity levels:

*  `error!` 
*  `warn!`
*  `info!`
*  `debug!`
*  `trace!`,

To start tracing, create a tracing span using the level you want, then enter the span, then record events.

```rust
use tracing::{info_span};

// Create a span
let span = info_span!("my span");

// Enter the span, which returns a guard object.
// Any tracing events that occur before the guard 
// is dropped will occur within the given span.
let _enter = span.enter();

// Event with a message
info!("hello");

// Event with a few variables then a message
info!(a, b, c, "hello");

// Event with a few keys and values then a message
info!(alfa = a, bravo = b, charlie = c, "hello");
```

To add a tracing span to a function, use the `#[instrument]` attribute:

```rust
#[instrument]
pub fn foo() {
    info!("inside my_function!");
}
```

Tracing has many more capabilities, and many crate extensions. See the tracing crate documentation for more information.