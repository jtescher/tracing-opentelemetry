#[macro_use]
extern crate tracing;

use opentelemetry::{api::{Provider, Sampler}, exporter::trace::jaeger, global, sdk};
use std::{thread, time::Duration};
use tracing_attributes::instrument;
use tracing_opentelemetry::OpentelemetryLayer;
use tracing_subscriber::{Layer, Registry};

#[instrument]
#[inline]
fn expensive_work() -> String {
    span!(tracing::Level::INFO, "expensive_step_1")
        .in_scope(|| thread::sleep(Duration::from_millis(25)));
    span!(tracing::Level::INFO, "expensive_step_2")
        .in_scope(|| thread::sleep(Duration::from_millis(25)));

    format!("success")
}

fn init_tracer() {
    let exporter = jaeger::Exporter::builder()
        .with_collector_endpoint("127.0.0.1:6831".parse().unwrap())
        .with_process(jaeger::Process {
            service_name: "report_example",
            tags: Vec::new(),
        })
        .init();
    let provider = sdk::Provider::builder()
        .with_exporter(exporter)
        .with_config(sdk::Config {
            default_sampler: Sampler::Always,
            ..Default::default()
        })
        .build();
    global::set_provider(provider);
}

fn main() {
    init_tracer();
    let tracer = global::trace_provider().get_tracer("tracing");
    let opentelemetry = OpentelemetryLayer::with_tracer(tracer);
    let subscriber = opentelemetry.with_subscriber(Registry::default());

    tracing::subscriber::with_default(subscriber, || {
        let root = span!(tracing::Level::INFO, "app_start", work_units = 2);
        let _enter = root.enter();

        let work_result = expensive_work();

        span!(tracing::Level::INFO, "faster_work")
            .in_scope(|| thread::sleep(Duration::from_millis(10)));

        warn!("About to exit!");
        trace!("status: {}", work_result);
    });

    // Allow flush
    thread::sleep(Duration::from_millis(250))
}
