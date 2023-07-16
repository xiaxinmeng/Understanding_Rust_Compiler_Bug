
error[E0596]: cannot borrow `*ref_term` as mutable, as it is behind a `&` reference
  --> issue-57431-narrowed.rs:13:5
   |
11 |         &X
   |         -- help: consider changing this to be a mutable reference: `&mut X`
12 |     };
13 |     ref_term.mutate();
   |     ^^^^^^^^ `ref_term` is a `&` reference, so the data it refers to cannot be borrowed as mutable
