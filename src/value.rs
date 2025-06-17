use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Number(Number),
    Empty,
}

#[cfg(test)]
mod tests {}
