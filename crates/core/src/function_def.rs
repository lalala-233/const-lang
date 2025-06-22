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
        let (name, rest) = s
            .split_once(' ')
            .ok_or(FunctionDefError::InvalidFunctionDef)?;
        let (parameters, body) = rest
            .split_once("=>")
            .ok_or(FunctionDefError::MissingArrow)?;
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
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_function_def_with_no_parameters() {
        assert_eq!(
            FunctionDef::new(&"fn nothing => {}".into()),
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
            FunctionDef::new(&"fn foo x => x ".into()),
            Ok(FunctionDef {
                name: "foo".try_into().unwrap(),
                parameters: vec!["x".try_into().unwrap()],
                body: Expression::Binding(Identifier::new(&"x".into()).unwrap()),
            })
        );
    }
    #[test]
    fn parse_function_def_with_multiple_parameter() {
        assert_eq!(
            FunctionDef::new(&"fn add x y => x + y ".into()),
            Ok(FunctionDef {
                name: "add".try_into().unwrap(),
                parameters: vec!["x".try_into().unwrap(), "y".try_into().unwrap()],
                body: Expression::Operation(Operation::new(&"x + y".into()).unwrap()),
            })
        );
    }
}
