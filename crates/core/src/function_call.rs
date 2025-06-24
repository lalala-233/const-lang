use crate::internal::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    name: Identifier,
    parameters: Vec<Expression>,
}

impl FunctionCall {
    pub fn new(s: &TrimmedStr) -> Result<Self, Error> {
        let mut parts = s.split_whitespace();
        let name = parts.next().ok_or(FunctionCallError::Empty)?.try_into()?;

        let parameters = parts
            .map(|s| Expression::new(&s.into()))
            .collect::<Result<_, _>>()?;
        Ok(Self { name, parameters })
    }
    pub fn try_get_expression_from(
        &self,
        local: &mut Environment,
    ) -> Result<Expression, FunctionCallError> {
        let Some(NamedValue::Function { parameters, body }) =
            local.get_from_self_and_parent(&self.name)
        else {
            return Err(FunctionCallError::NotFound);
        };
        if parameters.len() != self.parameters.len() {
            return Err(FunctionCallError::WrongParameterCount {
                expected: parameters.len(),
                got: self.parameters.len(),
            });
        }
        for (param, value) in parameters.iter().zip(&self.parameters) {
            local.insert_binding(param.clone(), value.clone());
        }
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_empty() {
        assert_eq!(
            FunctionCall::new(&"".into()),
            Err(Error::FunctionCall(FunctionCallError::Empty))
        );
    }
    #[test]
    fn parse_function_call_with_no_parameters() {
        assert_eq!(
            FunctionCall::new(&"foo".into()),
            Ok(FunctionCall {
                name: "foo".try_into().unwrap(),
                parameters: vec![]
            })
        );
    }
    #[test]
    fn parse_function_call_with_parameters() {
        assert_eq!(
            FunctionCall::new(&"add x y".into()),
            Ok(FunctionCall {
                name: "add".try_into().unwrap(),
                parameters: vec![
                    Expression::Binding("x".try_into().unwrap()),
                    Expression::Binding("y".try_into().unwrap())
                ]
            })
        );
    }
    #[test]
    fn try_get_expression_with_no_parameters() {
        let env = &mut Environment::default();
        FunctionDef::new(&"fn homo_number => 114".into())
            .unwrap()
            .store(env);
        assert_eq!(
            FunctionCall::new(&"homo_number".into())
                .unwrap()
                .try_get_expression_from(env),
            Ok(Expression::Number(Number::from_i32(114)))
        );
    }
    #[test]
    fn try_get_expression_with_parameters() {
        let env = &mut Environment::default();
        FunctionDef::new(&"fn add x y => x + y".into())
            .unwrap()
            .store(env);
        assert_eq!(
            FunctionCall::new(&"add 1 2".into())
                .unwrap()
                .try_get_expression_from(env),
            Ok(Expression::Operation(
                Operation::new(&"x+y".into()).unwrap()
            ))
        );
        assert_eq!(
            env.get_from_self_and_parent(&"x".try_into().unwrap()),
            Some(NamedValue::Binding(Expression::Number(Number::from_i32(1))))
        );
        assert_eq!(
            env.get_from_self_and_parent(&"y".try_into().unwrap()),
            Some(NamedValue::Binding(Expression::Number(Number::from_i32(2))))
        );
    }
    #[test]
    fn try_get_expression_with_non_existing_identifier() {
        let env = &mut Environment::default();
        assert_eq!(
            FunctionCall::new(&"non_existing".into())
                .unwrap()
                .try_get_expression_from(env),
            Err(FunctionCallError::NotFound)
        );
    }
    #[test]
    #[should_panic]
    fn try_get_expression_from_outer_scope() {
        let env = &mut Environment::default();
        let invalid_function_def = FunctionDef::new(&"fn sub => x - y".into()).unwrap();
        invalid_function_def.store(env);
        BindingDef::new(&"let x = 114".into()).unwrap().store(env);
        BindingDef::new(&"let y = 514".into()).unwrap().store(env);
        let call_invalid_function = FunctionCall::new(&"sub".into()).unwrap();
        // TODO: fix this because sub is a invalid function definition which use bindings from outer scope
        assert_eq!(
            call_invalid_function.try_get_expression_from(env),
            Ok(Expression::Operation(
                Operation::new(&"x - y".into()).unwrap()
            ))
        );
        assert_eq!(
            call_invalid_function
                .try_get_expression_from(env)
                .unwrap()
                .eval(env),
            Err(Error::Binding(BindingError::NotFound))
        );
        FunctionDef::new(&"fn sub x - y => x - y".into())
            .unwrap()
            .store(env);
        assert_eq!(
            FunctionCall::new(&"sub".into())
                .unwrap()
                .try_get_expression_from(env),
            Ok(Expression::Operation(
                Operation::new(&"x - y".into()).unwrap()
            ))
        );
        assert_eq!(
            FunctionCall::new(&"sub y x".into())
                .unwrap()
                .try_get_expression_from(env)
                .unwrap()
                .eval(env),
            Ok(Value::Number(Number::from_i32(514 - 114)))
        );
    }
}
