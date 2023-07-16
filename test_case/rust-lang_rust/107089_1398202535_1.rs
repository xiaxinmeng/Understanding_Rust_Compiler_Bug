console
error: internal compiler error[E0391]: cycle detected when computing function signature of `a`
 --> ./p/c.rs:1:1
  |
1 | fn a() -> _ { &a }
  | ^^^^^^^^^^^
  |
note: ...which requires type-checking `a`...
 --> ./p/c.rs:1:1
  |
1 | fn a() -> _ { &a }
  | ^^^^^^^^^^^
  = note: ...which again requires computing function signature of `a`, completing the cycle
note: cycle used when collecting item types in top-level module
 --> ./p/c.rs:1:1
  |
1 | / fn a() -> _ { &a }
2 | | fn main() { }
  | |_____________^
  = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
