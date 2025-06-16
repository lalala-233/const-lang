use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    BindingDef(BindingDef),
    Expr(Expression),
}

impl Statement {
    pub fn new(s: &str) -> Result<Self, StatementError> {
        if let Ok(binding_def) = BindingDef::new(s) {
            return Ok(Self::BindingDef(binding_def));
        }
        if let Ok(expr) = Expression::new(s) {
            return Ok(Self::Expr(expr));
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Statement::new("let x = 5"),
            Ok(Statement::BindingDef(BindingDef::new("let x = 5").unwrap()))
        );
    }
    #[test]
    fn parse_expr() {
        assert_eq!(
            Statement::new("114+514"),
            Ok(Statement::Expr(Expression::new("114+514").unwrap()))
        );
    }
}
