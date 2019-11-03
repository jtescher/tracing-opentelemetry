use opentelemetry::api::metrics::{
    gauge::GaugeHandle, measure::MeasureHandle, Gauge, Instrument, Measure, Meter, Options,
};
use opentelemetry::api::trace::provider::Provider;
use opentelemetry::api::{Span, Tracer};
use opentelemetry::{global, Key};

fn main() {
    let tracer = global::trace_provider().get_tracer("ex.com/basic");
    let meter = global::global_meter();

    let foo_key = Key::new("ex.com/foo");
    let bar_key = Key::new("ex.com/bar");
    let lemons_key = Key::new("ex.com/lemons");
    let another_key = Key::new("ex.com/another");

    let one_metric = meter.new_f64_gauge(
        "ex.com.one",
        Options::default()
            .with_keys(vec![foo_key, bar_key, lemons_key.clone()])
            .with_description("A gauge set to 1.0"),
    );

    let measure_two = meter.new_f64_measure("ex.com.two", Options::default());

    let common_labels = meter.labels(vec![lemons_key.i64(10)]);

    let mut gauge = one_metric.acquire_handle(&common_labels);

    let mut measure = measure_two.acquire_handle(&common_labels);

    tracer.with_span("operation", move |span| {
        let tracer = global::trace_provider().get_tracer("ex.com/basic");

        let mut span = span.lock().expect("Mutex poisoned");
        //        span.add_event("Nice operation!", Key::new("bogons").i64(100));
        span.add_event("Nice operation!".to_string());
        span.set_attribute(another_key.string("yes"));

        gauge.set(1.0);

        meter.record_batch(
            common_labels,
            vec![one_metric.measurement(1.0), measure_two.measurement(2.0)],
        );

        tracer.with_span("Sub operation...", move |span| {
            let mut span = span.lock().expect("Mutex poisoned");
            span.set_attribute(lemons_key.string("five"));

            // TODO Into<String>
            span.add_event("Sub span event".to_string());

            measure.record(1.3);
        })
    })
}
