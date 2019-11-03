//use crate::api;
//use std::sync::Arc;
//use crate::api::Tracer;
//use crate::api::metrics::Meter;
//
////lazy_static::lazy_static! {
////    static ref TRACE_PROVIDER: Mutex<dyn api::Provider<'static>> = {
////       api::trace::noop::NoopProvider::new()
////    };
////}
////
////pub fn trace_provider<'a>() -> &'static impl api::Provider<'a> {
////    &TRACE_PROVIDER
////}
//
////lazy_static::lazy_static! {
////    static ref GLOBAL_METER: Mutex<Box<dyn api::metrics::Meter>> = {
////       api::metrics::noop::NoopMeter {}
////    };
////}
//
////pub fn global_meter() -> &'static Mutex<dyn api::metrics::Meter> {
////    &Mutex::new(api::metrics::noop::NoopMeter {})
////}
//
//static GLOBAL_TRACER_INIT: AtomicUsize = AtomicUsize::new(UNINITIALIZED);
//static GLOBAL_METER_INIT: AtomicUsize = AtomicUsize::new(UNINITIALIZED);
//
//const UNINITIALIZED: usize = 0;
//const INITIALIZING: usize = 1;
//const INITIALIZED: usize = 2;
//
//static mut GLOBAL_TRACER_DISPATCH: Option<TracerDispatch> = None;
//static mut GLOBAL_METER_DISPATCH: Option<MeterDispatch> = None;
//
//
///// `TracerDispatch` dispatch trace data to a `Tracer`.
//#[derive(Clone)]
//pub struct TracerDispatch {
//    tracer: Arc<dyn Tracer + Send + Sync>,
//}
//
///// `TracerDispatch` dispatch trace data to a `Tracer`.
//#[derive(Clone)]
//pub struct MeterDispatch {
//    meter: Arc<dyn Meter + Send + Sync>,
//}

pub fn trace_provider() -> crate::api::trace::noop::NoopProvider {
    crate::api::trace::noop::NoopProvider {}
}

pub fn global_meter() -> crate::api::metrics::noop::NoopMeter {
    crate::api::metrics::noop::NoopMeter {}
}
