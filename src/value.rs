use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Number(Number),
}

impl Value {
    const fn from_number(number: Number) -> Self {
        Self::Number(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_number() {
        assert_eq!(
            Value::from_number(Number::from_i32(11451)),
            Value::Number(Number::from_i32(11451))
        );
    }
}
