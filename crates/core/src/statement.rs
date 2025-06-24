use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    BindingDef(BindingDef),
    Expression(Expression),
    FunctionDef(FunctionDef),
}

impl Statement {
    fn pre_parse(s: &TrimmedStr) -> Option<Self> {
        if let Ok(expression) = Expression::new(s) {
            return Some(Self::Expression(expression));
        }
        if let Ok(function_def) = FunctionDef::new(s) {
            return Some(Self::FunctionDef(function_def));
        }
        None
    }
    fn parse_after_strip_semicolon(s: &TrimmedStr) -> Option<Self> {
        if let Ok(binding_def) = BindingDef::new(s) {
            return Some(Self::BindingDef(binding_def));
        }
        if Expression::new(s).is_ok() {
            return Some(Self::Expression(Expression::Empty));
        }
        None
    }
    pub fn new(s: &TrimmedStr) -> Result<Self, StatementError> {
        if let Some(statement) = Self::pre_parse(s) {
            return Ok(statement);
        }
        let s = &TrimmedStr::new(
            s.strip_suffix(';')
                .ok_or(StatementError::BindingDefMissingSemicolon)?,
        );
        if let Some(statement) = Self::parse_after_strip_semicolon(s) {
            return Ok(statement);
        }
        Err(StatementError::InvalidStatement)
    }
    pub fn get_expression_in(&self, local: &mut Environment) -> Expression {
        match self {
            Self::BindingDef(binding_def) => binding_def.store(local),
            Self::FunctionDef(function_def) => function_def.store(local),
            Self::Expression(expression) => return expression.clone(),
        }
        Expression::Empty
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
    fn parse_function_def() {
        assert_eq!(
            Statement::new(&"fn one_add_one => 1+1".into()),
            Ok(Statement::FunctionDef(
                FunctionDef::new(&"fn one_add_one => 1+1".into()).unwrap()
            ))
        );
        assert_eq!(
            Statement::new(&"fn add_one x => x + 1".into()),
            Ok(Statement::FunctionDef(
                FunctionDef::new(&"fn add_one x => x + 1".into()).unwrap()
            ))
        );
    }
    #[test]
    fn parse_expression_without_semicolon() {
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
    fn pre_parse() {
        assert_eq!(Statement::pre_parse(&"let x = 1+1".into()), None);
        assert_eq!(
            Statement::pre_parse(&"1+1".into()),
            Some(Statement::Expression(Expression::Operation(
                Operation::new(&"1+1".into()).unwrap()
            )))
        );
        assert_eq!(
            Statement::pre_parse(&"fn one_add_one => 2".into()),
            Some(Statement::FunctionDef(
                FunctionDef::new(&"fn one_add_one => 2".into()).unwrap()
            ))
        );
    }
    #[test]
    fn parse_after_strip_semicolon() {
        assert_eq!(
            Statement::parse_after_strip_semicolon(&"1+1".into()),
            Some(Statement::Expression(Expression::Empty))
        );
        assert_eq!(
            Statement::parse_after_strip_semicolon(&"let x = 1+1".into()),
            Some(Statement::BindingDef(
                BindingDef::new(&"let x = 1+1".into()).unwrap()
            ))
        );
        assert_eq!(
            Statement::parse_after_strip_semicolon(&"fn one_add_one => 2".into()),
            None
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
            local.get_from_self_and_parent(&"x".try_into().unwrap()),
            Some(NamedValue::Binding(Expression::Operation(
                Operation::new(&"5+6".into()).unwrap()
            )))
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
    #[test]
    fn get_expression_in_function_def() {
        let local = &mut Environment::default();
        assert_eq!(
            Statement::FunctionDef(FunctionDef::new(&"fn add_one x => x + 1".into()).unwrap())
                .get_expression_in(local),
            Expression::Empty
        );
    }
}
