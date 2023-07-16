
error[E0391]: unsupported cyclic reference between types/traits detected
 --> src/main.rs:1:23
  |
1 | type SomeFn = fn() -> SomeFn;
  |                       ^^^^^^ cyclic reference
  |
note: the cycle begins when processing `SomeFn`...
 --> src/main.rs:1:1
  |
1 | type SomeFn = fn() -> SomeFn;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...which then again requires processing `SomeFn`, completing the cycle.
