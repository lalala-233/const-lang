use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    statements: Vec<Statement>,
}
impl Block {
    fn new(s: &TrimmedStr) -> Result<Self, StatementError> {
        let statements = s
            .lines()
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
}
