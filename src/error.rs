use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum BindingDefError {
    #[error("Expect `let` here")]
    MissingLetKeyword,
    #[error("Expect `=` here")]
    MissingEqualsSign,
    #[error(transparent)]
    Expression(#[from] ExpressionError),
    #[error(transparent)]
    Identifier(#[from] IdentifierError),
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
    #[error("Identifier must not contain whitespace")]
    ContainWhitespace,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExpressionError {
    #[error("Invalid expression")]
    InvalidExpression,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum StatementError {
    #[error("Invalid statement")]
    InvalidStatement,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum OperationError {
    #[error(transparent)]
    Number(#[from] NumberError),
    #[error("Operator is not found")]
    OperatorNotFound,
    #[error(transparent)]
    Operator(#[from] OperatorError),
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum NumberError {
    #[error("Invalid number")]
    InvalidNumber(#[from] ParseIntError),
}
