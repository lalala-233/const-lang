use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier(NonWhiteSpaceString);

impl Identifier {
    pub fn new(s: NonWhiteSpaceString) -> Self {
        if s.starts_with(|c: char| c.is_alphabetic()) {
            Self(s)
        } else {
            panic!("Identifier must start with a letter")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_identifier() {
        assert_eq!(
            Identifier::new("foo123".into()),
            Identifier("foo123".into())
        );
    }

    #[test]
    #[should_panic = "Identifier must start with a letter"]
    fn parse_invalid_identifier() {
        Identifier::new("123foo".into());
    }
}
