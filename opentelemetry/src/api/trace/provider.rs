use crate::api;

pub trait Provider<'span> {
    type Tracer: api::Tracer<'span>;
    fn get_tracer<S: Into<String>>(&self, name: S) -> Self::Tracer;
}
