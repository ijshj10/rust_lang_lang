use crate::expr::Expr;
use crate::{binding_def::BindingDef, env::Env, val::Val};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};
    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Stmt::new("let x = 3"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "x".to_owned(),
                    val: Expr::Number(Number(3)),
                })
            ))
        );
    }

    #[test]
    fn parse_expression() {
        assert_eq!(
            Stmt::new("1+1"),
            Ok((
                "",
                Stmt::Expr(Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(1),
                    op: Op::Add
                })
            ))
        );
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    BindingDef(BindingDef),
    Expr(Expr),
}

impl Stmt {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Stmt::BindingDef(binding_def) => binding_def.eval(env).map(|_| Val::Unit),
            Stmt::Expr(expr) => expr.eval(env),
        }
    }
}