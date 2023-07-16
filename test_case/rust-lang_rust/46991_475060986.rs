rust
use serde::Deserialize;

#[derive(Deserialize)]
struct S;

fn main() {
    serde_json::to_string(&S);
}
