#[macro_use]
extern crate tracing;

use opentelemetry::sdk;
use std::sync::Arc;
use std::{self, thread, time::Duration};
use tracing_attributes::instrument;
use tracing_opentelemetry::subscriber::OpentelemetrySubscriber;
use tracing_subscriber::{EnvFilter, Layer};

#[instrument]
#[inline]
fn expensive_work() -> String {
    span!(tracing::Level::TRACE, "expensive_step_1")
        .in_scope(|| thread::sleep(Duration::from_millis(25)));
    span!(tracing::Level::TRACE, "expensive_step_2")
        .in_scope(|| thread::sleep(Duration::from_millis(25)));

    format!("success")
}

fn main() {
    let tracer = sdk::Tracer::new("report_example");
    let opentelemetry = OpentelemetrySubscriber::<sdk::Tracer>::builder()
        .with_tracer(Arc::new(tracer))
        .init();
    let subscriber = EnvFilter::from_default_env().with_subscriber(opentelemetry);

    tracing::subscriber::with_default(subscriber, || {
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        let work_result = expensive_work();

        span!(tracing::Level::TRACE, "faster_work")
            .in_scope(|| thread::sleep(Duration::from_millis(10)));

        warn!("About to exit!");
        trace!("status: {}", work_result);
    });

    // Allow flush
    thread::sleep(Duration::from_millis(250))
}
