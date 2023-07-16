
[01:30:33] error[E0004]: non-exhaustive patterns: `Promoted(_)` not covered
[01:30:33]    --> tools/miri/src/validation.rs:138:18
[01:30:33]     |
[01:30:33] 138 |         Ok(match *place {
[01:30:33]     |                  ^^^^^^ pattern `Promoted(_)` not covered
