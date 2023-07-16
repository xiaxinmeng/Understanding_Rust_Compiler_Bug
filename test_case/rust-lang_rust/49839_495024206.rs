
error[E0594]: cannot assign to `t.v` which is behind a `&` reference
  --> src/main.rs:13:9
   |
12 |     for mut t in values {
   |         ----- help: consider changing this to be a mutable reference: `&mut Test`
13 |         t.v += 1;
   |         ^^^^^^^^ `t` is a `&` reference, so the data it refers to cannot be written
