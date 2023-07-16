
zmd@ReflectiveCoherence:~/Code/rust$ rustc +stage1 slice.rs 
error: unexpected `,` in pattern
 --> slice.rs:4:10
  |
4 |     let a, b = &[1, 2];
  |         -^-- help: try adding parentheses: `(a, b)`

error[E0658]: non-reference pattern used to match a reference (see issue #42640)
 --> slice.rs:4:9
  |
4 |     let a, b = &[1, 2];
  |         ^^^^ help: consider using a reference: `&a, b`
  |
  = help: add #![feature(match_default_bindings)] to the crate attributes to enable

error[E0308]: mismatched types
 --> slice.rs:4:9
  |
4 |     let a, b = &[1, 2];
  |         ^^^^ expected array of 2 elements, found tuple
  |
  = note: expected type `[{integer}; 2]`
             found type `(_, _)`

error: aborting due to 3 previous errors
