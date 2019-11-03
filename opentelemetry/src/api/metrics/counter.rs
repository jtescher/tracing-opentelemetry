use crate::api::metrics;

pub trait Counter<T>: metrics::Instrument<T> {
    type Handle: CounterHandle<T>;
    // Creates a Measurement object to use with batch recording.
    fn measurement(&self, value: T) -> metrics::Measurement;

    fn add<L: metrics::LabelSet>(&mut self, value: T, label_set: L);
}

pub trait CounterHandle<T> {
    fn add(&mut self, value: T);
}
