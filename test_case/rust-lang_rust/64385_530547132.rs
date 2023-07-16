rust
//use serde::Deserialize;

#[derive(Deserialize)]
struct Outer<'a>(#[serde(borrow)] Inner<'a>);
