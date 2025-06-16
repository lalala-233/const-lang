use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Number(Number),
    Operation(Operation),
}
impl Expression {
    pub fn new(s: &str) -> Result<Self, ExpressionError> {
        if let Ok(op) = Operation::new(s) {
            return Ok(Self::Operation(op));
        }
        if let Ok(number) = Number::new(&s.into()) {
            return Ok(Self::Number(number));
        }
        Err(ExpressionError::InvalidExpression)
    }
    const fn eval(&self) -> Value {
        match self {
            Self::Number(number) => Value::Number(*number),
            Self::Operation(operation) => operation.eval(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_operation() {
        assert_eq!(
            Expression::new("1+2"),
            Ok(Expression::Operation(Operation::new("1+2").unwrap()))
        );
    }
    #[test]
    fn parse_number() {
        assert_eq!(
            Expression::new("114"),
            Ok(Expression::Number(Number::new(&"114".into()).unwrap()))
        );
    }
    #[test]
    fn parse_invalid_expr() {
        assert_eq!(
            Expression::new("++"),
            Err(ExpressionError::InvalidExpression)
        );
        assert_eq!(
            Expression::new("1+"),
            Err(ExpressionError::InvalidExpression)
        );
    }
    #[test]
    fn parse_empty() {
        assert_eq!(Expression::new(""), Err(ExpressionError::InvalidExpression));
    }
    #[test]
    fn eval_operation() {
        assert_eq!(
            Expression::new("114+514").unwrap().eval(),
            Value::Number(Number::from_i32(114 + 514))
        );
    }
    #[test]
    fn eval_number() {
        assert_eq!(
            Expression::new("114").unwrap().eval(),
            Value::Number(Number::from_i32(114))
        );
    }
}
