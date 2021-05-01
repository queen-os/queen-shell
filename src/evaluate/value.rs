use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use num_bigint::BigInt;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Value {
    Nothing,
    /// A "big int", an integer with arbitrarily large size (aka not limited to 64-bit)
    Int(BigInt),
    /// A "big decimal", an decimal number with arbitrarily large size (aka not limited to 64-bit)
    Number(OrderedFloat<f64>),
    /// A string value
    String(String),
    /// A glob pattern, eg foo*
    Pattern(String),
    /// A file path
    Path(String),
    Boolean(bool),
    List(Vec<Value>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Number(i) => i.to_string(),
            Value::String(s) => s.clone(),
            Value::Pattern(s) => s.clone(),
            Value::Path(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::List(v) => v.iter().map(Self::to_string).collect::<Vec<_>>().join(" "),
            Value::Nothing => String::new(),
        }
    }
}
