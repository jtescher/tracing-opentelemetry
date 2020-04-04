use opentelemetry::api;
use std::any::TypeId;
use std::fmt;
use std::marker;
use std::time::SystemTime;
use tracing_core::span::{self, Attributes, Id, Record};
use tracing_core::{field, Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

/// OpenTelemetry layer for use in a project that uses [tracing].
///
/// [tracing]: https://github.com/tokio-rs/tracing
pub struct OpenTelemetryLayer<S, T: api::Tracer> {
    tracer: T,

    get_context: WithContext,
    _registry: marker::PhantomData<S>,
}

// this function "remembers" the types of the subscriber so that we
// can downcast to something aware of them without knowing those
// types at the callsite.
//
// See https://github.com/tokio-rs/tracing/blob/4dad420ee1d4607bad79270c1520673fa6266a3d/tracing-error/src/layer.rs
pub(crate) struct WithContext(
    fn(&tracing::Dispatch, &span::Id, f: &mut dyn FnMut(&mut api::SpanBuilder)),
);

impl WithContext {
    // This function allows a function to be called in the context of the
    // "remembered" subscriber.
    pub(crate) fn with_context<'a>(
        &self,
        dispatch: &'a tracing::Dispatch,
        id: &span::Id,
        mut f: impl FnMut(&mut api::SpanBuilder),
    ) {
        (self.0)(dispatch, id, &mut f)
    }
}

pub(crate) fn build_context(builder: &mut api::SpanBuilder) -> api::SpanContext {
    let span_id = builder.span_id.expect("Builders must have id");
    let (trace_id, trace_flags) = builder
        .parent_context
        .as_ref()
        .map(|parent_context| (parent_context.trace_id(), parent_context.trace_flags()))
        .unwrap_or((
            builder.trace_id.expect("trace_id should exist"),
            api::TRACE_FLAG_SAMPLED,
        ));

    api::SpanContext::new(trace_id, span_id, trace_flags, false)
}

struct SpanEventVisitor<'a>(&'a mut api::Event);

impl<'a> field::Visit for SpanEventVisitor<'a> {
    /// Record events on the underlying OpenTelemetry `Span`.
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.0.name = format!("{:?}", value);
        } else {
            self.0
                .attributes
                .push(api::Key::new(field.name()).string(format!("{:?}", value)));
        }
    }
}

struct SpanAttributeVisitor<'a>(&'a mut api::SpanBuilder);

impl<'a> field::Visit for SpanAttributeVisitor<'a> {
    /// Set attributes on the underlying OpenTelemetry `Span`.
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        let attribute = api::Key::new(field.name()).string(format!("{:?}", value));
        if let Some(attributes) = &mut self.0.attributes {
            attributes.push(attribute);
        } else {
            self.0.attributes = Some(vec![attribute]);
        }
    }
}

