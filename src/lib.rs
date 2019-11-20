//! # Tracing OpenTelemetry
//!
//! An opentelemetry layer for the [tracing] library.
//!
//! [tracing]: https://github.com/tokio-rs/tracing
//!
//! ```rust,no_run
//! #[macro_use]
//! extern crate tracing;
//!
//! use opentelemetry::{api::Provider, sdk};
//! use tracing_opentelemetry::OpentelemetryLayer;
//! use tracing_subscriber::{Layer, Registry};
//!
//! fn main() {
//!     // Create a new tracer
//!     let tracer = sdk::Provider::default().get_tracer("service_name");
//!
//!     // Create a new tracing layer
//!     let layer = OpentelemetryLayer::with_tracer(tracer);
//!
//!     let subscriber = layer.with_subscriber(Registry::default());
//!
//!     // Trace executed code
//!     tracing::subscriber::with_default(subscriber, || {
//!         let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
//!         let _enter = root.enter();
//!
//!         error!("This event will be logged in the root span.");
//!     });
//! }
//! ```
#![deny(unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

/// Implementation of the trace::Layer as a source of OpenTelemetry data.
mod layer;

pub use layer::OpentelemetryLayer;
