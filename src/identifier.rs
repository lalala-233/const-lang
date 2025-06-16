use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Identifier(NonWhiteSpaceString);

impl Identifier {
    pub fn new(s: NonWhiteSpaceString) -> Result<Self, IdentifierError> {
        s.starts_with(|c: char| c.is_alphabetic())
            .then_some(Self(s))
            .ok_or(IdentifierError::StartWithNonLetter)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_identifier() {
        assert_eq!(
            Identifier::new("foo123".into()),
            Ok(Identifier("foo123".into()))
        );
    }

    #[test]
    fn parse_invalid_identifier() {
        assert_eq!(
            Identifier::new("123foo".into()),
            Err(IdentifierError::StartWithNonLetter)
        );
    }
}
