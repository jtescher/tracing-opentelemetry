# Tracing Opentelemetry

An opentelemetry subscriber for the [tracing] library.

[tracing]: https://github.com/tokio-rs/tracing

## Tracers

Currently supports the Jaeger tracer via [rustracing_jaeger]

[rustracing_jaeger]: https://github.com/sile/rustracing_jaeger

Examples
--------

### Basic Usage

```rust
#[macro_use]
extern crate tracing;

use opentelemetry::sdk;
use std::sync::Arc;
use tracing_opentelemetry::subscriber::OpentelemetrySubscriber;

// Create a new tracer
let tracer = Arc::new(sdk::Tracer::new("service_name"));

// Create a new tracing subscriber
let subscriber = OpentelemetrySubscriber::<sdk::Tracer>::builder()
    .with_tracer(tracer)
    .init();

// Trace executed code
tracing::subscriber::with_default(subscriber, || {
    let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    let _enter = root.enter();

    error!("This event will be logged in the root span.");
});
```

### Executes `report.rs` example

```console
# Run jaeger in background
$ docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest

# Report example spans
$ cargo run --example report

# View spans (see the image below)
$ firefox http://localhost:16686/
```

![Jaeger UI](trace.png)

References
----------

- [OpenTelemetry](https://opentelemetry.io/)