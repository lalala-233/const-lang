use std::ops::Deref;

/// Store non-empty String
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TrimmedString(String);
impl TrimmedString {
    fn new(s: &str) -> Self {
        let s = s.trim();
        Self(s.to_string())
    }
    fn inner(self) -> String {
        self.0
    }
}
impl Deref for TrimmedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<&str> for TrimmedString {
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
            TrimmedString::new("   H e l l o  "),
            TrimmedString("H e l l o".to_string())
        );
    }
}
