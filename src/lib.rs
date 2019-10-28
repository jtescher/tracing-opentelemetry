#![deny(unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

/// Implementation of the tracing::Subscriber as a source of opentelemetry data.
pub mod subscriber;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
