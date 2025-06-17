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
        let statements = s
            .trim()
            .split_inclusive(';')
            .map(|s| Statement::new(&s.into()))
            .collect::<Result<_, _>>()?;
        Ok(Self { statements })
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
    fn parse_block_with_one_statement() {
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
    fn parse_block_with_one_expr() {
        assert_eq!(
            Block::new(&"{ a }".into()),
            Ok(Block {
                statements: vec![Statement::Expression(Expression::Binding(
                    Binding::new(&"a".into()).unwrap()
                ))]
            })
        );
    }
    #[test]
    fn parse_block_with_multiple_statements() {
        assert_eq!(
            Block::new(
                &"{ let a = 11451;
        let b = a;
        b}"
                .into()
            ),
            Ok(Block {
                statements: vec![
                    Statement::BindingDef(BindingDef::new(&"let a = 11451;".into()).unwrap()),
                    Statement::BindingDef(BindingDef::new(&"let b = a;".into()).unwrap()),
                    Statement::Expression(Expression::Binding(Binding::new(&"b".into()).unwrap()))
                ]
            })
        );
    }
    #[test]
    fn parse_block_with_one_line_but_multiple_statements() {
        assert_eq!(
            Block::new(&"{let a = 1;a}".into()),
            Ok(Block {
                statements: vec![
                    Statement::BindingDef(BindingDef::new(&"let a = 1;".into()).unwrap()),
                    Statement::Expression(Expression::Binding(Binding::new(&"a".into()).unwrap()))
                ]
            })
        );
    }
    #[test]
    fn parse_without_braces() {
        assert_eq!(
            Block::new(&"{let a = 11451;".into()),
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
        assert_eq!(
            Block::new(&"{let a = 11451}".into()),
            Err(Error::Statement(StatementError::InvalidStatement))
        );
    }
}
