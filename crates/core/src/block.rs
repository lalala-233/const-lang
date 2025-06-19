use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    statements: Vec<Statement>,
}
impl Block {
    pub fn new(s: &TrimmedStr) -> Result<Self, Error> {
        let Some(s) = s.strip_prefix('{') else {
            return Err(BlockError::MissingOpeningBrace)?;
        };
        let Some(s) = s.strip_suffix('}') else {
            return Err(BlockError::MissingClosingBrace)?;
        };
        let statements = TrimmedStr::new(s)
            .split_inclusive(';')
            .map(|s| Statement::new(&s.into()))
            .collect::<Result<_, _>>()?;
        Ok(Self { statements })
    }
    pub fn get_expression_from(&self, local: &mut Environment) -> Expression {
        let mut last = Expression::Empty;
        for statement in &self.statements {
            last = statement.get_expression_in(local);
        }
        last
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_empty_block() {
        assert_eq!(
            Block::new(&"{}".into()),
            Ok(Block {
                statements: Vec::new()
            })
        );
    }
    #[test]
    fn parse_empty_with_whitespace() {
        assert_eq!(
            Block::new(&"{   \n   \n     }".into()),
            Ok(Block {
                statements: Vec::new()
            })
        );
    }
    #[test]
    fn parse_with_one_statement() {
        assert_eq!(
            Block::new(&"{ 11451 }".into()),
            Ok(Block {
                statements: vec![Statement::Expression(Expression::Number(Number::from_i32(
                    11451
                )))]
            })
        );
    }
    #[test]
    fn parse_with_one_expr() {
        assert_eq!(
            Block::new(&"{ a }".into()),
            Ok(Block {
                statements: vec![Statement::Expression(Expression::Binding(
                    Binding::new("a").unwrap()
                ))]
            })
        );
    }
    #[test]
    fn parse_with_multiple_statements() {
        assert_eq!(
            Block::new(
                &"{ let a = 11451;
        let b = a;
        b}"
                .into()
            ),
            Ok(Block {
                statements: vec![
                    Statement::BindingDef(BindingDef::new(&"let a = 11451".into()).unwrap()),
                    Statement::BindingDef(BindingDef::new(&"let b = a".into()).unwrap()),
                    Statement::Expression(Expression::Binding(Binding::new("b").unwrap()))
                ]
            })
        );
    }
    #[test]
    fn parse_with_one_line_but_multiple_statements() {
        assert_eq!(
            Block::new(&"{let a = 1;a}".into()),
            Ok(Block {
                statements: vec![
                    Statement::BindingDef(BindingDef::new(&"let a = 1".into()).unwrap()),
                    Statement::Expression(Expression::Binding(Binding::new("a").unwrap()))
                ]
            })
        );
    }
    #[test]
    fn parse_without_braces() {
        assert_eq!(
            Block::new(&"{let a = 11451".into()),
            Err(Error::Block(BlockError::MissingClosingBrace))
        );
        assert_eq!(
            Block::new(&"let a = 11451;}".into()),
            Err(Error::Block(BlockError::MissingOpeningBrace))
        );
        assert_eq!(
            Block::new(&"".into()),
            Err(Error::Block(BlockError::MissingOpeningBrace))
        );
    }
    #[test]
    fn parse_with_error_in_statement() {
        assert_eq!(
            Block::new(&"{let a = 11451}".into()),
            Err(Error::Statement(StatementError::BindingDefMissingSemicolon))
        );
        assert_eq!(
            Block::new(&"{let 1 = 5;}".into()),
            Err(Error::Statement(StatementError::InvalidStatement))
        );
    }
    #[test]
    fn get_expression_from_empty() {
        assert_eq!(
            Block { statements: vec![] }.get_expression_from(&mut Environment::default()),
            Expression::Empty
        );
    }
    #[test]
    fn get_expression_from_multiple_line() {
        let local = &mut Environment::default();
        assert_eq!(
            Block::new(&"{let a = 11451; let b = 11452; b}".into())
                .unwrap()
                .get_expression_from(local),
            Expression::Binding(Binding::new("b").unwrap())
        );
        assert_eq!(
            local.get(&"a".try_into().unwrap()),
            Some(Expression::Number(Number::from_i32(11451)))
        );
        assert_eq!(
            local.get(&"b".try_into().unwrap()),
            Some(Expression::Number(Number::from_i32(11452)))
        );
    }
    #[test]
    fn get_expression_from_multiple_expr() {
        assert_eq!(
            Block::new(&"{114; 514; 1919;}".into())
                .unwrap()
                .get_expression_from(&mut Environment::default()),
            Expression::Empty
        );
        assert_eq!(
            Block::new(&"{114; 514; 1919; 810}".into())
                .unwrap()
                .get_expression_from(&mut Environment::default()),
            Expression::Number(Number::from_i32(810))
        );
    }
    #[test]
    fn get_block_expression_from_block() {
        // TODO: fix it
        assert_eq!(
            Block::new(&"{let a = 11451;{let b = 11452; b}}".into())
                .unwrap()
                .get_expression_from(&mut Environment::default()),
            Expression::Block(Block::new(&"{let b = 11452; b}".into()).unwrap())
        );
    }
}
