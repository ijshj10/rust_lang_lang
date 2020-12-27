use crate::stmt::Stmt;
use crate::utils;
use crate::env::Env;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{BindingUsage, Block, Expr, Op};

    #[test]
    fn parse_func_def_with_no_params_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn nothing => {}"),
            Ok((
                "",
                FuncDef {
                    name: "nothing".to_owned(),
                    params: vec![],
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: vec![] })))
                }
            ))
        );
    }

    #[test]
    fn parse_func_def_with_one_param_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn greet name => {}"),
            Ok((
                "",
                FuncDef {
                    name: "greet".to_owned(),
                    params: vec!["name".to_owned(),],
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: vec![] })))
                }
            ))
        )
    }

    #[test]
    fn parse_func_def_with_multiple_params() {
        assert_eq!(
            FuncDef::new("fn add x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_owned(),
                    params: vec!["x".to_owned(), "y".to_owned()],
                    body: Box::new(Stmt::Expr(Expr::Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "x".to_owned()
                        })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "y".to_owned()
                        })),
                        op: Op::Add
                    }))
                }
            ))
        )
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Stmt>,
}

impl FuncDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fn", s)?;
        let (s, _) = utils::extract_whitespace1(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, params) = utils::sequence(
            |s| utils::extract_ident(s).map(|(s, ident)| (s, ident.to_owned())),
            s,
        )?;

        let s = utils::tag("=>", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, body) = Stmt::new(s)?;

        Ok((
            s,
            FuncDef {
                name: name.to_owned(),
                params,
                body: Box::new(body),
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_func(self.name.clone(), self.params.clone(), *self.body.clone());
        Ok(())
    }
}
