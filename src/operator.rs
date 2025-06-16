use crate::internal::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
impl Operator {
    pub const OP_CHAR_LIST: [char; 4] = ['+', '-', '*', '/'];
    pub const LEN_OF_OP: usize = '+'.len_utf8();
    pub fn new(s: &TrimmedString) -> Result<Self, OperatorError> {
        match s.as_str() {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            _ => Err(OperatorError::InvalidOperator),
        }
    }
}
#[cfg(test)]
mod op_tests {
    use super::*;
    #[test]
    fn parse_add_op() {
        assert_eq!(Operator::new(&"+".into()), Ok(Operator::Add));
    }
    #[test]
    fn parse_sub_op() {
        assert_eq!(Operator::new(&"-".into()), Ok(Operator::Sub));
    }
    #[test]
    fn parse_mul_op() {
        assert_eq!(Operator::new(&"*".into()), Ok(Operator::Mul));
    }
    #[test]
    fn parse_div_op() {
        assert_eq!(Operator::new(&"/".into()), Ok(Operator::Div));
    }
    #[test]
    fn parse_invalid_operator() {
        assert_eq!(
            Operator::new(&"s".into()),
            Err(OperatorError::InvalidOperator)
        );
    }
}
