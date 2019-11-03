use crate::api;
use crate::api::NoopTracer;

pub struct NoopProvider {}

impl<'a> api::Provider<'a> for NoopProvider {
    type Tracer = NoopTracer;

    fn get_tracer<S: Into<String>>(&self, _name: S) -> Self::Tracer {
        NoopTracer {}
    }
}
