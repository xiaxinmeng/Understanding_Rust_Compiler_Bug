 rust
#[feature(struct_variants)];
#[allow(dead_code)];
#[deny(raw_pointer_deriving)];

#[deriving(Clone)]
struct Foo {
     x: *int //~ ERROR #[deriving] with raw pointer
}

#[deriving(Clone)]
struct Bar(*mut int); //~ ERROR #[deriving] with raw pointer

enum Baz {
     A(*int),  //~ ERROR #[deriving] with raw pointer
     B { x: *mut int }  //~ ERROR #[deriving] with raw pointer
}
fn main() {}
