use crate::internal::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamedValue {
    Binding(Expression),
    Function {
        parameters: Vec<Identifier>,
        body: Expression,
    },
}

impl NamedValue {
    pub fn into_expression(self) -> Option<Expression> {
        match self {
            Self::Binding(expr) => Some(expr),
            Self::Function { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_expression() {
        let binding = NamedValue::Binding(Expression::Number(Number::from_i32(42)));
        assert_eq!(
            binding.into_expression(),
            Some(Expression::Number(Number::from_i32(42)))
        );

        let function = NamedValue::Function {
            parameters: vec!["x".try_into().unwrap()],
            body: Expression::Operation(Operation::new(&"2 + x".into()).unwrap()),
        };
        assert_eq!(function.into_expression(), None);
    }
}
