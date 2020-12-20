mod binding_def;
mod expr;
mod stmt;
mod val;
mod env;
mod utils;

pub use env::Env;
pub use val::Val;

#[derive(Debug)]
pub struct Parse(stmt::Stmt);

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::new(s)?;
    match s.len() {
        0 => Ok(Parse(stmt)),
        _ => Err("input was not consumed fully by parser.".to_owned()),
    }
}

impl Parse {
    

    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}