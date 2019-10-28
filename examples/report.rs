#[macro_use]
extern crate tracing;

use std::{self, thread, time::Duration};
use tracing_attributes::instrument;
use tracing_opentelemetry::subscriber::OpentelemetrySubscriber;

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
    env_logger::init();
    let subscriber = OpentelemetrySubscriber::<opentelemetry::jaeger::JaegerTracer>::builder()
        .with_tracer(opentelemetry::jaeger::JaegerTracer::new("report_example"))
        .init();

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
    thread::sleep(Duration::from_millis(250));
}
