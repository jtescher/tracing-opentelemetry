use crate::layer::{build_context, WithContext};
use opentelemetry::api;

/// `OpenTelemetrySpanExt` allows tracing spans to accept and return
/// OpenTelemetry `SpanContext`s.
pub trait OpenTelemetrySpanExt {
    /// Associates `self` with a given `OpenTelemetry` trace, using
    /// the provided parent context.
    ///
    /// ```rust
    /// use opentelemetry::api::{self, HttpTextFormat};
    /// use tracing_opentelemetry::OpenTelemetrySpanExt;
    /// use std::collections::HashMap;
    ///
    /// // Example carrier, could be a framework header map that impls `api::Carrier`.
    /// let mut carrier = HashMap::new();
    ///
    /// // Propagator can be swapped with trace context propagator binary propagator, etc.
    /// let propagator = api::B3Propagator::new(true);
    ///
    /// // Extract otel parent span context via the chosen propagator
    /// let parent_context = propagator.extract(&carrier);
    ///
    /// // Generate a tracing span as usual
    /// let app_root = tracing::span!(tracing::Level::INFO, "app_start");
    ///
    /// // Assign parent trace from external context
    /// app_root.set_opentelemetry_parent(parent_context);
    /// ```
    fn set_opentelemetry_parent(&self, span_context: api::SpanContext);

    /// Extracts an `OpenTelemetry` context from `self`.
    ///
    /// ```rust
    /// use opentelemetry::api;
    /// use tracing_opentelemetry::OpenTelemetrySpanExt;
    ///
    /// fn make_request(span_context: api::SpanContext) {
    ///     // perform external request after injecting context
    ///     // e.g. if there are request headers that impl `opentelemetry::api::Carrier`
    ///     // then `propagator.inject(span_context, request.headers_mut())`
    /// }
    ///
    /// // Generate a tracing span as usual
    /// let app_root = tracing::span!(tracing::Level::INFO, "app_start");
    ///
    /// // To include tracing span context in client requests from _this_ app,
    /// // extract the current OpenTelemetry span context.
    /// make_request(app_root.opentelemetry_context())
    /// ```
    fn opentelemetry_context(&self) -> api::SpanContext;
}

impl OpenTelemetrySpanExt for tracing::Span {
    fn set_opentelemetry_parent(&self, parent_context: api::SpanContext) {
        self.with_subscriber(move |(id, subscriber)| {
            if let Some(get_context) = subscriber.downcast_ref::<WithContext>() {
                get_context.with_context(subscriber, id, move |otel_info| {
                    otel_info.trace_id = parent_context.trace_id();
                    otel_info.builder.parent_context = Some(parent_context.clone());
                });
            }
        });
    }

    fn opentelemetry_context(&self) -> api::SpanContext {
        let mut span_context = None;
        self.with_subscriber(|(id, subscriber)| {
            if let Some(get_context) = subscriber.downcast_ref::<WithContext>() {
                get_context.with_context(subscriber, id, |otel_info| {
                    span_context = Some(build_context(otel_info));
                })
            }
        });

        span_context.unwrap_or_else(api::SpanContext::empty_context)
    }
}
