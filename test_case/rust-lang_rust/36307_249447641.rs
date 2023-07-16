 rust
failures:

---- [compile-fail] compile-fail/E0408.rs stdout ----

error: /home/faebser/workspace/rust/src/test/compile-fail/E0408.rs:15: unexpected "error": '15:19: 15:23: variable `y` from pattern #1 is not bound in pattern #2 [E0408]'

error: /home/faebser/workspace/rust/src/test/compile-fail/E0408.rs:15: unexpected "note": 'pattern doesn't bind `y`'

error: /home/faebser/workspace/rust/src/test/compile-fail/E0408.rs:14: expected error not found: variable `y` from pattern #2 is not bound in pattern #1

error: /home/faebser/workspace/rust/src/test/compile-fail/E0408.rs:14: expected note not found: pattern doesn't bind `y`

error: 2 unexpected errors found, 2 expected errors not found
status: exit code: 101
