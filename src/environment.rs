use crate::internal::prelude::*;
use std::collections::HashMap;

pub struct Environment {
    bindings: HashMap<Identifier, Value>,
}

impl Environment {
    pub fn insert(&mut self, id: Identifier, value: Value) {
        self.bindings.insert(id, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
