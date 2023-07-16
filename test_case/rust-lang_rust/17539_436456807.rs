
error[E0391]: cycle detected when processing `SomeFn`
 --> src/main.rs:1:23
  |
1 | type SomeFn = fn() -> SomeFn;
  |                       ^^^^^^
  |
  = note: ...which again requires processing `SomeFn`, completing the cycle
