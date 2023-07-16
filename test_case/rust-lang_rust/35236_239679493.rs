 rust
#![crate_name = "foo"]

// @has 'foo/FooStruct.t.html'
// @has - '//p/a' 'struct.FooStruct.html'
// @has 'foo/struct.FooStruct.html'
pub struct FooStruct {
    // @has - '//*[@id="foo.v"]' 'foo'
    pub foo: (),
    // @has - '//*[@id="dupe.v"]' 'dupe'
    pub dupe: (),
}
impl FooStruct {
    // @has - '//*[@id="bar.v"]' 'bar'
    pub fn bar() {}
    // @has - '//*[@id="dupe.v-1"]' 'dupe'
    pub fn dupe() {}
}
