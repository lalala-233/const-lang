#[derive(PartialEq, Eq, Debug)]
pub struct Number(pub i32);
impl Number {
    fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }
}
