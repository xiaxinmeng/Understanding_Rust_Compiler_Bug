console
error[E0308]: mismatched types
 --> src/main.rs:6:24
  |
6 |     let _x: Clonable = 0i32;
  |                        ^^^^ expected anonymized type, found i32
  |
  = note: expected type `Clonable`
             found type `i32`

error: could not find defining uses
 --> src/main.rs:3:1
  |
3 | existential type Clonable: Clone;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
