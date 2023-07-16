rust
macro serde() {}

#[derive(Serialize)]
#[serde] // ERROR macro `serde` may not be used in attributes
struct S;

fn main() {}
