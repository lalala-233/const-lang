use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    BindingDef(BindingDef),
    Expression(Expression),
}

impl Statement {
    pub fn new(s: &TrimmedStr) -> Result<Self, StatementError> {
        if let Ok(expr) = Expression::new(s) {
            return Ok(Self::Expression(expr));
        }
        let s = &TrimmedStr::new(
            s.strip_suffix(';')
                .ok_or(StatementError::BindingDefMissingSemicolon)?,
        );
        if let Ok(binding_def) = BindingDef::new(s) {
            return Ok(Self::BindingDef(binding_def));
        }
        if Expression::new(s).is_ok() {
            return Ok(Self::Expression(Expression::Empty));
        }
        Err(StatementError::InvalidStatement)
    }
    pub fn get_expression_in(&self, local: &mut Environment) -> Expression {
        match self {
            Self::BindingDef(binding_def) => {
                binding_def.store(local);
                Expression::Empty
            }
            Self::Expression(expression) => expression.clone(),
        }
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
                BindingDef::new(&"let x = 5".into()).unwrap()
            ))
        );
    }
    #[test]
    fn parse_expr() {
        assert_eq!(
            Statement::new(&"114+514".into()),
            Ok(Statement::Expression(Expression::Operation(
                Operation::new(&"114+514".into()).unwrap()
            )))
        );
        assert_eq!(
            Statement::new(&"1919".into()),
            Ok(Statement::Expression(Expression::Number(Number::from_i32(
                1919
            ))))
        );
    }
    #[test]
    fn parse_expression_with_semicolon() {
        // TODO: fix it when expression with semicolon has side-effect.
        assert_eq!(
            Statement::new(&"1+1;".into()),
            Ok(Statement::Expression(Expression::Empty))
        );
    }
    #[test]
    fn parse_invalid() {
        assert_eq!(
            Statement::new(&"let a=a=1;".into()),
            Err(StatementError::InvalidStatement)
        );
    }
    #[test]
    fn parse_binding_def_missing_semicolon() {
        assert_eq!(
            Statement::new(&"let a=114".into()),
            Err(StatementError::BindingDefMissingSemicolon)
        );
    }
    #[test]
    fn parse_empty() {
        assert_eq!(
            Statement::new(&"".into()),
            Ok(Statement::Expression(Expression::Empty))
        );
    }
    #[test]
    fn get_expression_in_binding_def() {
        let local = &mut Environment::default();
        assert_eq!(
            Statement::BindingDef(BindingDef::new(&"let x = 5+6".into()).unwrap())
                .get_expression_in(local),
            Expression::Empty
        );
        assert_eq!(
            local.get(&Identifier::new(&"x".into()).unwrap()),
            Some(Expression::Operation(
                Operation::new(&"5+6".into()).unwrap()
            ))
        );
    }
    #[test]
    fn get_expression_in_expression() {
        let local = &mut Environment::default();
        assert_eq!(
            Statement::new(&"114".into())
                .unwrap()
                .get_expression_in(local),
            Expression::Number(Number::from_i32(114))
        );
    }
}
