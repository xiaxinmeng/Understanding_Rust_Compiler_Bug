 rust
#[allow(non_snake_case)]
mod Foo { pub fn foo() -> i32 { 4 } }

struct Foo(i32);
impl Foo { fn foo() -> i32 { 3 } }

fn main() { println!("foo: {}", Foo::foo()); } // what does this print?
