
! Compiling of exercises/conversions/from_into.rs failed! Please try again. Here's the output:
error: `let` expressions are not supported here
  --> exercises/conversions/from_into.rs:44:33
   |
44 |         if (parts.len() == 2 && let Some(n) = name && let Some(Ok(a))= age) {
   |                                 ^^^^^^^^^^^^^^^^^^
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions
