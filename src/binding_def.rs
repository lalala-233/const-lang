use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct BindingDef {
    name: Identifier,
    val: Operation,
}

impl BindingDef {
    fn new(s: &str) -> Self {
        let s = s.strip_prefix("let").expect("Expect let");
        let (identifier, expr) = s.split_once('=').unwrap();

        Self {
            name: Identifier::new(identifier.into()),
            val: Operation::new(expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let foo = 1 + 1"),
            BindingDef {
                name: Identifier::new("foo".into()),
                val: Operation::new("1+1")
            }
        );
    }
}
