console
$ cat a.rs 
#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    A = 1,
    B = 2,
}
pub fn encode(v: Type) -> Type {
    match v {
        Type::A => Type::B,
        _ => v,
    }
}
fn main() {
  assert_eq!(Type::B, encode(Type::A));
}
$ rustc -Zmir-opt-level=0 a.rs && ./a
$ rustc a.rs && ./a
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `B`,
 right: `A`', a.rs:13:3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
