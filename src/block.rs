use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    statements: Vec<Statement>,
}
impl Block {
    fn new(s: &TrimmedStr) -> Result<Self, StatementError> {
        if let Some(s) = s.strip_prefix('{') {
            if let Some(s) = s.strip_suffix('}') {
                let statements = s
                    .trim()
                    .lines()
                    .map(|s| Statement::new(&s.into()))
                    .collect::<Result<_, _>>()?;
                return Ok(Self { statements });
            }
        }
        Err(StatementError::InvalidStatement)
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
}
