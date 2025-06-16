use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Number(Number),
    Operation(Operation),
}
impl Expr {
    fn new(s: &str) -> Result<Self, ExprError> {
        if let Ok(op) = Operation::new(s) {
            return Ok(Self::Operation(op));
        }
        if let Ok(number) = Number::new(&s.into()) {
            return Ok(Self::Number(number));
        }
        Err(ExprError::InvalidExpr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_operation() {
        assert_eq!(
            Expr::new("1+2"),
            Ok(Expr::Operation(Operation::new("1+2").unwrap()))
        );
    }
    #[test]
    fn parse_number() {
        assert_eq!(
            Expr::new("114"),
            Ok(Expr::Number(Number::new(&"114".into()).unwrap()))
        );
    }
    #[test]
    fn parse_invalid_expr() {
        assert_eq!(Expr::new(""), Err(ExprError::InvalidExpr));
        assert_eq!(Expr::new("++"), Err(ExprError::InvalidExpr));
        assert_eq!(Expr::new("1+"), Err(ExprError::InvalidExpr));
    }
}
