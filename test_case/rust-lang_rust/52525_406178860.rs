rust
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
enum Foo {
    A(u8)
}
