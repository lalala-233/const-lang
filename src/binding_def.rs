use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct BindingDef {
    name: Identifier,
    val: Expression,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<Self, BindingDefError> {
        let s = s
            .strip_prefix("let ")
            .ok_or(BindingDefError::MissingLetKeyword)?;
        let (identifier, expr) = s
            .split_once('=')
            .ok_or(BindingDefError::MissingEqualsSign)?;

        Ok(Self {
            name: Identifier::new(identifier.into())?,
            val: Expression::new(expr)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_binding_def_with_expr() {
        assert_eq!(
            BindingDef::new("let foo = 1 + 1"),
            Ok(BindingDef {
                name: Identifier::new("foo".into()).unwrap(),
                val: Expression::new("1+1").unwrap()
            })
        );
    }
    #[test]
    fn parse_binding_def_with_value() {
        assert_eq!(
            BindingDef::new("let foo = 3"),
            Ok(BindingDef {
                name: Identifier::new("foo".into()).unwrap(),
                val: Expression::new("3").unwrap()
            })
        );
    }
    #[test]
    fn parse_without_equal() {
        assert_eq!(
            BindingDef::new("let foo 1+1"),
            Err(BindingDefError::MissingEqualsSign)
        );
    }
    #[test]
    fn parse_without_let() {
        assert_eq!(
            BindingDef::new("good morning"),
            Err(BindingDefError::MissingLetKeyword)
        );
    }
    #[test]
    fn parse_invalid_binding_def() {
        assert_eq!(
            BindingDef::new("letdown=1+1"),
            Err(BindingDefError::MissingLetKeyword)
        );
    }
}
