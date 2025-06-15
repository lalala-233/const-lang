use crate::internal::prelude::*;
#[derive(PartialEq, Eq, Debug)]
pub struct Number(i32);
impl Number {
    pub fn new(s: &NonWhiteSpaceString) -> Self {
        Self(s.parse().unwrap())
    }
    pub const fn inner(&self)->i32{
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new(&"123".into()), Number(123));
    }
}
