
use crate::{expr::Expr, stmt::Stmt};
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
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

        Ok((s, Self {stmts}))
    }
}

#[cfg(test)]
mod tests {
    use crate::{binding_def::BindingDef};
    use super::*;
    use super::super::{Expr, Number, BindingUsage};

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new()})));
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(Block::new("{5}"), Ok(("", Block { stmts: vec![Stmt::Expr(Expr::Number(Number(5)))]})));
    }

    #[test]
    fn parse_block_with_multiple_stmts() {
        assert_eq!(
            Block::new("{
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
                }))
        );
    }
}