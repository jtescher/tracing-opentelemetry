use crate::api;
use std::sync::{Arc, Mutex};

pub struct NoopTracer {}

impl<'a> api::Tracer<'a> for NoopTracer {
    type Span = api::NoopSpan;
    fn start<ParentSpan>(
        &self,
        _name: String,
        _context: Option<ParentSpan>,
    ) -> Arc<Mutex<Self::Span>>
    where
        ParentSpan: Into<api::SpanContext>,
    {
        Arc::new(Mutex::new(api::NoopSpan::new()))
    }

    fn get_active_span(&self) -> Arc<Mutex<Self::Span>> {
        Arc::new(Mutex::new(api::NoopSpan::new()))
    }

    fn get_span_by_id(&self, _span_id: u64) -> Arc<Mutex<Self::Span>> {
        Arc::new(Mutex::new(api::NoopSpan::new()))
    }

    fn mark_span_as_active(&self, _span_id: u64) {
        // Noop
    }

    fn drop_span(&self, _span_id: u64) {
        // Noop
    }
}
