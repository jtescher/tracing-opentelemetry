pub mod api;
pub mod core;
pub mod global;
pub mod jaeger;
pub mod sdk;

pub use self::core::{Key, KeyValue, Unit};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
