use crate::utils;
use crate::val::Val;
use crate::{
    expr::{Expr, Number},
    Env,
};

#[cfg(test)]
mod tests {
    use crate::expr::BindingUsage;
    use crate::stmt::Stmt;

    use super::*;

    #[test]
    fn eval_func_call() {
        let mut env = Env::default();

        env.store_func(
            "id".to_owned(),
            vec!["x".to_owned()],
            Stmt::Expr(Expr::BindingUsage(BindingUsage {
                name: "x".to_owned(),
            })),
        );

        assert_eq!(
            FuncCall {
                callee: "id".to_owned(),
                params: vec![Expr::Number(Number(10))]
            }
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_non_existent_func_call() {
        let env = Env::default();

        assert_eq!(
            FuncCall {
                callee: "i_dont_exist".to_owned(),
                params: vec![]
            }
            .eval(&env),
            Err("function with name 'i_dont_exist' does not exist.".to_owned())
        );
    }

    #[test]
    fn eval_func_call_with_too_few_parameters() {
        let mut env = Env::default();

        env.store_func(
            "mul".to_owned(),
            vec!["x".to_owned(), "y".to_owned()],
            Stmt::Expr(Expr::Operation {
                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "x".to_owned(),
                })),
                rhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "y".to_owned(),
                })),
                op: crate::expr::Op::Mul,
            }),
        );

        assert_eq!(
            FuncCall {
                callee: "mul".to_owned(),
                params: vec![Expr::Number(Number(100))]
            }
            .eval(&env),
            Err("expected 2 parameters, got 1".to_owned())
        );
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_ident(s)?;
        let (s, _) = utils::take_while(|c| c == ' ', s);

        let (s, params) = utils::sequence1(Expr::new, |s| utils::take_while(|c| c == ' ', s), s)?;

        Ok((
            s,
            Self {
                callee: callee.to_owned(),
                params,
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        let mut child_env = env.create_child();

        let (param_names, body) = env.get_func(&self.callee)?;

        let num_expected_params = param_names.len();
        let num_acutal_params = self.params.len();

        if num_expected_params != num_acutal_params {
            return Err(format!(
                "expected {} parameters, got {}",
                num_expected_params, num_acutal_params
            ));
        }

        for (param_name, param_expr) in param_names.into_iter().zip(&self.params) {
            let param_val = param_expr.eval(&child_env)?;
            child_env.store_binding(param_name, param_val);
        }

        body.eval(&mut child_env)
    }
}
