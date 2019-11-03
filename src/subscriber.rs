//use opentelemetry::api;
//use opentelemetry::api::trace::span::Span;
//use opentelemetry::api::NoopTracer;
//use std::fmt;
//use tracing_core::span::{Attributes, Id, Record};
//use tracing_core::{field, Event, Metadata, Subscriber};
//
//pub struct OpentelemetrySubscriber<T: api::Tracer<'_>> {
//    tracer: T,
//}
//
//struct SpanEventVisitor<'a, S: api::Span>(&'a mut S);
//
//impl<'a, S: api::Span> field::Visit for SpanEventVisitor<'a, S> {
//    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
//        self.add_event(format!("{} = {:?}; ", field.name(), value));
//    }
//}
//
//struct SpanAttributeVisitor<'a, S: api::Span>(&'a mut S);
//
//impl<S: api::Span> field::Visit for SpanAttributeVisitor<S> {
//    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
//        self.set_attribute(field.name().to_string(), format!("{:?}", value));
//    }
//}
//
//impl<T: api::Tracer + 'static> OpentelemetrySubscriber<T> {
//    fn parse_context(&self, attrs: &Attributes<'_>) -> Option<api::SpanContext> {
//        if attrs.parent().is_some() {
//            self.tracer
//                .get_span_by_id(attrs.parent().unwrap().into_u64())
//                .try_lock()
//                .map(|span| span.get_context())
//                .ok()
//        } else if attrs.is_contextual() {
//            self.tracer
//                .get_active_span()
//                .try_lock()
//                .map(|span| span.get_context())
//                .ok()
//        } else {
//            None
//        }
//    }
//
//    pub fn builder() -> Builder<NoopTracer> {
//        Builder::default()
//    }
//}
//
///// Configures a new `OpentelemetrySubscriber`.
//#[derive(Debug)]
//pub struct Builder<T: api::Tracer> {
//    tracer: T,
//}
//
//impl<T: api::Tracer> Builder<T> {
//    pub fn with_tracer<B: api::Tracer>(self, tracer: B) -> Builder<B> {
//        Builder { tracer }
//    }
//
//    pub fn init(self) -> OpentelemetrySubscriber<T> {
//        OpentelemetrySubscriber {
//            tracer: self.tracer,
//        }
//    }
//}
//
//impl Default for Builder<NoopTracer> {
//    fn default() -> Self {
//        Self {
//            tracer: api::NoopTracer {},
//        }
//    }
//}
//
//impl<T: api::Tracer + 'static> Subscriber for OpentelemetrySubscriber<T> {
//    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
//        true
//    }
//
//    fn new_span(&self, span_attributes: &Attributes<'_>) -> Id {
//        let span_context = self.parse_context(span_attributes);
//        let span = self
//            .tracer
//            .start(span_attributes.metadata().name().to_string(), span_context);
//
//        span_attributes.record(&mut SpanAttributeVisitor(span.clone()));
//
//        Id::from_u64(
//            span.lock()
//                .map(|span| span.get_context().span_id())
//                .unwrap(),
//        )
//    }
//
//    fn record(&self, span_id: &Id, values: &Record<'_>) {
//        let span = self.tracer.get_span_by_id(span_id.into_u64());
//        values.record(&mut SpanEventVisitor(span));
//    }
//
//    fn record_follows_from(&self, _id: &Id, _follows: &Id) {
//        // Ignore for now
//    }
//
//    fn event(&self, event: &Event<'_>) {
//        let span = self.tracer.get_active_span();
//        event.record(&mut SpanEventVisitor(span));
//    }
//
//    fn enter(&self, id: &Id) {
//        self.tracer.mark_span_as_active(id.into_u64())
//    }
//
//    fn exit(&self, id: &Id) {
//        let span = self.tracer.get_span_by_id(id.into_u64());
//        let _ = span.try_lock().map(|mut span| span.end());
//        // TODO auto drop span when ended
//        self.tracer.drop_span(ids);
//    }
//}
