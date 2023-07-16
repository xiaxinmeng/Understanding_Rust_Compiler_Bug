
pub fn foo() {}

#[cfg(test)]
#[test]
fn bar() { panic!("the disco"); }

$ rustdoc +local --test e.rs

running 1 test
test e.rs - foo (line 3) ... FAILED

failures:

---- e.rs - foo (line 3) stdout ----
error[E0425]: cannot find value `no` in this scope
 --> e.rs:4:1
  |
3 | no
  | ^^ not found in this scope

thread 'e.rs - foo (line 3)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    e.rs - foo (line 3)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

