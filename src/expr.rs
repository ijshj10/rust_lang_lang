use crate::{utils, val::Val};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), ("", Op::Add));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), ("", Op::Sub));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), ("", Op::Mul));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), ("", Op::Div));
    }
    #[test]
    #[should_panic(expected = "bad operator")]
    fn parse_wrong_op() {
        Op::new("_");
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            (
                "",
                Expr {
                    lhs: Number(1),
                    op: Op::Add,
                    rhs: Number(2),
                }
            )
        );
    }

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            (
                "",
                Expr {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mul,
                },
            )
        );
    }

    #[test]
    fn eval_add() {
        let (_, expr) = Expr::new("3+4");
        assert_eq!(expr.eval(), Val::Number(7));
    }

    #[test]
    fn eval_sub() {
        let (_, expr) = Expr::new("26-5");
        assert_eq!(expr.eval(), Val::Number(21));
    }

    #[test]
    fn eval_mul() {
        let (_, expr) = Expr::new("4*365");
        assert_eq!(expr.eval(), Val::Number(1460));
    }

    #[test]
    fn eval_div() {
        let (_, expr) = Expr::new("1460/7");
        assert_eq!(expr.eval(), Val::Number(208));
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, digits) = utils::extract_digits(s);
        let digits = digits.parse().unwrap();
        (s, Self(digits))
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = utils::extract_op(s);
        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("bad operator"),
        };
        (s, op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s);

        let expr = Self { lhs, rhs, op };
        (s, expr)
    }

    pub(crate) fn eval(&self) -> Val {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;

        let res = match self.op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        };

        Val::Number(res)
    }
}
