use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error(transparent)]
    FunctionCall(#[from] FunctionCallError),
    #[error(transparent)]
    FunctionDef(#[from] FunctionDefError),
    #[error(transparent)]
    Statement(#[from] StatementError),
    #[error(transparent)]
    Expression(#[from] ExpressionError),
    #[error(transparent)]
    Identifier(#[from] IdentifierError),
    #[error(transparent)]
    Binding(#[from] BindingError),
    #[error(transparent)]
    Number(#[from] NumberError),
    #[error(transparent)]
    Operator(#[from] OperatorError),
    #[error(transparent)]
    Operation(#[from] OperationError),
    #[error(transparent)]
    BindingDef(#[from] BindingDefError),
    #[error(transparent)]
    Block(#[from] BlockError),
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum BindingDefError {
    #[error("Expect `let` here")]
    MissingLetKeyword,
    #[error("Expect `=` here")]
    MissingEqualsSign,
    // #[error("Invalid binding definition")]
    // InvalidBindingDef
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
    #[error("Identifier must not contain special characters")]
    ContainSpecialCharacters,
    #[error("Identifier must not be empty")]
    Empty,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExpressionError {
    #[error("Invalid expression")]
    InvalidExpression,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum StatementError {
    #[error("Expect `;` here")]
    BindingDefMissingSemicolon,
    #[error("Invalid statement")]
    InvalidStatement,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum OperationError {
    #[error("Operator is not found")]
    NotFound,
    #[error("Expect a number in the left-hand side")]
    InvalidLhs,
    #[error("Expect a number in the right-hand side")]
    InvalidRhs,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum NumberError {
    #[error("Invalid number")]
    InvalidNumber(#[from] ParseIntError),
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum BindingError {
    #[error("Binding is not found")]
    NotFound,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum BlockError {
    #[error("Missing opening brace `{{`")]
    MissingOpeningBrace,
    #[error("Missing closing brace `}}`")]
    MissingClosingBrace,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum FunctionDefError {
    #[error("Expect `fn` here")]
    MissingFnKeyword,
    // #[error("Invalid function definition")]
    // InvalidFunctionDef,
    #[error("Expect `=>` here")]
    MissingArrow,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum FunctionCallError {
    #[error("Function call is not found")]
    NotFound,
    #[error("Wrong parameter count, expected {expected}, got {got}")]
    WrongParameterCount { expected: usize, got: usize },
    #[error("Expect a function call here")]
    Empty,
}
