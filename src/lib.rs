//! # Tracing OpenTelemetry
//!
//! An OpenTelemetry layer for the [tracing] library.
//!
//! [tracing]: https://github.com/tokio-rs/tracing
//!
//! ```rust,no_run
//! #[macro_use]
//! extern crate tracing;
//!
//! use opentelemetry::{api::Provider, sdk};
//! use tracing_opentelemetry::OpenTelemetryLayer;
//! use tracing_subscriber::{Layer, Registry};
//! use tracing_subscriber::layer::SubscriberExt;
//!
//! fn main() {
//!     // Create a new tracer
//!     let tracer = sdk::Provider::default().get_tracer("service_name");
//!
//!     // Create a new OpenTelemetry tracing layer
//!     let telemetry = OpenTelemetryLayer::with_tracer(tracer);
//!
//!     let subscriber = Registry::default().with(telemetry);
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
/// Span extension which enables OpenTelemetry span context management.
mod span_ext;

pub use layer::OpenTelemetryLayer;
pub use span_ext::OpenTelemetrySpanExt;
