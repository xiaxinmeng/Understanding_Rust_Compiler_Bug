rust
fn foo() -> Box<Iterator<Item = Input>> {
    Box::new(once(Expr { ident: "my_string".to_string() }))
}
