
mod foo {
    pub(crate) fn foo() -> ::S {
      ::S
    }
}

struct S;

fn main() {
  foo::foo();
}
