use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Operation {
    lhs: Number,
    rhs: Number,
    op: Operator,
}

impl Operation {
    pub fn new(s: &TrimmedStr) -> Result<Self, OperationError> {
        // find and parse is not the best, but it's simple
        let nth = s
            .find(Operator::OP_CHAR_LIST)
            .ok_or(OperationError::OperatorNotFound)?;
        let (lhs, s) = s.split_at(nth);
        let (op, rhs) = s.split_at(Operator::LEN_OF_OP);

        Ok(Self {
            lhs: Number::new(&lhs.into())?,
            rhs: Number::new(&rhs.into())?,
            op: Operator::new(&op.into())?,
        })
    }
    pub const fn eval(&self) -> Value {
        let (lhs, rhs) = (self.lhs.inner(), self.rhs.inner());
        let value = match self.op {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        };
        Value::Number(Number::from_i32(value))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_one_add_two() {
        assert_eq!(
            Operation::new(&"1+2".into()),
            Ok(Operation {
                lhs: Number::new(&"1".into()).unwrap(),
                rhs: Number::new(&"2".into()).unwrap(),
                op: Operator::Add
            })
        );
    }
    #[test]
    fn parse_with_whitespace() {
        assert_eq!(
            Operation::new(&" 12 * 32 ".into()),
            Ok(Operation {
                lhs: Number::new(&"12".into()).unwrap(),
                rhs: Number::new(&"32".into()).unwrap(),
                op: Operator::Mul
            })
        );
    }
    #[test]
    fn eval_add() {
        assert_eq!(
            Operation::new(&"1+2".into()).unwrap().eval(),
            Value::Number(Number::from_i32(1 + 2))
        );
    }
    #[test]
    fn eval_sub() {
        assert_eq!(
            Operation::new(&"15-2".into()).unwrap().eval(),
            Value::Number(Number::from_i32(15 - 2))
        );
    }
    #[test]
    fn eval_mul() {
        assert_eq!(
            Operation::new(&"12*12".into()).unwrap().eval(),
            Value::Number(Number::from_i32(12 * 12))
        );
    }
    #[test]
    fn eval_div() {
        assert_eq!(
            Operation::new(&"9/3".into()).unwrap().eval(),
            Value::Number(Number::from_i32(9 / 3))
        );
    }
}
