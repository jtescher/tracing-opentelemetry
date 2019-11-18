#[macro_use]
extern crate tracing;

use opentelemetry::api::Provider;
use opentelemetry::sdk;
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

fn main() {
    let tracer = sdk::Provider::default().get_tracer("report_example");
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
