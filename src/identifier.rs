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
        if s.chars().any(char::is_whitespace) {
            return Err(IdentifierError::ContainWhitespace);
        }
        // TODO: more checks like ops, keywords, etc.
        Ok(Self(s.into()))
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
            Err(IdentifierError::ContainWhitespace)
        );
    }
    #[test]
    fn parse_contain_crlf() {
        assert_eq!(
            Identifier::new(&"foo\n b\rar".into()),
            Err(IdentifierError::ContainWhitespace)
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
