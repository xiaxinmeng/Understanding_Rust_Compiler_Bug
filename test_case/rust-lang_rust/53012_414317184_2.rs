rust
#[derive(Serialize)]
pub struct Foo {
    #[serde(...)]
    pub foo_field: u32,
}

#[derive_serialize_as_an_attribute]
pub struct Foo {
    #[serde(...)]
    pub foo_field: u32,
}
