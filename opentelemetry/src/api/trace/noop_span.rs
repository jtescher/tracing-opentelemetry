use crate::api;
use std::time::SystemTime;

pub struct NoopSpan {
    span_context: api::SpanContext,
}

impl NoopSpan {
    pub fn new() -> Self {
        NoopSpan {
            span_context: api::SpanContext::new(0, 0, 0),
        }
    }
}

impl<'a> api::Span<'a> for NoopSpan {
    type Children = std::iter::Empty<&'a u64>;
    type Follows = std::iter::Empty<&'a u64>;

    fn id(&self) -> u64 {
        unimplemented!()
    }

    fn parent(&self) -> Option<u64> {
        unimplemented!()
    }

    fn children(&'a self) -> Self::Children {
        unimplemented!()
    }

    fn follows_from(&'a self) -> Self::Follows {
        unimplemented!()
    }

    fn add_event(&mut self, _message: String) {
        // Ignore
    }
    fn add_event_with_timestamp(&mut self, _message: String, _timestamp: SystemTime) {
        // Ignored
    }

    fn get_context(&'a self) -> &'a api::SpanContext {
        &self.span_context
    }

    fn is_recording(&self) -> bool {
        false
    }

    fn set_attribute(&mut self, _attribute: crate::KeyValue) {
        // Ignored
    }

    fn end(&mut self) {
        // Ignored
    }
}
