pub mod counter;
pub mod gauge;
pub mod measure;
pub mod noop;
pub mod value;

pub use counter::Counter;
pub use gauge::Gauge;
pub use measure::Measure;
pub use value::MeasurementValue;

// Instrument is the implementation-level interface Set/Add/Record
// individual metrics without precomputed labels.
pub trait Instrument<T> {
    type Handle: Handle<T>;
    // AcquireHandle creates a Handle to record metrics with
    // precomputed labels.
    fn acquire_handle<L: LabelSet>(&self, labels: &L) -> Self::Handle;

    // RecordOne allows the SDK to observe a single metric event.
    fn record_one<L: LabelSet>(&mut self, value: MeasurementValue, labels: L);
}

// Handle is the implementation-level interface to Set/Add/Record
// individual metrics with precomputed labels
pub trait Handle<T> {
    // RecordOne allows the SDK to observe a single metric event.
    fn record_one(&mut self, value: MeasurementValue);
}

// LabelSet is an implementation-level interface that represents a
// KeyValue for use as pre-defined labels in the metrics API.
pub trait LabelSet {}

// Options contains some options for metrics of any kind.
#[derive(Default)]
pub struct Options {
    // Description is an optional field describing the metric
    // instrument.
    pub description: String,

    // Unit is an optional field describing the metric instrument.
    pub unit: crate::Unit,

    // Keys are recommended keys determined in the handles
    // obtained for the metric.
    pub keys: Vec<crate::Key>,

    // Alternate defines the property of metric value dependent on
    // a metric type.
    //
    // - for Counter, true implies that the metric is an up-down
    //   Counter
    //
    // - for Gauge, true implies that the metric is a
    //   non-descending Gauge
    //
    // - for Measure, true implies that the metric supports
    //   positive and negative values
    pub alternate: bool,
}

impl Options {
    pub fn with_description<S: Into<String>>(self, description: S) -> Self {
        Options {
            description: description.into(),
            ..self
        }
    }

    pub fn with_unit(self, unit: crate::Unit) -> Self {
        Options { unit, ..self }
    }

    pub fn with_keys(self, keys: Vec<crate::Key>) -> Self {
        Options { keys, ..self }
    }

    pub fn with_monotonic(self, _monotonic: bool) -> Self {
        // TODO figure out counter vs gauge issue here.
        unimplemented!()
    }

    pub fn with_absolute(self, absolute: bool) -> Self {
        Options {
            alternate: !absolute,
            ..self
        }
    }
}

pub struct Measurement {
    value: MeasurementValue,
}

//impl Measurement {
//    // Instrument returns an instrument that created this measurement.
//    fn instrument(&self) -> &Instrument {
//        &self.instrument
//    }
//
//    // Value returns a value recorded in this measurement.
//    fn value(&self) -> &MeasuremMeasurementValueentValue {
//        &self.value
//    }
//}

// Meter is an interface to the metrics portion of the OpenTelemetry SDK.
pub trait Meter {
    type LabelSet: LabelSet;
    type I64Counter: Counter<i64>;
    type F64Counter: Counter<f64>;
    type I64Gauge: Gauge<i64>;
    type F64Gauge: Gauge<f64>;
    type I64Measure: Measure<i64>;
    type F64Measure: Measure<f64>;

    // Returns a reference to a set of labels that cannot be read by the application.
    fn labels(&self, key_values: Vec<crate::KeyValue>) -> Self::LabelSet;

    // Creates a new integral counter with a given name and customized with passed options.
    fn new_i64_counter<S: Into<String>>(&self, name: S, opts: Options) -> Self::I64Counter;

    // Creates a new floating point counter with a given name and customized with passed options.
    fn new_f64_counter<S: Into<String>>(&self, name: S, opts: Options) -> Self::F64Counter;

    // Creates a new integral gauge with a given name and customized with passed options.
    fn new_i64_gauge<S: Into<String>>(&self, name: S, opts: Options) -> Self::I64Gauge;

    // Creates a new floating point gauge with a given name and customized with passed options.
    fn new_f64_gauge<S: Into<String>>(&self, name: S, opts: Options) -> Self::F64Gauge;

    // Creates a new integral measure with a given name and customized with passed options.
    fn new_i64_measure<S: Into<String>>(&self, name: S, opts: Options) -> Self::I64Measure;

    // Creates a new floating point measure with a given name and customized with passed options.
    fn new_f64_measure<S: Into<String>>(&self, name: S, opts: Options) -> Self::F64Measure;

    // Atomically records a batch of measurements.
    fn record_batch<M: IntoIterator<Item = Measurement>>(
        &self,
        label_set: Self::LabelSet,
        measurements: M,
    );
}
