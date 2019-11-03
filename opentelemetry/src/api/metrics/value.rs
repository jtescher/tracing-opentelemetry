// `MeasurementValue` represents either an integer or a floating point value of a measurement. It
// needs to be accompanied with a value kind or some source that provides a value kind describing
// this measurement value
pub struct MeasurementValue(u64);

impl MeasurementValue {
    pub fn new(value: u64) -> Self {
        MeasurementValue(value)
    }
}
