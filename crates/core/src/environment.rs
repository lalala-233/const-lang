use crate::internal::prelude::*;
use std::collections::HashMap;
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Environment<'parent> {
    bindings: HashMap<Identifier, Expression>,
    parent: Option<&'parent Self>,
}

impl<'parent> Environment<'parent> {
    pub fn create_child(&'parent self) -> Self {
        Self {
            parent: Some(self),
            ..Default::default()
        }
    }
    pub fn insert(&mut self, name: Identifier, expr: Expression) {
        self.bindings.insert(name, expr);
    }
    pub fn get(&self, name: &Identifier) -> Option<Expression> {
        self.bindings
            .get(name)
            .cloned()
            .or_else(|| self.parent.and_then(|parent| parent.get(name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get() {
        let env = &mut Environment::default();
        env.insert(
            "x".try_into().unwrap(),
            Expression::Number(Number::from_i32(11451)),
        );
        assert_eq!(
            env.get(&"x".try_into().unwrap()),
            Some(Expression::Number(Number::from_i32(11451)))
        );
    }
    #[test]
    fn get_from_parent() {
        let parent = &mut Environment::default();
        BindingDef::new(&"let x = 114".into())
            .unwrap()
            .store(parent);
        let child = &mut parent.create_child();
        assert_eq!(
            child.get(&"x".try_into().unwrap()),
            Some(Expression::Number(Number::from_i32(114)))
        );
    }
    #[test]
    fn get_when_parent_have_same_identifier() {
        let parent = &mut Environment::default();
        BindingDef::new(&"let x = 114".into())
            .unwrap()
            .store(parent);
        let child = &mut parent.create_child();
        BindingDef::new(&"let x = 514".into())
            .unwrap()
            .store(child);
        assert_eq!(
            child.get(&"x".try_into().unwrap()),
            Some(Expression::Number(Number::from_i32(514)))
        );
    }
    #[test]
    fn multiple_insert() {
        let env = &mut Environment::default();
        BindingDef::new(&"let x = 11451".into()).unwrap().store(env);
        assert_eq!(
            env.bindings.get(&"x".try_into().unwrap()),
            Some(&Expression::Number(Number::from_i32(11451)))
        );
        BindingDef::new(&"let x = 19198".into()).unwrap().store(env);
        assert_eq!(
            env.bindings.get(&"x".try_into().unwrap()),
            Some(&Expression::Number(Number::from_i32(19198)))
        );
    }
}
