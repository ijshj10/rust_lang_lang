use crate::env::Env;
use binding_usage::BindingUsage;
use block::Block;

use crate::{utils, val::Val};
pub mod binding_usage;
pub mod block;

#[cfg(test)]
mod tests {
    use crate::stmt::Stmt;
    use binding_usage::BindingUsage;

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
                    lhs: Box::new(Expr::Number(Number(1))),
                    op: Op::Add,
                    rhs: Box::new(Expr::Number(Number(2))),
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
                    lhs: Box::new(Expr::Number(Number(2))),
                    rhs: Box::new(Expr::Number(Number(2))),
                    op: Op::Mul,
                }
            ),)
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Number(Number(10))),
                op: Op::Add,
            }
            .eval(&Env::default()),
            Ok(Val::Number(20))
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(1))),
                rhs: Box::new(Expr::Number(Number(5))),
                op: Op::Sub,
            }
            .eval(&Env::default()),
            Ok(Val::Number(-4))
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(5))),
                rhs: Box::new(Expr::Number(Number(4))),
                op: Op::Mul,
            }
            .eval(&Env::default()),
            Ok(Val::Number(20))
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(1460))),
                rhs: Box::new(Expr::Number(Number(7))),
                op: Op::Div,
            }
            .eval(&Env::default()),
            Ok(Val::Number(208))
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
    
    #[test]
    fn eval_non_number_operation() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Block(Block { stmts: vec![] })),
                op: Op::Add,
            }.eval(&Env::default()),
            Err("cannot evaluate operation whose operands are not numbers".to_owned())
        )
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
pub(crate) enum Expr {
    Number(Number),
    Operation { lhs: Box<Self>, rhs: Box<Self>, op: Op },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s).or_else(|_| Self::new_non_operation(s))
    }

    fn new_non_operation(s: &str) -> Result<(&str, Self), String> {
        Self::new_number(s)
            .or_else(|_| Self::new_binding_usage(s))
            .or_else(|_| Self::new_block(s))
    }

    pub fn new_block(s: &str) -> Result<(&str, Self), String> {
        Block::new(s).map(|(s, block)| (s, Self::Block(block)))
    }

    pub fn new_binding_usage(s: &str) -> Result<(&str, Self), String> {
        BindingUsage::new(s).map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
    }
    pub fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Expr::new_non_operation(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Expr::new_non_operation(s)?;

        let expr = Self::Operation { lhs:Box::new(lhs), rhs:Box::new(rhs), op };
        Ok((s, expr))
    }

    pub fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Number(Number(number)) => Ok(Val::Number(*number)),
            Self::Operation { lhs, rhs, op } => {
                let lhs = lhs.eval(env)?;
                let rhs = rhs.eval(env)?;

                let (lhs, rhs) = match (lhs, rhs) {
                    (Val::Number(lhs), Val::Number(rhs)) => (lhs, rhs),
                    _ => return Err("cannot evaluate operation whose operands are not numbers".to_owned()),
                };


                let res = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                Ok(Val::Number(res))
            }

            Self::BindingUsage(binding_usage) => binding_usage.eval(&env),

            Self::Block(block) => block.eval(&env),
        }
    }
}
