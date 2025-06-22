use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Expression {
    Number(Number),
    Operation(Operation),
    Binding(Identifier),
    Block(Block),
    #[default]
    Empty,
}
impl Expression {
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
    pub fn new(s: &TrimmedStr) -> Result<Self, ExpressionError> {
        if let Ok(op) = Operation::new(s) {
            return Ok(Self::Operation(op));
        }
        if let Ok(number) = Number::new(s) {
            return Ok(Self::Number(number));
        }
        if let Ok(binding) = Identifier::new(s) {
            return Ok(Self::Binding(binding));
        }
        if let Ok(block) = Block::new(s) {
            return Ok(Self::Block(block));
        }
        if s.is_empty() {
            return Ok(Self::Empty);
        }
        Err(ExpressionError::InvalidExpression)
    }
    pub fn eval(&self, env: &Environment) -> Result<Value, Error> {
        match self {
            Self::Number(number) => Ok(Value::Number(*number)),
            Self::Operation(operation) => operation.eval(env),
            Self::Empty => Ok(Value::Empty),
            Self::Binding(binding) => binding.get_expression_from(env)?.eval(env),
            Self::Block(block) => {
                let local = &mut env.create_child();
                block.get_expression_from(local).eval(local)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_operation_without_binding() {
        assert_eq!(
            Expression::new(&"1+2".into()),
            Ok(Expression::Operation(
                Operation::new(&"1+2".into()).unwrap()
            ))
        );
    }
    #[test]
    fn parse_operation_with_binding() {
        assert_eq!(
            Expression::new(&"x+2".into()),
            Ok(Expression::Operation(
                Operation::new(&"x+2".into()).unwrap()
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
    fn parse_empty() {
        assert_eq!(Expression::new(&"".into()), Ok(Expression::Empty));
    }
    #[test]
    fn parse_binding() {
        assert_eq!(
            Expression::new(&"something".into()),
            Ok(Expression::Binding(
                Identifier::new(&"something".into()).unwrap()
            ))
        );
    }
    #[test]
    fn parse_block() {
        assert_eq!(
            Expression::new(&"{let x=114;x}".into()),
            Ok(Expression::Block(
                Block::new(&"{let x=114;x}".into()).unwrap()
            ))
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
    fn eval_operation_without_binding() {
        assert_eq!(
            Expression::Operation(Operation::new(&"114+514".into()).unwrap())
                .eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(114 + 514)))
        );
    }
    #[test]
    fn eval_operation_with_binding() {
        let env = &mut Environment::default();
        BindingDef::new(&"let x = 114".into()).unwrap().store(env);
        assert_eq!(
            Expression::Operation(Operation::new(&"x+2".into()).unwrap()).eval(env),
            Ok(Value::Number(Number::from_i32(114 + 2)))
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
        let env = &mut Environment::default();
        BindingDef::new(&"let a = 114".into()).unwrap().store(env);
        assert_eq!(
            Expression::Binding(Identifier::new(&"a".into()).unwrap()).eval(env),
            Ok(Value::Number(Number::from_i32(114)))
        );
    }
    #[test]
    fn eval_non_existing_binding() {
        let env = Environment::default();
        assert_eq!(
            Expression::Binding(Identifier::new(&"a".into()).unwrap()).eval(&env),
            Err(Error::Binding(BindingError::NotFound))
        );
    }
    #[test]
    fn eval_block_with_binding() {
        assert_eq!(
            Expression::Block(Block::new(&"{let x = 114; x}".into()).unwrap())
                .eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(114)))
        );
    }
    #[test]
    fn eval_block_with_one_expr() {
        assert_eq!(
            Expression::Block(Block::new(&"{ 514 }".into()).unwrap()).eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(514))),
        );
    }
    #[test]
    fn eval_block_from_parent() {
        let env = {
            let mut binding = Environment::default();
            BindingDef::new(&"let a = 11451".into())
                .unwrap()
                .store(&mut binding);
            binding
        };
        assert_eq!(
            Expression::Block(Block::new(&"{ let b = a; b }".into()).unwrap()).eval(&env),
            Ok(Value::Number(Number::from_i32(11451)))
        );
    }
}
