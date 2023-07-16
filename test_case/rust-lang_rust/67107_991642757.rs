rust
#![forbif(unused)]
// ^^^^^^ note the typo

use serde::Serialize;

#[derive(Serialize)]
pub struct Foo {
    #[serde(rename = "bar")]
    foo: i32,
    #[serde(rename = "quux")]
    baz: i32,
}
