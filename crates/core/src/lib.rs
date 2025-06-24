mod binding_def;
mod block;
mod environment;
mod error;
mod expression;
mod function_call;
mod function_def;
mod identifier;
mod named_value;
mod number;
mod operation;
mod operator;
pub mod parser;
mod statement;
mod trimmed_str;
mod value;

pub use parser::Parser;

mod internal {
    pub mod prelude {
        pub use crate::{
            binding_def::BindingDef, block::Block, environment::Environment, error::*,
            expression::Expression, function_call::FunctionCall, function_def::FunctionDef,
            identifier::Identifier, named_value::NamedValue, number::Number, operation::Operation,
            operator::Operator, statement::Statement, trimmed_str::TrimmedStr, value::Value,
        };
    }
}
