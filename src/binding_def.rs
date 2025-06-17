use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct BindingDef {
    name: Identifier,
    val: Expression,
}

impl BindingDef {
    pub fn new(s: &TrimmedStr) -> Result<Self, BindingDefError> {
        let s = s
            .strip_prefix("let ")
            .ok_or(BindingDefError::MissingLetKeyword)?;
        let (identifier, expr) = s
            .split_once('=')
            .ok_or(BindingDefError::MissingEqualsSign)?;

        Ok(Self {
            name: Identifier::new(identifier.into())?,
            val: Expression::new(&expr.into())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_binding_def_with_expr() {
        assert_eq!(
            BindingDef::new(&"let foo = 1 + 1".into()),
            Ok(BindingDef {
                name: Identifier::new("foo".into()).unwrap(),
                val: Expression::new(&"1+1".into()).unwrap()
            })
        );
    }
    #[test]
    fn parse_binding_def_with_value() {
        assert_eq!(
            BindingDef::new(&"let foo = 3".into()),
            Ok(BindingDef {
                name: Identifier::new("foo".into()).unwrap(),
                val: Expression::new(&"3".into()).unwrap()
            })
        );
    }
    #[test]
    fn parse_without_equal() {
        assert_eq!(
            BindingDef::new(&"let foo 1+1".into()),
            Err(BindingDefError::MissingEqualsSign)
        );
    }
    #[test]
    fn parse_without_let() {
        assert_eq!(
            BindingDef::new(&"good morning".into()),
            Err(BindingDefError::MissingLetKeyword)
        );
    }
    #[test]
    fn parse_invalid_binding_def() {
        assert_eq!(
            BindingDef::new(&"letdown=1+1".into()),
            Err(BindingDefError::MissingLetKeyword)
        );
    }
}
