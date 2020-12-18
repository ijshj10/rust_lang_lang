#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }

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
    #[test]
    #[should_panic(expected = "bad operator")]
    fn parse_wrong_op() {
        Op::new("_");
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(Expr::new("1+2"),
            Expr{
                lhs: Number(1),
                op: Op::Add,
                rhs: Number(2),
            }
        );
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("bad operator")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> Self {

    }
}