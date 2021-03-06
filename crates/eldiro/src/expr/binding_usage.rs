use crate::env::Env;
use crate::utils;
use crate::val::Val;
use crate::expr::FuncCall;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_owned()
                }
            ))
        );
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(3));

        assert_eq! {
            BindingUsage {
                name: "foo".to_owned()
            }.eval(&env),
            Ok(Val::Number(3))
        };
    }

    #[test]
    fn eval_non_existing_binding_usage() {
        let env = Env::default();

        assert_eq! {
            BindingUsage {
                name: "foo".to_owned()
            }.eval(&env),
            Err("binding with name 'foo' does not exist".to_owned())
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;
        Ok((
            s,
            BindingUsage {
                name: name.to_owned(),
            },
        ))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name).or_else(|error_msg| {
            if env.get_func(&self.name).is_ok() {
                FuncCall {
                    callee: self.name.clone(),
                    params: vec![]
                }.eval(env)
            } else {
                Err(error_msg)
            }
        })
    }
}
