use opentelemetry::api::{self, Provider, Span, Tracer};
use std::fmt;
use tracing_core::span::{self, Attributes, Id, Record};
use tracing_core::{field, Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

pub struct OpentelemetryLayer<T: api::Tracer> {
    tracer: T,
}

struct SpanEventVisitor<'a, S: api::Span>(&'a mut S);

impl<'a, S: api::Span> field::Visit for SpanEventVisitor<'a, S> {
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        self.0
            .add_event(format!("{} = {:?}; ", field.name(), value));
    }
}

struct SpanAttributeVisitor<'a, S: api::Span>(&'a mut S);

impl<'a, S: api::Span> field::Visit for SpanAttributeVisitor<'a, S> {
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        self.0
            .set_attribute(api::Key::new(field.name()).string(format!("{:?}", value)))
    }
}

impl<T: api::Tracer + 'static> OpentelemetryLayer<T> {
    fn parse_context<S>(
        &self,
        attrs: &Attributes<'_>,
        ctx: &Context<'_, S>,
    ) -> Option<api::SpanContext>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        if let Some(parent) = attrs.parent() {
            let span = ctx.span(parent).expect("Span not found, this is a bug");
            let extensions = span.extensions();
            extensions
                .get::<T::Span>()
                .map(|otel_span| otel_span.get_context())
        } else if attrs.is_contextual() {
            ctx.current_span()
                .id()
                .and_then(|span_id| {
                    let span = ctx.span(span_id).expect("Span not found, this is a bug");
                    let extensions = span.extensions();
                    extensions
                        .get::<T::Span>()
                        .map(|otel_span| otel_span.get_context())
                })
                .or_else(|| {
                    let ctx = opentelemetry::global::trace_provider()
                        .get_tracer("tracing-opentelemetry")
                        .get_active_span()
                        .get_context();
                    Some(ctx)
                })
        } else {
            None
        }
    }

    pub fn with_tracer(tracer: T) -> Self {
        OpentelemetryLayer { tracer }
    }
}

impl<S, T> Layer<S> for OpentelemetryLayer<T>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    T: api::Tracer + 'static,
{
    fn new_span(&self, attrs: &Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        let span_context = self.parse_context(attrs, &ctx);
        let mut span = self.tracer.start(attrs.metadata().name(), span_context);

        attrs.record(&mut SpanAttributeVisitor(&mut span));
        extensions.insert(span);
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(otel_span) = extensions.get_mut::<T::Span>() {
            values.record(&mut SpanEventVisitor(otel_span));
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Ignore events that are not in the context of a span
        if let Some(span_id) = ctx.current_span().id() {
            let span = ctx.span(span_id).expect("Span not found, this is a bug");
            let mut extensions = span.extensions_mut();
            if let Some(otel_span) = extensions.get_mut::<T::Span>() {
                event.record(&mut SpanEventVisitor(otel_span));
            }
        };
    }

    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(otel_span) = extensions.get_mut::<T::Span>() {
            otel_span.end()
        }
    }
}
