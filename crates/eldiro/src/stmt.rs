use crate::expr::{BindingUsage, Expr};
use crate::func_def::FuncDef;
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
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs: Box::new(Expr::Number(Number(1))),
                    op: Op::Add
                })
            ))
        );
    }

    #[test]
    fn parse_func_def() {
        assert_eq!(
            Stmt::new("fn identity x => x"),
            Ok((
                "",
                Stmt::FuncDef(FuncDef {
                    name: "identity".to_owned(),
                    params: vec!["x".to_owned()],
                    body: Box::new(Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "x".to_owned()
                    })))
                })
            ))
        );
    }

    #[test]
    fn eval_func_def() {
        assert_eq!(
            Stmt::FuncDef(FuncDef {
                name: "always_return_one".to_owned(),
                params: vec![],
                body: Box::new(Stmt::Expr(Expr::Number(Number(1))))
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit)
        );
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Stmt {
    BindingDef(BindingDef),
    Expr(Expr),
    FuncDef(FuncDef),
}

impl Stmt {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        FuncDef::new(s)
            .map(|(s, func_def)| (s, Self::FuncDef(func_def)))
            .or_else(|_| {
                BindingDef::new(s).map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            })
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Stmt::BindingDef(binding_def) => binding_def.eval(env).map(|_| Val::Unit),
            Stmt::Expr(expr) => expr.eval(env),
            Stmt::FuncDef(func_def) => {
                func_def.eval(env)?;
                Ok(Val::Unit)
            },
        }
    }
}
