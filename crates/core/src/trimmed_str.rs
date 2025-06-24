use std::ops::Deref;

/// Store non-empty &str
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TrimmedStr<'a>(&'a str);
impl<'a> TrimmedStr<'a> {
    pub fn new(s: &'a str) -> Self {
        let s = s.trim();
        Self(s)
    }
}
impl Deref for TrimmedStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> From<&'a str> for TrimmedStr<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn non_white_space_string() {
        assert_eq!(TrimmedStr::new("   H e l l o  "), TrimmedStr("H e l l o"));
    }
}
