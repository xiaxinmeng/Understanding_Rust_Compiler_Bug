 rust
#[macro_use]
extern crate lust;

use lust::{Parser, Scope};

#[test]
fn test_macro_expansion() {
    let ref mut scope = Scope::new_std();
    let input = "(def m (macro (a) '(+ 1 ~a)))";
    let expr = Parser::new(input.chars())
        .parse().ok().unwrap()
        .expand(scope).ok().unwrap()
        .eval(scope).ok().unwrap();
....
