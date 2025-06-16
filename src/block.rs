use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    exprs: Vec<Expr>,
}
impl Block {
    fn new(s: &str) -> Result<Self, Error> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(Block { exprs: Vec::new() }));
    }
}
