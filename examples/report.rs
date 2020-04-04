#[macro_use]
extern crate tracing;

use opentelemetry::{api::Provider, sdk};
use std::{thread, time::Duration};
use tracing_attributes::instrument;
use tracing_opentelemetry::OpenTelemetryLayer;
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

fn init_tracer() -> thrift::Result<(sdk::Tracer, sdk::Sampler)> {
    let sampler = sdk::Sampler::Always;
    let exporter = opentelemetry_jaeger::Exporter::builder()
        .with_agent_endpoint("127.0.0.1:6831".parse().unwrap())
        .with_process(opentelemetry_jaeger::Process {
            service_name: "report_example".to_string(),
            tags: Vec::new(),
        })
        .init()?;
    let provider = sdk::Provider::builder()
        .with_simple_exporter(exporter)
        .with_config(sdk::Config {
            default_sampler: Box::new(sampler),
            ..Default::default()
        })
        .build();
    let tracer = provider.get_tracer("report");

    Ok((tracer, sampler))
}

fn main() -> thrift::Result<()> {
    let (tracer, sampler) = init_tracer()?;
    let opentelemetry = OpenTelemetryLayer::new(tracer, sampler);
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

    Ok(())
}
