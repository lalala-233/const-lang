use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Invalid number")]
    InvalidNumber(#[from] ParseIntError),
    #[error("Operator is not found")]
    OpNotFound,
    #[error(transparent)]
    Identifier(#[from] IdentifierError),
    #[error(transparent)]
    Operator(#[from] OperatorError),
    #[error(transparent)]
    BindingDef(#[from] BindingDefError),
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum BindingDefError {
    #[error("Expect `let` here")]
    MissingLetKeyword,
    #[error("Expect `=` here")]
    MissingEqualsSign,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum OperatorError {
    #[error("Invalid operator")]
    InvalidOperator,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum IdentifierError {
    #[error("Identifier must start with a letter")]
    StartWithNonLetter,
}
