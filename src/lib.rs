#![deny(unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

/// Implementation of the trace::Layer as a source of opentelemetry data.
mod layer;

pub use layer::OpentelemetryLayer;
