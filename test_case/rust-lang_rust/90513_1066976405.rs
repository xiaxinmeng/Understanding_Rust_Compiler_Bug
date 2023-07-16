
warning: function cannot return without recursing
  --> a.rs:8:5
   |
8  |     fn default() -> Self {
   |     ^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
...
11 |             ..Default::default()
   |               ------------------ recursive call site
   |
   = note: `#[warn(unconditional_recursion)]` on by default
   = help: a `loop` may express intention better if this is on purpose
