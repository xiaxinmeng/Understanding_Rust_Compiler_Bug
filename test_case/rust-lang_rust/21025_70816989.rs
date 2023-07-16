 rust
struct Foo<T, U> ( u8 );

fn bar(x: Foo<u8, i8>) {

}

fn main() {
  let x: Foo<u8, u8> = Foo(42);
  bar(x);
}
