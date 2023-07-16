
warning: function cannot return without recursing
 --> src/lib.rs:6:5
  |
6 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
7 |         fmt::Display::fmt(self, f)
  |         -------------------------- recursive call site
  |
  = note: `#[warn(unconditional_recursion)]` on by default
  = help: a `loop` may express intention better if this is on purpose

warning: 1 warning emitted
