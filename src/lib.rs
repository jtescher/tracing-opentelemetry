#![deny(unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

/// Implementation of the trace::Subscriber as a source of opentelemetry data.
pub mod subscriber;
