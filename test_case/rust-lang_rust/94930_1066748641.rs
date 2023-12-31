
warning: function cannot return without recursing
  --> src/main.rs:21:5
21 |     fn default() -> Self {
   |     ^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
...
25 |             ..Default::default()
   |               ------------------ recursive call site
   |
   = note: `#[warn(unconditional_recursion)]` on by default
   = help: a `loop` may express intention better if this is on purpose
