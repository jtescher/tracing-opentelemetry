use crate::api;
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

pub struct JaegerTracer {
    tracer: rustracing_jaeger::Tracer,
    spans: Mutex<HashMap<u64, Arc<Mutex<rustracing_jaeger::span::Span>>>>,
}

thread_local! {
    static CURRENT: RefCell<Vec<u64>> = RefCell::new(Vec::new());
}

pub const TRACER_CONTEXT_HEADER_NAME: &str = "uber-trace-id";

impl JaegerTracer {
    pub fn new(service_name: &str) -> Self {
        let service_name = service_name.to_owned();
        let (span_tx, span_rx) = crossbeam_channel::bounded(10);
        let tracer =
            rustracing_jaeger::Tracer::with_sender(rustracing::sampler::AllSampler, span_tx);

        // Spin up thread to report finished spans
        let _ = thread::Builder::new()
            .name("jaeger span reporter".to_string())
            .spawn(move || {
                let reporter =
                    rustracing_jaeger::reporter::JaegerCompactReporter::new(&service_name)
                        .expect("can't initialize jaeger reporter");
                for span in span_rx {
                    let _ = reporter.report(&[span]);
                }
            });

        JaegerTracer {
            tracer,
            spans: Default::default(),
        }
    }

    fn current_span_id(&self) -> Option<u64> {
        CURRENT
            .try_with(|current| current.borrow().last().cloned())
            .ok()?
    }
}

impl api::Span for rustracing_jaeger::Span {
    fn add_event(&mut self, event: String) {
        self.add_event_with_timestamp(event, SystemTime::now())
    }

    fn add_event_with_timestamp(&mut self, message: String, _timestamp: SystemTime) {
        self.log(|log| {
            log.std().message(message);
        });
    }

    fn get_context(&self) -> api::SpanContext {
        match self.context() {
            Some(context) => {
                let state = context.state();
                let trace_id = u128::from_str_radix(&state.trace_id().to_string(), 16).unwrap();
                let trace_flags = if state.is_sampled() { 1 } else { 0 };

                api::SpanContext::new(trace_id, state.span_id(), trace_flags)
            }
            None => api::SpanContext::new(rand::random(), 0, 0),
        }
    }

    fn is_recording(&self) -> bool {
        true
    }

    fn set_attribute(&mut self, key: String, value: String) {
        self.set_tag(|| rustracing::tag::Tag::new(key, value))
    }

    fn end(&mut self) {
        // TODO remove from active span list to drop
        self.set_finish_time(SystemTime::now)
    }
}

impl From<api::SpanContext> for rustracing_jaeger::span::SpanContext {
    fn from(context: api::SpanContext) -> Self {
        let parent_id = 0; // TODO
        let jaeger_trace_str = format!(
            "{:x}:{:x}:{:x}:{:x}",
            context.trace_id(),
            context.span_id(),
            parent_id,
            context.trace_flags()
        );
        let span_context_state =
            rustracing_jaeger::span::SpanContextState::from_str(&jaeger_trace_str)
                .expect("should always parse");

        rustracing::span::SpanContext::new(span_context_state, Vec::new())
    }
}

impl api::Tracer for JaegerTracer {
    type Span = rustracing_jaeger::Span;

    fn start<Parent>(&self, name: String, context: Option<Parent>) -> Arc<Mutex<Self::Span>>
    where
        Parent: Into<api::SpanContext>,
    {
        let start_options = self.tracer.span(name);
        let started = match context.map(|sc| rustracing_jaeger::span::SpanContext::from(sc.into()))
        {
            Some(span_context) => start_options.child_of(&span_context).start(),
            None => start_options.start(),
        };
        let span_id = started
            .context()
            .expect("must have context")
            .state()
            .span_id();

        let span = Arc::new(Mutex::new(started));
        self.spans
            .lock()
            .expect("mutex poisoned!")
            .insert(span_id, span.clone());
        self.mark_span_as_active(span_id);

        span
    }

    fn get_active_span(&self) -> Arc<Mutex<Self::Span>> {
        match self.current_span_id() {
            Some(id) => {
                let spans = self.spans.lock().expect("mutex poisoned!");
                spans
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| Arc::new(Mutex::new(rustracing_jaeger::Span::inactive())))
            }
            None => Arc::new(Mutex::new(rustracing_jaeger::Span::inactive())),
        }
    }

    fn get_span_by_id(&self, span_id: u64) -> Arc<Mutex<Self::Span>> {
        let spans = self.spans.lock().expect("mutex poisoned!");
        spans
            .get(&span_id)
            .cloned()
            .unwrap_or_else(|| Arc::new(Mutex::new(rustracing_jaeger::Span::inactive())))
    }

    fn mark_span_as_active(&self, span_id: u64) {
        let _ = CURRENT.try_with(|current| {
            let mut current_ids = current.borrow_mut();
            current_ids.retain(|id| id != &span_id);
            current_ids.push(span_id);
        });
    }

    fn drop_span(&self, span_id: u64) {
        let _ = CURRENT.try_with(|current| {
            current.borrow_mut().retain(|id| id != &span_id);
        });
        let _ = self.spans.lock().expect("mutex poisoned!").remove(&span_id);
    }
}
