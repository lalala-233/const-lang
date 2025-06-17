use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BindingDef {
    name: Identifier,
    expr: Expression,
}

impl BindingDef {
    pub fn new(s: &TrimmedStr) -> Result<Self, BindingDefError> {
        let s = s
            .strip_prefix("let ")
            .ok_or(BindingDefError::MissingLetKeyword)?
            .strip_suffix(';')
            .ok_or(BindingDefError::MissingSemicolon)?;
        let (identifier, expr) = s
            .split_once('=')
            .ok_or(BindingDefError::MissingEqualsSign)?;

        Ok(Self {
            name: Identifier::new(&identifier.into())?,
            expr: Expression::new(&expr.into())?,
        })
    }
    pub const fn name(&self) -> &Identifier {
        &self.name
    }
    pub const fn expr(&self) -> &Expression {
        &self.expr
    }
    pub fn store(&self, env: &mut Environment) {
        env.insert(self.name.clone(), self.expr.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_binding_def_with_expr() {
        assert_eq!(
            BindingDef::new(&"let foo = 1 + 1;".into()),
            Ok(BindingDef {
                name: Identifier::new(&"foo".into()).unwrap(),
                expr: Expression::new(&"1+1".into()).unwrap()
            })
        );
    }
    #[test]
    fn parse_binding_def_with_value() {
        assert_eq!(
            BindingDef::new(&"let foo = 3;".into()),
            Ok(BindingDef {
                name: Identifier::new(&"foo".into()).unwrap(),
                expr: Expression::Number(Number::from_i32(3))
            })
        );
    }
    #[test]
    fn parse_without_equal() {
        assert_eq!(
            BindingDef::new(&"let foo 1+1;".into()),
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
    #[test]
    fn parse_without_semicolon() {
        assert_eq!(
            BindingDef::new(&"let foo = 1+1".into()),
            Err(BindingDefError::MissingSemicolon)
        );
    }
}
