
pub use foo::foo;

mod foo {
    pub fn foo() -> ::S {
      ::S
    }
}

struct S;

fn main() {
  foo::foo();
}
