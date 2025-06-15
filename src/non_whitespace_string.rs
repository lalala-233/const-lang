use std::ops::Deref;

/// Store non-empty String
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NonWhiteSpaceString(String);
impl NonWhiteSpaceString {
    fn new(s: &str) -> Self {
        let s = s.chars().filter(|c| !c.is_whitespace()).collect();
        Self(s)
    }
    fn inner(self) -> String {
        self.0
    }
}
impl Deref for NonWhiteSpaceString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<&str> for NonWhiteSpaceString {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn non_white_space_string() {
        assert_eq!(
            NonWhiteSpaceString::new(" H e l l o "),
            NonWhiteSpaceString("Hello".to_string())
        );
    }
}
