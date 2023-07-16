rust
error[E0391]: cycle detected when evaluating constant `main::FOO`
 --> src/main.rs:1:28
  |
1 | pub const FOO: usize = foo();
  |           ^^^
  |
note: ...which then calls function `main::foo()`
 --> src/main.rs:1:28
  |
1 | pub const FOO: usize = foo();
  |                        ^^^^^
  |
note: ...which then uses `main::FOO` within its body
 --> src/main.rs:2:28
  |
2 | pub const fn foo() -> usize { FOO }
  |                               ^^^
  |
  = note: ...evaluating constant `main::FOO` then depends on function `main::foo()`
  = note: ...which again requires evaluating constant `main::FOO`, completing the cycle
