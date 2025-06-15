use crate::internal::prelude::*;
#[derive(Debug, PartialEq)]
struct Expr {
    lhs: Number,
    rhs: Number,
    op: Op,
}

impl Expr {
    fn new(s: &str) -> Self {
        let nth = s.find(Op::OP_CHAR_LIST).expect("Invalid Expr");
        let (lhs, left) = s.split_at(nth);
        let (op, rhs) = left.split_at(Op::LEN_OF_OP);
        Self {
            lhs: Number::new(&lhs.into()),
            rhs: Number::new(&rhs.into()),
            op: Op::new(&op.into()),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_one_add_two() {
        assert_eq!(
            Expr::new(" 1 + 2 "),
            Expr {
                lhs: Number::new(&"1".into()),
                rhs: Number::new(&"2".into()),
                op: Op::Add
            }
        );
    }
}
