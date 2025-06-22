use crate::internal::prelude::*;
use std::collections::HashMap;
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Environment<'parent> {
    bindings: HashMap<Identifier, NamedValue>,
    parent: Option<&'parent Self>,
}

impl<'parent> Environment<'parent> {
    pub fn create_child(&'parent self) -> Self {
        Self {
            parent: Some(self),
            ..Default::default()
        }
    }
    pub fn insert_binding(&mut self, name: Identifier, expr: Expression) {
        self.bindings.insert(name, NamedValue::Binding(expr));
    }
    pub fn insert_function(
        &mut self,
        name: Identifier,
        parameters: Vec<Identifier>,
        body: Expression,
    ) {
        self.bindings
            .insert(name, NamedValue::Function { parameters, body });
    }
    pub fn get_binding_by(&self, name: &Identifier) -> Option<Expression> {
        self.bindings
            .get(name)
            .cloned()
            .and_then(NamedValue::into_expression)
            .or_else(|| self.parent.and_then(|parent| parent.get_binding_by(name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_binding_by() {
        let env = &mut Environment::default();
        env.insert_binding(
            "x".try_into().unwrap(),
            Expression::Number(Number::from_i32(11451)),
        );
        assert_eq!(
            env.get_binding_by(&"x".try_into().unwrap()),
            Some(Expression::Number(Number::from_i32(11451)))
        );
        env.insert_function(
            "add".try_into().unwrap(),
            vec!["x".try_into().unwrap(), "y".try_into().unwrap()],
            Expression::Operation(Operation::new(&"x + y".into()).unwrap()),
        );
        assert_eq!(env.get_binding_by(&"add".try_into().unwrap()), None);
    }
    #[test]
    fn get_from_parent() {
        let parent = &mut Environment::default();
        BindingDef::new(&"let x = 114".into())
            .unwrap()
            .store(parent);
        let child = &mut parent.create_child();
        assert_eq!(
            child.get_binding_by(&"x".try_into().unwrap()),
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
        BindingDef::new(&"let x = 514".into()).unwrap().store(child);
        assert_eq!(
            child.get_binding_by(&"x".try_into().unwrap()),
            Some(Expression::Number(Number::from_i32(514)))
        );
    }
    #[test]
    fn insert_multiple() {
        let env = &mut Environment::default();
        BindingDef::new(&"let x = 11451".into()).unwrap().store(env);
        assert_eq!(
            env.bindings.get(&"x".try_into().unwrap()),
            Some(&NamedValue::Binding(Expression::Number(Number::from_i32(
                11451
            ))))
        );
        BindingDef::new(&"let x = 19198".into()).unwrap().store(env);
        assert_eq!(
            env.bindings.get(&"x".try_into().unwrap()),
            Some(&NamedValue::Binding(Expression::Number(Number::from_i32(
                19198
            ))))
        );
        FunctionDef::new(&"fn x => 114+514".into())
            .unwrap()
            .store(env);
        assert_eq!(env.get_binding_by(&"x".try_into().unwrap()), None);
        assert_eq!(
            env.bindings.get(&"x".try_into().unwrap()),
            Some(&NamedValue::Function {
                parameters: vec![],
                body: Expression::Operation(Operation::new(&"114 + 514".into()).unwrap())
            })
        );
    }
    #[test]
    fn insert_function() {
        let env = &mut Environment::default();
        env.insert_function(
            "add".try_into().unwrap(),
            vec!["x".try_into().unwrap(), "y".try_into().unwrap()],
            Expression::Operation(Operation::new(&"x + y".into()).unwrap()),
        );
        assert_eq!(
            env.bindings.get(&"add".try_into().unwrap()),
            Some(&NamedValue::Function {
                parameters: vec!["x".try_into().unwrap(), "y".try_into().unwrap()],
                body: Expression::Operation(Operation::new(&"x + y".into()).unwrap())
            })
        );
    }
}
