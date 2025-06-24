use crate::internal::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDef {
    name: Identifier,
    parameters: Vec<Identifier>,
    body: Expression,
}
impl FunctionDef {
    pub fn new(s: &TrimmedStr) -> Result<Self, Error> {
        let Some(s) = s.strip_prefix("fn ") else {
            return Err(FunctionDefError::MissingFnKeyword)?;
        };
        let (rest, body) = s.split_once("=>").ok_or(FunctionDefError::MissingArrow)?;
        let (name, parameters) = rest.trim().split_once(' ').unwrap_or((rest, ""));
        let parameters = parameters
            .split_whitespace()
            .filter_map(|s| s.try_into().ok())
            .collect();
        let body = Expression::new(&body.into())?;
        Ok(Self {
            name: name.try_into()?,
            parameters,
            body,
        })
    }
    pub fn store(&self, env: &mut Environment) {
        env.insert_function(
            self.name.clone(),
            self.parameters.clone(),
            self.body.clone(),
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_function_def_with_no_parameters() {
        assert_eq!(
            FunctionDef::new(&"fn nothing=>{}".into()),
            Ok(FunctionDef {
                name: "nothing".try_into().unwrap(),
                parameters: vec![],
                body: Expression::Block(Block::new(&"{}".into()).unwrap()),
            })
        );
    }
    #[test]
    fn parse_function_def_with_one_parameter() {
        assert_eq!(
            FunctionDef::new(&"fn foo x=>x ".into()),
            Ok(FunctionDef {
                name: "foo".try_into().unwrap(),
                parameters: vec!["x".try_into().unwrap()],
                body: Expression::Binding(Identifier::new(&"x".into()).unwrap()),
            })
        );
    }
    #[test]
    fn parse_function_def_with_parameter() {
        assert_eq!(
            FunctionDef::new(&"fn add x y=>x + y ".into()),
            Ok(FunctionDef {
                name: "add".try_into().unwrap(),
                parameters: vec!["x".try_into().unwrap(), "y".try_into().unwrap()],
                body: Expression::Operation(Operation::new(&"x + y".into()).unwrap()),
            })
        );
    }
    #[test]
    fn parse_invalid_function_def() {
        assert_eq!(
            FunctionDef::new(&"fn invalid".into()),
            Err(Error::FunctionDef(FunctionDefError::MissingArrow))
        );
        assert_eq!(
            FunctionDef::new(&"invalid fn x => x".into()),
            Err(Error::FunctionDef(FunctionDefError::MissingFnKeyword))
        );
        // TODO: fix it because x is undefined
        assert_eq!(
            FunctionDef::new(&"invalid fn => x".into()),
            Err(Error::FunctionDef(FunctionDefError::MissingFnKeyword))
        );
    }
}
