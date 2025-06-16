use crate::internal::prelude::*;
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Number(i32);
impl Number {
    pub fn new(s: &TrimmedString) -> Result<Self, Error> {
        Ok(Self(s.parse()?))
    }
    pub const fn inner(self) -> i32 {
        self.0
    }
    pub const fn from_i32(value: i32) -> Self {
        Self(value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new(&"123".into()), Ok(Number(123)));
    }

    #[test]
    fn parse_non_number() {
        assert!(matches!(
            Number::new(&"non-number".into()),
            Err(Error::InvalidNumber(_))
        ));
    }
}
