rust
const fn foo() {}
const fn bar() {}
fn baz() {}

fn main() {
 let x = match 2 {
  2 => foo,
  3 => bar,
  _ => baz
 };
}
