#[derive(Clone, Debug)]
pub struct Key(String);

impl Key {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Key(value.into())
    }

    pub fn bool(&self, value: bool) -> KeyValue {
        KeyValue {
            key: self.clone(),
            value: Values::Bool(value),
        }
    }

    pub fn i64(&self, value: i64) -> KeyValue {
        KeyValue {
            key: self.clone(),
            value: Values::I64(value),
        }
    }

    pub fn u64(&self, value: u64) -> KeyValue {
        KeyValue {
            key: self.clone(),
            value: Values::U64(value),
        }
    }

    pub fn f64(&self, value: f64) -> KeyValue {
        KeyValue {
            key: self.clone(),
            value: Values::F64(value),
        }
    }

    pub fn string<S: Into<String>>(&self, value: S) -> KeyValue {
        KeyValue {
            key: self.clone(),
            value: Values::String(value.into()),
        }
    }

    pub fn bytes(&self, value: Vec<u8>) -> KeyValue {
        KeyValue {
            key: self.clone(),
            value: Values::Bytes(value),
        }
    }
}

#[derive(Clone, Debug)]
enum Values {
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct KeyValue {
    key: Key,
    value: Values,
}

#[derive(Default)]
pub struct Unit(String);

impl Unit {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Unit(value.into())
    }
}
