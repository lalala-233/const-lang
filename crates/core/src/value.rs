use crate::internal::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Number(Number),
    Empty,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number.inner()),
            Self::Empty => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod tests {}
