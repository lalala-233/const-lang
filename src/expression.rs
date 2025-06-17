use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Number(Number),
    Operation(Operation),
    Binding(Binding),
    Empty,
}
impl Expression {
    pub fn new(s: &TrimmedStr) -> Result<Self, ExpressionError> {
        if let Ok(op) = Operation::new(s) {
            return Ok(Self::Operation(op));
        }
        if let Ok(number) = Number::new(s) {
            return Ok(Self::Number(number));
        }
        if let Ok(binding) = Binding::new(s) {
            return Ok(Self::Binding(binding));
        }
        if s.is_empty() {
            return Ok(Self::Empty);
        }
        Err(ExpressionError::InvalidExpression)
    }
    fn eval(&self, env: &Environment) -> Result<Value, Error> {
        match self {
            Self::Number(number) => Ok(Value::Number(*number)),
            Self::Operation(operation) => Ok(operation.eval()),
            Self::Empty => Ok(Value::Empty),
            Self::Binding(binding) => binding.get_from(env).map(|expr| expr.eval(env))?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_operation() {
        assert_eq!(
            Expression::new(&"1+2".into()),
            Ok(Expression::Operation(
                Operation::new(&"1+2".into()).unwrap()
            ))
        );
    }
    #[test]
    fn parse_number() {
        assert_eq!(
            Expression::new(&"114".into()),
            Ok(Expression::Number(Number::from_i32(114)))
        );
    }
    #[test]
    fn parse_invalid_expr() {
        assert_eq!(
            Expression::new(&"++".into()),
            Err(ExpressionError::InvalidExpression)
        );
        assert_eq!(
            Expression::new(&"1+".into()),
            Err(ExpressionError::InvalidExpression)
        );
    }
    #[test]
    fn parse_empty() {
        assert_eq!(Expression::new(&"".into()), Ok(Expression::Empty));
    }
    #[test]
    fn parse_binding() {
        assert_eq!(
            Expression::new(&"something".into()),
            Ok(Expression::Binding(
                Binding::new(&"something".into()).unwrap()
            ))
        );
    }
    #[test]
    fn eval_operation() {
        assert_eq!(
            Expression::Operation(Operation::new(&"114+514".into()).unwrap())
                .eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(114 + 514)))
        );
    }
    #[test]
    fn eval_number() {
        assert_eq!(
            Expression::Number(Number::from_i32(114)).eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(114)))
        );
    }
    #[test]
    fn eval_empty() {
        assert_eq!(
            Expression::Empty.eval(&Environment::default()),
            Ok(Value::Empty)
        );
    }
    #[test]
    fn eval_existing_binding() {
        let mut env = Environment::default();
        BindingDef::new(&"let a = 114;".into())
            .unwrap()
            .store(&mut env);
        assert_eq!(
            Expression::Binding(Binding::new(&"a".into()).unwrap()).eval(&env),
            Ok(Value::Number(Number::from_i32(114)))
        );
    }
    #[test]
    fn eval_non_existing_binding() {
        let env = Environment::default();
        assert_eq!(
            Expression::Binding(Binding::new(&"a".into()).unwrap()).eval(&env),
            Err(Error::Binding(BindingError::BindingNotFound))
        );
    }
}
