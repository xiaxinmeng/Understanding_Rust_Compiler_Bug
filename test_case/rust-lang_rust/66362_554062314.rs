rust
warning: variable `iHateSnakes` should have a snake case name
  --> src/main.rs:10:26
   |
10 |         Animal::Spider { iHateSnakes } => dbg!(iHateSnakes),
   |                          ^^^^^^^^^^^ help: convert the identifier to snake case: `i_hate_snakes`
   |
   = note: `#[warn(non_snake_case)]` on by default
