
warning: function cannot return without recursing
 --> b.rs:6:3
  |
6 |   fn b(&mut self, c: &str) -> Option<usize> {
  |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
7 |     self.b(c.trim())
  |     ---------------- recursive call site
  |
  = note: `#[warn(unconditional_recursion)]` on by default
  = help: a `loop` may express intention better if this is on purpose

warning: 1 warning emitted
