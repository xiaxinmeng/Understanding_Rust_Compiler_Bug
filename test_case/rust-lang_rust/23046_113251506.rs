
test.rs:1:1: 4:2 error: this type cannot be instantiated without an instance of itself [E0073]
test.rs:1 pub enum Expr<'var, VAR> {
test.rs:2     Let(Box<Expr<'var, VAR>>,
test.rs:3         Box<for<'v: 'var> Fn(Expr<'v, VAR>) -> Expr<'v, VAR> + 'var>)
test.rs:4 }
test.rs:1:1: 4:2 help: run `rustc --explain E0073` to see a detailed explanation
test.rs:1:1: 4:2 help: consider using `Option<Expr<'var, VAR>>`
error: aborting due to previous error

