mod binding;
mod binding_def;
mod block;
mod environment;
mod error;
mod expression;
mod function_def;
mod identifier;
mod number;
mod operation;
mod operator;
pub mod parser;
mod statement;
mod trimmed_string;
mod value;

pub use parser::Parser;

mod internal {
    pub mod prelude {
        pub use crate::{
            binding::Binding,
            binding_def::BindingDef,
            block::Block,
            environment::Environment,
            error::*,
            expression::Expression,
            function_def::FunctionDef,
            identifier::Identifier,
            number::Number,
            operation::Operation,
            operator::Operator,
            statement::Statement,
            trimmed_string::{TrimmedStr, TrimmedString},
            value::Value,
        };
    }
}
