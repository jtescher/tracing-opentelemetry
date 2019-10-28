pub mod api;
pub mod jaeger;
pub mod sdk;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
