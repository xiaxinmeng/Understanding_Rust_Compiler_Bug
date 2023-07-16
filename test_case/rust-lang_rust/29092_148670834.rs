 rust
use self::Term::*;

#[derive(Clone)]
pub enum Term {
    True,
    False,
    If(Box<Term>),
}

// a small-step evaluator
pub fn small_eval(v: Term) -> Term {
    match v {
        If(ref con) => match **con {
            True => True,
            False => *con.clone(),
            _ => True,
        },
        _ => True,
    }
}

fn main() {
    small_eval(If(Box::new(True)));
}
