
warning: function cannot return without recursing
  --> src/lib.rs:15:5
   |
15 |     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
16 |         &mut self[index]
   |              ----------- recursive call site
   |
   = help: a `loop` may express intention better if this is on purpose
   = note: `#[warn(unconditional_recursion)]` on by default
