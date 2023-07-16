text
error[E0769]: expected field identifier, found tuple index
  --> src/main.rs:8:9
  |
8 |         E::S { 0, 1 } => {}
  |         ----   ^
  |         |
  |         while parsing the fields for this pattern
  |
   = help: use the tuple variant pattern syntax instead
