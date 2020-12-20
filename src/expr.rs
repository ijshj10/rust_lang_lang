use binding_usage::BindingUsage;
use block::Block;

use crate::{utils, val::Val};
pub mod binding_usage;
pub mod block;

#[cfg(test)]
mod tests {
    use binding_usage::BindingUsage;
    use crate::stmt::Stmt;

    use super::*;
    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Ok(("", Op::Add)));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Ok(("", Op::Mul)));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
    }
    #[test]
    fn parse_wrong_op() {
        assert_eq!(Op::new("_"), Err("expected /".to_owned()));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Number(1),
                    op: Op::Add,
                    rhs: Number(2),
                }
            ))
        );
    }

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mul,
                }
            ),)
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Add,
            }
            .eval(),
            Val::Number(20)
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(1),
                rhs: Number(5),
                op: Op::Sub,
            }
            .eval(),
            Val::Number(-4)
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(5),
                rhs: Number(4),
                op: Op::Mul,
            }
            .eval(),
            Val::Number(20)
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(1460),
                rhs: Number(7),
                op: Op::Div,
            }
            .eval(),
            Val::Number(208)
        );
    }

    #[test]
    fn parse_number_as_expr() {
        assert_eq!(Expr::new("3"), Ok(("", Expr::Number(Number(3)))));
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::new("bar"),
            Ok((
                "",
                Expr::BindingUsage(BindingUsage {
                    name: "bar".to_owned()
                })
            ))
        )
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expr::new("{ 200 }"),
            Ok((
                "",
                Expr::Block(Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(200)))],
                }),
            )),
        );
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, digits) = utils::extract_digits(s)?;
        let digits = digits.parse().unwrap();
        Ok((s, Self(digits)))
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
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
    }
}
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Number),
    Operation { lhs: Number, rhs: Number, op: Op },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_block(s)
            .or_else(|_| Self::new_binding_usage(s))
            .or_else(|_| Self::new_operation(s))
            .or_else(|_| Self::new_number(s))
    }

    pub fn new_block(s: &str) -> Result<(&str, Self), String> {
        Block::new(s)
            .map(|(s, block)| (s, Self::Block(block)))
    }

    pub fn new_binding_usage(s: &str) -> Result<(&str, Self), String> {
        BindingUsage::new(s)
            .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
    }
    pub fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s)?;

        let expr = Self::Operation { lhs, rhs, op };
        Ok((s, expr))
    }

    pub fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub(crate) fn eval(&self) -> Val {
        match self {
            Self::Number(Number(number)) => Val::Number(*number),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let res = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                Val::Number(res)
            },
            
            Self::BindingUsage(binding_usage) => {
                Val::Number(2)
            },
            
            _ => todo!(),
        }
    }
}
