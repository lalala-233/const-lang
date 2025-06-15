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
        let (op, rhs) = left.split_at(1);
        Self {
            lhs: Number(lhs.parse().unwrap()),
            rhs: Number(rhs.parse().unwrap()),
            op: Op::new(op),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_one_add_two() {
        assert_eq!(
            Expr::new("1+2"),
            Expr {
                lhs: Number(1),
                rhs: Number(2),
                op: Op::Add
            }
        );
    }
}
