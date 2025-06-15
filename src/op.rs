#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl Op {
    pub const OP_CHAR_LIST: [char; 4] = ['+', '-', '*', '/'];
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Invalid operator."),
        }
    }
}
#[cfg(test)]
mod op_tests {
    use super::*;
    
    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Op::Add);
    }
    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Op::Sub);
    }
    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Op::Mul);
    }
    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Op::Div);
    }
}
