use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    fn new(s: &str) -> Self {
        let s = s.strip_prefix("let").expect("Expect let");
        let (lhs, rhs) = s.split_once('=').unwrap();

        Self {
            name: lhs.trim().to_string(),
            val: Expr::new(rhs),
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
                name: "foo".to_string(),
                val: Expr::new("1+1")
            }
        );
    }
}
