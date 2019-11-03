use crate::api::metrics;
use crate::api::metrics::Instrument;
use std::marker;

pub struct NoopMeter {}

impl metrics::Meter for NoopMeter {
    type LabelSet = NoopLabelSet;
    type I64Counter = NoopCounter<i64>;
    type F64Counter = NoopCounter<f64>;
    type I64Gauge = NoopGauge<i64>;
    type F64Gauge = NoopGauge<f64>;
    type I64Measure = NoopMeasure<i64>;
    type F64Measure = NoopMeasure<f64>;

    fn labels(&self, _key_values: Vec<crate::KeyValue>) -> Self::LabelSet {
        NoopLabelSet {}
    }

    fn new_i64_counter<S: Into<String>>(
        &self,
        _name: S,
        _opts: metrics::Options,
    ) -> Self::I64Counter {
        NoopCounter {
            _marker: marker::PhantomData,
        }
    }

    fn new_f64_counter<S: Into<String>>(
        &self,
        _name: S,
        _opts: metrics::Options,
    ) -> Self::F64Counter {
        NoopCounter {
            _marker: marker::PhantomData,
        }
    }

    fn new_i64_gauge<S: Into<String>>(&self, _name: S, _opts: metrics::Options) -> Self::I64Gauge {
        NoopGauge {
            _marker: marker::PhantomData,
        }
    }

    fn new_f64_gauge<S: Into<String>>(&self, _name: S, _opts: metrics::Options) -> Self::F64Gauge {
        NoopGauge {
            _marker: marker::PhantomData,
        }
    }

    fn new_i64_measure<S: Into<String>>(
        &self,
        _name: S,
        _opts: metrics::Options,
    ) -> Self::I64Measure {
        NoopMeasure {
            _marker: marker::PhantomData,
        }
    }

    fn new_f64_measure<S: Into<String>>(
        &self,
        _name: S,
        _opts: metrics::Options,
    ) -> Self::F64Measure {
        NoopMeasure {
            _marker: marker::PhantomData,
        }
    }

    fn record_batch<M: IntoIterator<Item = metrics::Measurement>>(
        &self,
        _label_set: NoopLabelSet,
        _measurements: M,
    ) {
        // Ignored
    }
}

pub struct NoopLabelSet {}

impl metrics::LabelSet for NoopLabelSet {}

pub struct NoopHandle<T> {
    _marker: marker::PhantomData<T>,
}

impl<T> metrics::Handle<T> for NoopHandle<T> {
    fn record_one(&mut self, _value: metrics::MeasurementValue) {
        // Ignored
    }
}

impl<T> metrics::counter::CounterHandle<T> for NoopHandle<T> {
    fn add(&mut self, _value: T) {
        // Ignored
    }
}

impl<T> metrics::gauge::GaugeHandle<T> for NoopHandle<T> {
    fn set(&mut self, _value: T) {
        // Ignored
    }
}

impl<T> metrics::measure::MeasureHandle<T> for NoopHandle<T> {
    fn record(&mut self, _value: T) {
        // Ignored
    }
}

pub struct NoopCounter<T> {
    _marker: marker::PhantomData<T>,
}

impl<T> metrics::Counter<T> for NoopCounter<T> {
    type Handle = NoopHandle<T>;
    fn measurement(&self, _value: T) -> metrics::Measurement {
        unimplemented!()
    }

    fn add<L: metrics::LabelSet>(&mut self, _value: T, _label_set: L) {
        // Ignored
    }
}

impl<T> metrics::Instrument<T> for NoopCounter<T> {
    type Handle = NoopHandle<T>;

    fn acquire_handle<L: metrics::LabelSet>(&self, _labels: &L) -> Self::Handle {
        NoopHandle {
            _marker: marker::PhantomData,
        }
    }

    fn record_one<L: metrics::LabelSet>(&mut self, _value: metrics::MeasurementValue, _labels: L) {
        // Ignored
    }
}

pub struct NoopGauge<T> {
    _marker: marker::PhantomData<T>,
}

impl metrics::Gauge<i64> for NoopGauge<i64> {
    type Handle = NoopHandle<i64>;
    fn measurement(&self, value: i64) -> metrics::Measurement {
        metrics::Measurement {
            value: metrics::MeasurementValue::new(value as u64),
        }
    }

    fn set<L: metrics::LabelSet>(&mut self, value: i64, label_set: L) {
        self.record_one(metrics::MeasurementValue::new(value as u64), label_set)
    }
}

impl metrics::Gauge<f64> for NoopGauge<f64> {
    type Handle = NoopHandle<f64>;
    fn measurement(&self, value: f64) -> metrics::Measurement {
        metrics::Measurement {
            value: metrics::MeasurementValue::new(value.to_bits()),
        }
    }

    fn set<L: metrics::LabelSet>(&mut self, value: f64, label_set: L) {
        self.record_one(metrics::MeasurementValue::new(value.to_bits()), label_set)
    }
}

impl<T> metrics::Instrument<T> for NoopGauge<T> {
    type Handle = NoopHandle<T>;

    fn acquire_handle<L: metrics::LabelSet>(&self, _labels: &L) -> Self::Handle {
        NoopHandle {
            _marker: marker::PhantomData,
        }
    }

    fn record_one<L: metrics::LabelSet>(&mut self, _value: metrics::MeasurementValue, _labels: L) {
        // Ignored
    }
}

pub struct NoopMeasure<T> {
    _marker: marker::PhantomData<T>,
}

impl metrics::Measure<i64> for NoopMeasure<i64> {
    type Handle = NoopHandle<i64>;

    fn measurement(&self, value: i64) -> metrics::Measurement {
        metrics::Measurement {
            value: metrics::MeasurementValue::new(value as u64),
        }
    }

    fn record<L: metrics::LabelSet>(&mut self, value: i64, label_set: L) {
        self.record_one(metrics::MeasurementValue::new(value as u64), label_set)
    }
}

impl metrics::Measure<f64> for NoopMeasure<f64> {
    type Handle = NoopHandle<f64>;

    fn measurement(&self, value: f64) -> metrics::Measurement {
        metrics::Measurement {
            value: metrics::MeasurementValue::new(value.to_bits()),
        }
    }

    fn record<L: metrics::LabelSet>(&mut self, value: f64, label_set: L) {
        self.record_one(metrics::MeasurementValue::new(value.to_bits()), label_set)
    }
}

impl<T> metrics::Instrument<T> for NoopMeasure<T> {
    type Handle = NoopHandle<T>;

    fn acquire_handle<L: metrics::LabelSet>(&self, _labels: &L) -> Self::Handle {
        NoopHandle {
            _marker: marker::PhantomData,
        }
    }

    fn record_one<L: metrics::LabelSet>(&mut self, _value: metrics::MeasurementValue, _labels: L) {
        // Ignored
    }
}