impl<S, T> OpenTelemetryLayer<S, T>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    T: api::Tracer + 'static,
{
    /// Retrieve the parent OpenTelemetry [`SpanContext`] from the current
    /// tracing [`span`] through the [`Registry`]. This [`SpanContext`]
    /// links spans to their parent for proper hierarchical visualization.
    fn parent_context(
        &self,
        attrs: &Attributes<'_>,
        ctx: &Context<'_, S>,
    ) -> Option<api::SpanContext> {
        // If a span is specified, it _should_ exist in the underlying `Registry`.
        if let Some(parent) = attrs.parent() {
            let span = ctx.span(parent).expect("Span not found, this is a bug");
            let mut extensions = span.extensions_mut();
            extensions.get_mut::<api::SpanBuilder>().map(build_context)
        // Else if the span is inferred from context, look up any available current span.
        } else if attrs.is_contextual() {
            ctx.current_span().id().and_then(|span_id| {
                let span = ctx.span(span_id).expect("Span not found, this is a bug");
                let mut extensions = span.extensions_mut();
                extensions.get_mut::<api::SpanBuilder>().map(build_context)
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
    /// use tracing_opentelemetry::OpenTelemetryLayer;
    /// use tracing_subscriber::{Layer, Registry};
    /// use tracing_subscriber::layer::SubscriberExt;
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
    /// let otel_layer = OpenTelemetryLayer::with_tracer(tracer);
    ///
    /// // Use the tracing subscriber `Registry`, or any other subscriber
    /// // that impls `LookupSpan`
    /// let _subscriber = Registry::default()
    ///     .with(otel_layer);
    /// ```
    pub fn with_tracer(tracer: T) -> Self {
        OpenTelemetryLayer {
            tracer,
            get_context: WithContext(Self::get_context),
            _registry: marker::PhantomData,
        }
    }

    fn get_context(
        dispatch: &tracing::Dispatch,
        id: &span::Id,
        f: &mut dyn FnMut(&mut api::SpanBuilder),
    ) {
        let subscriber = dispatch
            .downcast_ref::<S>()
            .expect("subscriber should downcast to expected type; this is a bug!");
        let span = subscriber
            .span(id)
            .expect("registry should have a span for the current ID");

        let mut extensions = span.extensions_mut();
        if let Some(builder) = extensions.get_mut::<api::SpanBuilder>() {
            f(builder);
        }
    }
}

impl<S, T> Layer<S> for OpenTelemetryLayer<S, T>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    T: api::Tracer + 'static,
{
    /// Creates an `OpenTelemetry` `Span` for the corresponding `tracing` `Span`.
    /// This will attempt to parse the parent context if possible from the given attributes.
    fn new_span(&self, attrs: &Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        let mut builder = self
            .tracer
            .span_builder(attrs.metadata().name())
            .with_start_time(SystemTime::now())
            // Eagerly assign span id so children have stable parent id
            .with_span_id(api::SpanId::from_u64(rand::random()));
        builder.parent_context = self.parent_context(attrs, &ctx);

        // Ensure trace id exists so children are matched properly.
        if builder.parent_context.is_none() {
            builder.trace_id = Some(api::TraceId::from_u128(rand::random()));
        }

        attrs.record(&mut SpanAttributeVisitor(&mut builder));
        extensions.insert(builder);
    }

    /// Record values for the given span.
    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(builder) = extensions.get_mut::<api::SpanBuilder>() {
            values.record(&mut SpanAttributeVisitor(builder));
        }
    }

    /// Record logs for the given event.
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Ignore events that are not in the context of a span
        if let Some(span_id) = ctx.current_span().id() {
            let span = ctx.span(span_id).expect("Span not found, this is a bug");
            let mut extensions = span.extensions_mut();
            if let Some(builder) = extensions.get_mut::<api::SpanBuilder>() {
                let mut otel_event = api::Event::new(
                    String::new(),
                    SystemTime::now(),
                    vec![
                        api::Key::new("level").string(event.metadata().level().to_string()),
                        api::Key::new("target").string(event.metadata().target()),
                    ],
                );

                event.record(&mut SpanEventVisitor(&mut otel_event));

                if let Some(ref mut events) = builder.message_events {
                    events.push(otel_event);
                } else {
                    builder.message_events = Some(vec![otel_event]);
                }
            }
        };
    }

    /// Mark the `Span` as ended when it is closed.
    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(builder) = extensions.remove::<api::SpanBuilder>() {
            // Assign end time, build and start span, drop span to export
            builder.with_end_time(SystemTime::now()).start(&self.tracer);
        }
    }

    // SAFETY: this is safe because the `WithContext` function pointer is valid
    // for the lifetime of `&self`.
    unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()> {
        match id {
            id if id == TypeId::of::<Self>() => Some(self as *const _ as *const ()),
            id if id == TypeId::of::<WithContext>() => {
                Some(&self.get_context as *const _ as *const ())
            }
            _ => None,
        }
    }
}
