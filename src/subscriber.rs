use opentelemetry::api;
use opentelemetry::api::trace::span::Span;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use tracing_core::span::{Attributes, Id, Record};
use tracing_core::{field, Event, Metadata, Subscriber};

pub struct OpentelemetrySubscriber<T: api::Tracer> {
    tracer: Arc<T>,
    spans: Mutex<HashMap<u64, Arc<Mutex<T::Span>>>>,
}

thread_local! {
    static CURRENT: RefCell<Vec<u64>> = RefCell::new(Vec::new());
}

struct SpanEventVisitor<S: api::Span>(Arc<Mutex<S>>);

impl<S: api::Span> field::Visit for SpanEventVisitor<S> {
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        let _ = self
            .0
            .try_lock()
            .map(|mut span| span.add_event(format!("{} = {:?}; ", field.name(), value)));
    }
}

struct SpanAttributeVisitor<S: api::Span>(Arc<Mutex<S>>);

impl<S: api::Span> field::Visit for SpanAttributeVisitor<S> {
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        let _ = self.0.try_lock().map(|mut span| {
            span.set_attribute(opentelemetry::Key::new(field.name()).string(format!("{:?}", value)))
        });
    }
}

impl<T: api::Tracer> OpentelemetrySubscriber<T> {
    fn parse_context(&self, attrs: &Attributes<'_>) -> Option<api::SpanContext> {
        if attrs.parent().is_some() {
            self.get_span_by_id(attrs.parent().unwrap().into_u64())
                .try_lock()
                .map(|span| span.get_context())
                .ok()
        } else if attrs.is_contextual() {
            self.get_active_span()
                .try_lock()
                .map(|span| span.get_context())
                .ok()
        } else {
            None
        }
    }

    pub fn builder() -> Builder<api::NoopTracer> {
        Builder::default()
    }

    fn get_span_by_id(&self, span_id: u64) -> Arc<Mutex<T::Span>> {
        let spans = self.spans.lock().expect("mutex poisoned!");
        spans
            .get(&span_id)
            .cloned()
            .unwrap_or_else(|| Arc::new(Mutex::new(self.tracer.invalid())))
    }

    fn get_active_span(&self) -> Arc<Mutex<T::Span>> {
        match self.current_span_id() {
            Some(id) => {
                let spans = self.spans.lock().expect("mutex poisoned!");
                spans
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| Arc::new(Mutex::new(self.tracer.invalid())))
            }
            None => Arc::new(Mutex::new(self.tracer.invalid())),
        }
    }

    fn current_span_id(&self) -> Option<u64> {
        CURRENT
            .try_with(|current| current.borrow().last().cloned())
            .ok()?
    }

    fn mark_span_as_active(&self, span_id: u64) {
        let _ = CURRENT.try_with(|current| {
            let mut current_ids = current.borrow_mut();
            current_ids.retain(|id| id != &span_id);
            current_ids.push(span_id);
        });
    }
}

/// Configures a new `OpentelemetrySubscriber`.
pub struct Builder<T: api::Tracer> {
    tracer: Arc<T>,
}

impl<T: api::Tracer> Builder<T> {
    pub fn with_tracer<B: api::Tracer>(self, tracer: Arc<B>) -> Builder<B> {
        Builder { tracer }
    }

    pub fn init(self) -> OpentelemetrySubscriber<T> {
        OpentelemetrySubscriber {
            tracer: self.tracer,
            spans: Default::default(),
        }
    }
}

impl Default for Builder<api::NoopTracer> {
    fn default() -> Self {
        Self {
            tracer: Arc::new(api::NoopTracer {}),
        }
    }
}

impl<T: api::Tracer + 'static> Subscriber for OpentelemetrySubscriber<T> {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, span_attributes: &Attributes<'_>) -> Id {
        let span_context = self.parse_context(span_attributes);
        let started = self
            .tracer
            .start(span_attributes.metadata().name().to_string(), span_context);

        let span_id = started.get_context().span_id();
        let span = Arc::new(Mutex::new(started));

        self.spans
            .lock()
            .expect("mutex poisoned!")
            .insert(span_id, span.clone());

        self.mark_span_as_active(span_id);
        span_attributes.record(&mut SpanAttributeVisitor(span));

        Id::from_u64(span_id)
    }

    fn record(&self, span_id: &Id, values: &Record<'_>) {
        let span = self.get_span_by_id(span_id.into_u64());
        values.record(&mut SpanEventVisitor(span));
    }

    fn record_follows_from(&self, _id: &Id, _follows: &Id) {
        // Ignore for now
    }

    fn event(&self, event: &Event<'_>) {
        let span = self.get_active_span();
        event.record(&mut SpanEventVisitor(span));
    }

    fn enter(&self, id: &Id) {
        self.mark_span_as_active(id.into_u64())
    }

    fn exit(&self, id: &Id) {
        let span_id = id.into_u64();
        let _ = CURRENT.try_with(|current| {
            current.borrow_mut().retain(|id| id != &span_id);
        });
        let _ = self.spans.lock().expect("mutex poisoned!").remove(&span_id);
    }
}
