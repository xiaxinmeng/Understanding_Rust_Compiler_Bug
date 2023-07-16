rust
#[derive(Deserialize)]
struct Foo {
    #[cfg(something)]
    a: i32,
    #[serde(malformed attribute)]
    b: u32,
}
