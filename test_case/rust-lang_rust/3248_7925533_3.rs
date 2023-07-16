
$ cat ./simple_expansion_test_expanded.rs
use std;


fn unwrap_42() -> int { match some(42) { some(x) => x, none => -1 } }

#[test]
fn test_unwrap_42() {
  assert (unwrap_42() == 42);
}
