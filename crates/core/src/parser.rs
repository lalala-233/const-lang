use crate::internal::prelude::*;
#[derive(Default)]
pub struct Parser<'a> {
    environment: Environment<'a>,
}

impl Parser<'_> {
    /// # Errors
    ///
    /// This function will return an error if s cannot be parsed to a valid statement.
    pub fn parse(&mut self, s: &str) -> Result<String, String> {
        let value = Statement::new(&s.into())
            .map_err(|err| err.to_string())?
            .get_expression_in(&mut self.environment)
            .eval(&self.environment)
            .map_err(|err| err.to_string())?;
        Ok(value.to_string())
    }
}
