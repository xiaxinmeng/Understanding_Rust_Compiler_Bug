rust
const ONE : i32 = 1;
const TWO : i32 = 2;

struct Foo<const i: i32, const j: i32> {}

impl<const i: i32> Foo<i, ONE> {
    pub fn foo() {}
}

impl<const i: i32> Foo<i, TWO> {
    pub fn foo() {}
}
