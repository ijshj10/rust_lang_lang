use crate::{expr::Expr, utils};
use crate::env::Env;
#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String, 
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, Self) {
        let s = if s.starts_with("let") {
            &s[3..]
        } else {
            panic!("expected let")
        };

        let (s, _) = utils::extract_whitespace(s);
        
        let (s, name) = utils::extract_ident(s);
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("=", s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s);
        
        let name = name.to_owned();
        let binding_def = Self { name, val };

        (s, binding_def)
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        env.store_binding(self.name.clone(), self.val.eval());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            (
                "", 
                BindingDef {
                    name: String::from("a"),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div
                    }
                })
        );
    }
}