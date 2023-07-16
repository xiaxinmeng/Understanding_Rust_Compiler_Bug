
error: non-exhaustive patterns: `Outer(Inner2)` and `Inner3` not covered [--explain E0004]
  --> <anon>:11:5
   |>
11 |>     match A::Outer(B::Inner1) {A::Outer(B::Inner1) => {}}
   |>     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
