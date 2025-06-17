use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq)]
/// Every valid identifier is a valid binding.
pub struct Binding {
    binding: Identifier,
}
impl Binding {
    pub fn new(binding: &TrimmedStr) -> Result<Self, BindingError> {
        let binding = Identifier::new(binding)?;
        Ok(Self { binding })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_binding() {
        assert_eq!(
            Binding::new(&"foo".into()),
            Ok(Binding {
                binding: Identifier::new(&"foo".into()).unwrap()
            })
        );
    }
    #[test]
    fn parse_binding_error() {
        assert!(matches!(
            Binding::new(&"114514abc".into()),
            Err(BindingError::Identifier(
                IdentifierError::StartWithNonLetter
            ))
        ));
    }
    #[test]
    fn parse_empty() {
        assert!(matches!(
            Binding::new(&"".into()),
            Err(BindingError::Identifier(IdentifierError::Empty))
        ));
    }
}
