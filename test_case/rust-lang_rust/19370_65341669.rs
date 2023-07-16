 rust
struct Foo;

fn foo() {
  *&mut Foo = Foo;
}
