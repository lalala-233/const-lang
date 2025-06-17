use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    BindingDef(BindingDef),
    Expression(Expression),
}

impl Statement {
    pub fn new(s: &TrimmedStr) -> Result<Self, StatementError> {
        if let Ok(binding_def) = BindingDef::new(s) {
            return Ok(Self::BindingDef(binding_def));
        }
        if let Ok(expr) = Expression::new(s) {
            return Ok(Self::Expression(expr));
        }
        Err(StatementError::InvalidStatement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Statement::new(&"let x = 5;".into()),
            Ok(Statement::BindingDef(
                BindingDef::new(&"let x = 5;".into()).unwrap()
            ))
        );
    }
    #[test]
    fn parse_expr() {
        assert_eq!(
            Statement::new(&"114+514".into()),
            Ok(Statement::Expression(
                Expression::new(&"114+514".into()).unwrap()
            ))
        );
    }
    #[test]
    fn parse_invalid() {
        assert_eq!(
            Statement::new(&"let a=a=1".into()),
            Err(StatementError::InvalidStatement)
        );
    }
    #[test]
    fn parse_empty() {
        assert_eq!(
            Statement::new(&"".into()),
            Ok(Statement::Expression(Expression::Empty))
        );
    }
}
