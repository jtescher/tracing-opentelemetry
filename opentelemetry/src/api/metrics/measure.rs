use crate::api::metrics;

pub trait Measure<T>: metrics::Instrument<T> {
    type Handle: MeasureHandle<T>;
    // Creates a Measurement object to use with batch recording.
    fn measurement(&self, value: T) -> metrics::Measurement;

    fn record<L: metrics::LabelSet>(&mut self, value: T, label_set: L);
}

pub trait MeasureHandle<T> {
    fn record(&mut self, value: T);
}
