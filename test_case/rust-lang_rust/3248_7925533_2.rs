
$ rustc --pretty expanded ./simple_expansion_test.rs > ./simple_expansion_test_expanded.rs

$ cat >> ./simple_expansion_test_expanded.rs
#[test]
fn test_unwrap_42() {
  assert (unwrap_42() == 42);
}
