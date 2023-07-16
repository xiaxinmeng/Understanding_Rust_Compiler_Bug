
$ cat ./simple_expansion_test.rs
use std;

macro_rules! unwrap_something (
  ($some_expr:expr, $default:expr) => (
    match $some_expr {
      some(x) => x,
      none => $default
    }
  )
)

fn unwrap_42() -> int { unwrap_something!(some(42), -1) }

#[test]
fn test_unwrap_42() {
  assert (unwrap_42() == 42);
}
