use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
/// Every valid identifier is a valid binding.
pub struct Binding {
    name: Identifier,
}
impl Binding {
    pub fn new(name: &TrimmedStr) -> Result<Self, Error> {
        let name = Identifier::new(name)?;
        Ok(Self { name })
    }
    pub fn get_from(&self, env: &Environment) -> Result<Expression, BindingError> {
        env.get(&self.name).ok_or(BindingError::BindingNotFound)
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
                name: "foo".try_into().unwrap()
            })
        );
    }
    #[test]
    fn parse_binding_error() {
        assert!(matches!(
            Binding::new(&"114514abc".into()),
            Err(Error::Identifier(IdentifierError::StartWithNonLetter))
        ));
    }
    #[test]
    fn parse_empty() {
        assert!(matches!(
            Binding::new(&"".into()),
            Err(Error::Identifier(IdentifierError::Empty))
        ));
    }
    #[test]
    fn eval_existing_binding() {
        let mut env = Environment::default();
        BindingDef::new(&"let foo = 11451;".into())
            .unwrap()
            .store(&mut env);
        assert_eq!(
            Binding::new(&"foo".into()).unwrap().get_from(&env),
            Ok(Expression::Number(Number::from_i32(11451)))
        );
    }
    #[test]
    fn eval_non_exist_binding() {
        let env = Environment::default();
        assert_eq!(
            Binding::new(&"foo".into()).unwrap().get_from(&env),
            Err(BindingError::BindingNotFound)
        );
    }
}
