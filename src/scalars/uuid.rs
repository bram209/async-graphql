use crate::{GQLScalar, InputValueError, InputValueResult, ScalarType, Value};
use uuid::Uuid;

#[GQLScalar(internal, name = "UUID")]
impl ScalarType for Uuid {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(Uuid::parse_str(&s)?),
            _ => Err(InputValueError::ExpectedType(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}
