
error[E0308]: mismatched types
 --> src/main.rs:3:9
  |
3 |     for &x in v {
  |         ^^    - this is an iterator with items of type `&mut u32`
  |         |
  |         types differ in mutability
  |
  = note: expected mutable reference `&mut u32`
                     found reference `&_`
help: consider removing `&` from the pattern
  |
3 -     for &x in v {
3 +     for x in v {
  |
