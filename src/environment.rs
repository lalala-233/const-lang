use crate::internal::prelude::*;
use std::collections::HashMap;
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Environment {
    bindings: HashMap<Identifier, Expression>,
}

impl Environment {
    pub fn insert(&mut self, name: Identifier, expr: Expression) {
        self.bindings.insert(name, expr);
    }
    pub fn get(&self, name: &Identifier) -> Option<Expression> {
        self.bindings.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multiple_insert() {
        let mut env = Environment::default();
        BindingDef::new(&"let x = 11451;".into())
            .unwrap()
            .store(&mut env);
        assert_eq!(
            env.bindings.get(&"x".try_into().unwrap()),
            Some(&Expression::Number(Number::from_i32(11451)))
        );
        BindingDef::new(&"let x = 19198;".into())
            .unwrap()
            .store(&mut env);
        assert_eq!(
            env.bindings.get(&"x".try_into().unwrap()),
            Some(&Expression::Number(Number::from_i32(19198)))
        );
    }
}
