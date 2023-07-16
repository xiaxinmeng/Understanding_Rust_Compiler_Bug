rust
union Foo { a: u32, b: u32 }

fn main() {
  let foo: Foo;
  foo.a = 22;
  foo.b = 44; // error
}
