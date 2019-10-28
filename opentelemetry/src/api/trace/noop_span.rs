use crate::api;
use std::time::SystemTime;

pub struct NoopSpan {}

impl api::Span for NoopSpan {
    fn add_event(&mut self, _message: String) {
        // Ignore
    }
    fn add_event_with_timestamp(&mut self, _message: String, _timestamp: SystemTime) {
        // Ignored
    }

    fn get_context(&self) -> api::SpanContext {
        api::SpanContext::new(0, 0, 0)
    }

    fn is_recording(&self) -> bool {
        false
    }

    fn set_attribute(&mut self, _key: String, _value: String) {
        // Ignored
    }

    fn end(&mut self) {
        // Ignored
    }
}
