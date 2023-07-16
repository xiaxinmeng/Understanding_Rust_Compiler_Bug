rust
#[derive(Serialize)]
#[serde(tag = "type")]
enum Expression {
    Foo,
    Baz(Box<Expression>),
}
