use crate::env::Env;
use crate::utils;
use crate::val::Val;
use crate::stmt::Stmt;
use crate::expr::Op;

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    pub(crate) stmts: Vec<Stmt>,
}

impl Block {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let mut s = s;
        let mut stmts = vec![];
        while let Ok((new_s, stmt)) = Stmt::new(s) {
            stmts.push(stmt);

            let (new_s, _) = utils::extract_whitespace(new_s);
            s = new_s;
        }

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;

        Ok((s, Self { stmts }))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        if self.stmts.is_empty() {
            return Ok(Val::Unit);
        }
        let mut env = env.create_child();
        let stmts_except_last_one = &self.stmts[..self.stmts.len() - 1];
        for stmt in stmts_except_last_one {
            stmt.eval(&mut env)?;
        }
        self.stmts
            .last()
            .map_or(Ok(Val::Unit), |stmt| stmt.eval(&mut env))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BindingUsage, Expr, Number};
    use super::*;
    use crate::binding_def::BindingDef;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })));
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{5}"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))]
                }
            ))
        );
    }

    #[test]
    fn parse_block_with_multiple_stmts() {
        assert_eq!(
            Block::new(
                "{
    let a = 10
    let b = a
    b
}"
            ),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_owned(),
                            val: Expr::Number(Number(10))
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_owned(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_owned()
                            })
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "b".to_owned()
                        }))
                    ]
                }
            ))
        );
    }

    #[test]
    fn eval_block() {
        assert_eq!(
            Expr::Block(Block {
                stmts: vec![Stmt::Expr(Expr::Number(Number(10)))],
            })
            .eval(&Env::default()),
            Ok(Val::Number(10))
        )
    }
    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Expr::Block(Block { stmts: vec![] }).eval(&Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn eval_block_with_binding_def_and_usage() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "foo".to_owned(),
                        val: Expr::Number(Number(3)),
                    }),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "foo".to_owned()
                    })),
                ]
            }
            .eval(&Env::default()),
            Ok(Val::Number(3))
        );
    }

    #[test]
    fn eval_block_with_only_binding_defs() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "foo".to_owned(),
                        val: Expr::Number(Number(3))
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "bar".to_owned(),
                        val: Expr::Number(Number(4)),
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "foobar".to_owned(),
                        val: Expr::Number(Number(5)),
                    })
                ],
            }
            .eval(&mut Env::default()),
            Ok(Val::Unit)
        );
    }

    #[test]
    fn eval_block_with_multiple_exprs() {
        assert_eq!(
            Block {
                stmts:vec![
                    Stmt::Expr(Expr::Number(Number(5))),
                    Stmt::Expr(Expr::Number(Number(42))),
                    Stmt::Expr(Expr::Operation {
                        lhs: Number(5),
                        rhs: Number(26),
                        op: Op::Mul,
                    })
                ]
            }.eval(&Env::default()),
            Ok(Val::Number(130))
        );
    }

    #[test]
    fn eval_block_using_bindings_from_parent_env() {
        assert_eq!(Block {
            stmts: vec![
                Stmt::BindingDef(BindingDef {
                    name: "foo".to_owned(),
                    val: Expr::Number(Number(3)),
                }),
                Stmt::Expr(Expr::Block(Block {
                    stmts: vec![
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "foo".to_owned(),
                        }))
                    ]
                })), 
            ],
        }.eval(&Env::default()), Ok(Val::Number(3)))
    }

}
