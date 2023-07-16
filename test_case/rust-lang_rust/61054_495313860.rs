
error[E0308]: mismatched types
 --> file2.rs:4:12
  |
4 | (&mut x) = None;
  |            ^^^^ expected mutable reference, found enum `std::option::Option`
  |
  = note: expected type `&mut {integer}`
             found type `std::option::Option<_>`

error[E0070]: invalid left-hand side expression
 --> file2.rs:4:1
  |
4 | (&mut x) = None;
  | ^^^^^^^^^^^^^^^ left-hand of expression not valid
