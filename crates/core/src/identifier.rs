use std::str::FromStr;

use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier(TrimmedString);

impl Identifier {
    pub fn new(s: &TrimmedStr) -> Result<Self, IdentifierError> {
        if s.is_empty() {
            return Err(IdentifierError::Empty);
        }
        if !s.starts_with(char::is_alphabetic) {
            return Err(IdentifierError::StartWithNonLetter);
        }
        if s.chars().any(|c| !c.is_alphanumeric() && c != '_') {
            return Err(IdentifierError::ContainSpecialCharacters);
        }
        // TODO: more checks like ops, keywords, etc.
        Ok(Self(s.into()))
    }
}
impl FromStr for Identifier {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(&s.into())
    }
}
impl TryFrom<&str> for Identifier {
    type Error = IdentifierError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(&s.into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_identifier() {
        assert_eq!(
            Identifier::new(&"foo123".into()),
            Ok(Identifier("foo123".into()))
        );
    }
    #[test]
    fn parse_with_allowed_character() {
        assert_eq!(
            Identifier::new(&"foo_123_中文".into()),
            Ok(Identifier("foo_123_中文".into()))
        );
    }
    #[test]
    fn parse_start_with_number() {
        assert_eq!(
            Identifier::new(&"123foo".into()),
            Err(IdentifierError::StartWithNonLetter)
        );
    }
    #[test]
    fn parse_contain_whitespace() {
        assert_eq!(
            Identifier::new(&"foo bar".into()),
            Err(IdentifierError::ContainSpecialCharacters)
        );
    }
    #[test]
    fn parse_contain_crlf() {
        assert_eq!(
            Identifier::new(&"foo\n b\rar".into()),
            Err(IdentifierError::ContainSpecialCharacters)
        );
    }
    #[test]
    fn parse_contain_special_characters() {
        assert_eq!(
            Identifier::new(&"foo=bar".into()),
            Err(IdentifierError::ContainSpecialCharacters)
        );
    }
    #[test]
    fn parse_empty() {
        assert_eq!(Identifier::new(&"".into()), Err(IdentifierError::Empty));
    }
    #[test]
    fn parse_whitespace() {
        assert_eq!(
            Identifier::new(&"         \n".into()),
            Err(IdentifierError::Empty)
        );
    }
}
