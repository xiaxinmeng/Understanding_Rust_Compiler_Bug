
error[E0596]: cannot borrow `term` as mutable, as it is not declared as mutable
 --> src/main.rs:9:23
  |
9 |         Some(term) => &mut term,
  |                       ^^^^^^^^^
  |                       |
  |                       cannot borrow as mutable
  |                       try removing `&mut` here

error[E0596]: cannot borrow `*term` as mutable, as it is behind a `&` reference
  --> src/main.rs:12:5
   |
9  |         Some(term) => &mut term,
   |                       --------- help: consider changing this to be a mutable reference: `&mut mut term`
...
12 |     term.mutate();
   |     ^^^^ `term` is a `&` reference, so the data it refers to cannot be borrowed as mutable
