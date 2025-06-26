use std::str::FromStr;

use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier {
    name: TrimmedString,
}

impl Identifier {
    pub fn new(s: &TrimmedStr) -> Result<Self, IdentifierError> {
        if s.is_empty() {
            return Err(IdentifierError::Empty);
        }
        if !s.starts_with(unicode_ident::is_xid_start) {
            return Err(IdentifierError::StartWithNonLetter);
        }
        if s.chars().all(unicode_ident::is_xid_continue) {
            Ok(Self { name: s.into() })
        } else {
            Err(IdentifierError::ContainSpecialCharacters)
        }
        // TODO: more checks like ops, keywords, etc.
    }
    pub fn try_get_expression_from(&self, env: &Environment) -> Result<Expression, BindingError> {
        env.get_from_self_and_parent(self)
            .and_then(NamedValue::into_expression)
            .ok_or(BindingError::NotFound)
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
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct TrimmedString(String);
impl From<&TrimmedStr<'_>> for TrimmedString {
    fn from(value: &TrimmedStr<'_>) -> Self {
        Self(value.to_string())
    }
}
impl From<&str> for TrimmedString {
    fn from(value: &str) -> Self {
        let trimmed_str = &TrimmedStr::new(value);
        trimmed_str.into()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_identifier() {
        assert_eq!(
            Identifier::new(&"foo123".into()),
            Ok(Identifier {
                name: "foo123".into()
            })
        );
    }
    #[test]
    fn parse_with_allowed_character() {
        assert_eq!(
            Identifier::new(&"foo_123_中文".into()),
            Ok(Identifier {
                name: "foo_123_中文".into()
            })
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
    // Test from binding
    #[test]
    fn get_expression_with_existing_identifier() {
        let env = &mut Environment::default();
        BindingDef::new(&"let foo = 11451".into())
            .unwrap()
            .store(env);
        assert_eq!(
            Identifier::new(&"foo".into())
                .unwrap()
                .try_get_expression_from(env),
            Ok(Expression::Number(Number::from_i32(11451)))
        );
    }
    #[test]
    fn get_expression_with_non_existing_identifier() {
        let env = Environment::default();
        assert_eq!(
            Identifier::new(&"foo".into())
                .unwrap()
                .try_get_expression_from(&env),
            Err(BindingError::NotFound)
        );
    }
}
