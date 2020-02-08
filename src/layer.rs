use opentelemetry::api::{self, Span};
use std::fmt;
use tracing_core::span::{self, Attributes, Id, Record};
use tracing_core::{field, Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

/// OpenTelemetry layer for use in a project that uses [tracing].
///
/// [tracing]: https://github.com/tokio-rs/tracing
pub struct OpentelemetryLayer<T: api::Tracer> {
    tracer: T,
}

struct SpanEventVisitor<'a, S: api::Span>(&'a mut S);

impl<'a, S: api::Span> field::Visit for SpanEventVisitor<'a, S> {
    /// Record events on the underlying OpenTelemetry `Span`.
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        self.0
            .add_event(format!("{} = {:?}; ", field.name(), value));
    }
}

struct SpanAttributeVisitor<'a, S: api::Span>(&'a mut S);

impl<'a, S: api::Span> field::Visit for SpanAttributeVisitor<'a, S> {
    /// Set attributes on the underlying OpenTelemetry `Span`.
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        self.0
            .set_attribute(api::Key::new(field.name()).string(format!("{:?}", value)))
    }
}

impl<T: api::Tracer + 'static> OpentelemetryLayer<T> {
    /// Retrieve the parent OpenTelemetry `SpanContext` either from the current
    /// `tracing` span through the `Registry`, or from `OpenTelemetry` active
    /// span as fallback. This `SpanContext` links created spans to their parent
    /// context.
    fn parse_context<S>(
        &self,
        attrs: &Attributes<'_>,
        ctx: &Context<'_, S>,
    ) -> Option<api::SpanContext>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        // If a span is specified, it _should_ exist in the underlying `Registry`.
        if let Some(parent) = attrs.parent() {
            let span = ctx.span(parent).expect("Span not found, this is a bug");
            let extensions = span.extensions();
            extensions
                .get::<T::Span>()
                .map(|otel_span| otel_span.get_context())
        // Else if the span is inferred from context, look up any available current span.
        } else if attrs.is_contextual() {
            ctx.current_span().id().and_then(|span_id| {
                let span = ctx.span(span_id).expect("Span not found, this is a bug");
                let extensions = span.extensions();
                extensions
                    .get::<T::Span>()
                    .map(|otel_span| otel_span.get_context())
            })
        // Explicit root spans should have no parent context.
        } else {
            None
        }
    }

    /// Set the `OpenTelemetry` `Tracer` that this layer will use to produce
    /// and track `Span`s.
    ///
    /// ```rust,no_run
    /// use opentelemetry::{api::Provider, global, sdk};
    /// use tracing_opentelemetry::OpentelemetryLayer;
    ///
    /// // Create a jaeger exporter for a `trace-demo` service.
    /// let exporter = opentelemetry_jaeger::Exporter::builder()
    ///     .with_agent_endpoint("127.0.0.1:6831".parse().unwrap())
    ///     .with_process(opentelemetry_jaeger::Process {
    ///         service_name: "trace_demo".to_string(),
    ///         tags: Vec::new(),
    ///     })
    ///     .init().expect("Error initializing Jaeger exporter");
    ///
    /// // Build a provider from the jaeger exporter that always samples.
    /// let provider = sdk::Provider::builder()
    ///     .with_simple_exporter(exporter)
    ///     .with_config(sdk::Config {
    ///         default_sampler: Box::new(sdk::Sampler::Always),
    ///         ..Default::default()
    ///     })
    ///     .build();
    ///
    /// // Get a tracer from the provider for a component
    /// let tracer = provider.get_tracer("component-name");
    ///
    /// // Create a layer with the configured tracer
    /// let _layer = OpentelemetryLayer::with_tracer(tracer);
    /// ```
    pub fn with_tracer(tracer: T) -> Self {
        OpentelemetryLayer { tracer }
    }
}

impl<S, T> Layer<S> for OpentelemetryLayer<T>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    T: api::Tracer + 'static,
{
    /// Creates an `OpenTelemetry` `Span` for the corresponding `tracing` `Span`.
    /// This will attempt to parse the parent context if possible from the given attributes.
    fn new_span(&self, attrs: &Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        let span_context = self.parse_context(attrs, &ctx);
        let mut span = self.tracer.start(attrs.metadata().name(), span_context);

        attrs.record(&mut SpanAttributeVisitor(&mut span));
        extensions.insert(span);
    }

    /// Record values for the given span.
    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(otel_span) = extensions.get_mut::<T::Span>() {
            values.record(&mut SpanEventVisitor(otel_span));
        }
    }

    /// Record logs for the given event.
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

    /// Mark the `Span` as ended when it is closed.
    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(otel_span) = extensions.get_mut::<T::Span>() {
            otel_span.end()
        }
    }
}
