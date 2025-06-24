use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Operation {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
    op: Operator,
}

impl Operation {
    pub fn new(s: &TrimmedStr) -> Result<Self, Error> {
        // find and parse is not the best, but it's simple
        let nth = s
            .find(Operator::OP_CHAR_LIST)
            .ok_or(OperationError::NotFound)?;
        let (lhs, s) = s.split_at(nth);
        let (op, rhs) = s.split_at(Operator::LEN_OF_OP);
        let lhs = Expression::new(&lhs.into())?;
        if lhs.is_empty() {
            return Err(OperationError::InvalidLhs)?;
        }
        let rhs = Expression::new(&rhs.into())?;
        if rhs.is_empty() {
            return Err(OperationError::InvalidRhs)?;
        }
        Ok(Self {
            lhs: lhs.into(),
            rhs: rhs.into(),
            op: Operator::new(&op.into())?,
        })
    }
    pub fn eval(&self, env: &Environment) -> Result<Value, Error> {
        let Value::Number(lhs) = self.lhs.eval(env)? else {
            return Err(OperationError::InvalidLhs)?;
        };
        let Value::Number(rhs) = self.rhs.eval(env)? else {
            return Err(OperationError::InvalidRhs)?;
        };
        let (lhs, rhs) = (lhs.inner(), rhs.inner());
        let value = match self.op {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        };
        Ok(Value::Number(Number::from_i32(value)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_empty() {
        assert_eq!(
            Operation::new(&"".into()),
            Err(Error::Operation(OperationError::NotFound))
        );
    }
    #[test]
    #[should_panic]
    fn parse_block() {
        // TODO: fix it because now expression cannot parse symbols in block
        assert_eq!(
            Operation::new(&"{let x = 0; x + 1} + 2".into()),
            Ok(Operation {
                lhs: Expression::Block(Block::new(&"{let x = 0; x + 1}".into()).unwrap()).into(),
                rhs: Expression::Number(Number::from_i32(2)).into(),
                op: Operator::Add
            })
        );
    }
    #[test]
    fn parse_binding() {
        assert_eq!(
            Operation::new(&"x - y".into()),
            Ok(Operation {
                lhs: Expression::Binding(Identifier::new(&"x".into()).unwrap()).into(),
                rhs: Expression::Binding(Identifier::new(&"y".into()).unwrap()).into(),
                op: Operator::Sub
            })
        );
    }
    #[test]
    fn parse_without_operator() {
        assert_eq!(
            Operation::new(&"1".into()),
            Err(Error::Operation(OperationError::NotFound))
        );
    }
    #[test]
    fn parse_one_add_two() {
        assert_eq!(
            Operation::new(&"1+2".into()),
            Ok(Operation {
                lhs: Expression::Number(Number::new(&"1".into()).unwrap()).into(),
                rhs: Expression::Number(Number::new(&"2".into()).unwrap()).into(),
                op: Operator::Add
            })
        );
    }
    #[test]
    fn parse_with_whitespace() {
        assert_eq!(
            Operation::new(&" 12 * 32 ".into()),
            Ok(Operation {
                lhs: Expression::Number(Number::new(&"12".into()).unwrap()).into(),
                rhs: Expression::Number(Number::new(&"32".into()).unwrap()).into(),
                op: Operator::Mul
            })
        );
    }
    #[test]
    fn eval_add() {
        assert_eq!(
            Operation::new(&"1+2".into())
                .unwrap()
                .eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(1 + 2)))
        );
    }
    #[test]
    fn eval_sub() {
        assert_eq!(
            Operation::new(&"15-2".into())
                .unwrap()
                .eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(15 - 2)))
        );
    }
    #[test]
    fn eval_mul() {
        assert_eq!(
            Operation::new(&"12*12".into())
                .unwrap()
                .eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(12 * 12)))
        );
    }
    #[test]
    fn eval_div() {
        assert_eq!(
            Operation::new(&"9/3".into())
                .unwrap()
                .eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32(9 / 3)))
        );
    }
    #[test]
    fn eval_binding() {
        let local = &mut Environment::default();
        BindingDef::new(&"let x = 5".into()).unwrap().store(local);
        assert_eq!(
            Operation::new(&"x+2".into()).unwrap().eval(local),
            Ok(Value::Number(Number::from_i32(5 + 2)))
        );
        BindingDef::new(&"let y = 3".into()).unwrap().store(local);
        assert_eq!(
            Operation::new(&"x*y".into()).unwrap().eval(local),
            Ok(Value::Number(Number::from_i32(5 * 3)))
        );
        assert_eq!(
            Operation::new(&"z+1".into()).unwrap().eval(local),
            Err(Error::Binding(BindingError::NotFound))
        );
    }
    #[test]
    #[should_panic]
    fn eval_block_without_last_expression() {
        // TODO: fix it because now expression cannot parse symbols in block
        assert_eq!(
            Operation::new(&"{1+1;}+1".into())
                .unwrap()
                .eval(&Environment::default()),
            Err(Error::Operation(OperationError::InvalidLhs))
        );
    }
    #[test]
    #[should_panic]
    fn eval_block_with_last_expression() {
        // TODO: fix it because now expression cannot parse symbols in block
        assert_eq!(
            Operation::new(&"{let x = 1; x + 2} + 3".into())
                .unwrap()
                .eval(&Environment::default()),
            Ok(Value::Number(Number::from_i32((1 + 2) + 3)))
        );
    }
}
