use crate::api::metrics;

pub trait Gauge<T>: metrics::Instrument<T> {
    type Handle: GaugeHandle<T>;
    // Creates a Measurement object to use with batch recording.
    fn measurement(&self, value: T) -> metrics::Measurement;

    fn set<L: metrics::LabelSet>(&mut self, value: T, label_set: L);
}

pub trait GaugeHandle<T> {
    fn set(&mut self, value: T);
}
