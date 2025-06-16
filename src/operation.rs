use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    lhs: Number,
    rhs: Number,
    op: Op,
}

impl Operation {
    pub fn new(s: &str) -> Self {
        let nth = s.find(Op::OP_CHAR_LIST).expect("Invalid Expr");
        let (lhs, s) = s.split_at(nth);
        let (op, rhs) = s.split_at(Op::LEN_OF_OP);

        Self {
            lhs: Number::new(&lhs.into()),
            rhs: Number::new(&rhs.into()),
            op: Op::new(&op.into()),
        }
    }
    pub const fn eval(&self) -> i32 {
        let (lhs, rhs) = (self.lhs.inner(), self.rhs.inner());
        match self.op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_one_add_two() {
        assert_eq!(
            Operation::new(" 1 + 2 "),
            Operation {
                lhs: Number::new(&"1".into()),
                rhs: Number::new(&"2".into()),
                op: Op::Add
            }
        );
    }
    #[test]
    fn eval_add() {
        assert_eq!(Operation::new("1+2").eval(), 3);
    }
    #[test]
    fn eval_sub() {
        assert_eq!(Operation::new("15-2").eval(), 13);
    }
    #[test]
    fn eval_mul() {
        assert_eq!(Operation::new("12*12").eval(), 144);
    }
    #[test]
    fn eval_div() {
        assert_eq!(Operation::new("9/3").eval(), 3);
    }
}
