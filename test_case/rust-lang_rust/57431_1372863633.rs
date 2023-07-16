
error[E0596]: cannot borrow `term` as mutable, as it is not declared as mutable
 --> src/main.rs:9:23
  |
9 |         Some(term) => &mut term,
  |                       ^^^^^^^^^
  |                       |
  |                       cannot borrow as mutable
  |                       help: try removing `&mut` here

error[E0596]: cannot borrow `*term` as mutable, as it is behind a `&` reference
  --> src/main.rs:12:5
   |
8  |     let term = match &term {
   |         ---- consider changing this binding's type to be: `&mut X`
...
12 |     term.mutate();
   |     ^^^^^^^^^^^^^ `term` is a `&` reference, so the data it refers to cannot be borrowed as mutable
