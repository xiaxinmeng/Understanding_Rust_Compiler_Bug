 rust
fn work(_: ~int) {}
fn foo(_: proc()) {}

fn main() {
  let a = ~1;
  foo(proc() { foo(proc() { work(a) }) })
}
