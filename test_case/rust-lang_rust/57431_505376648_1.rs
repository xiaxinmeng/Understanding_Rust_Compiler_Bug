
error[E0596]: cannot borrow `*ref_term` as mutable, as it is behind a `&` reference
 --> src/main.rs:9:5
  |
8 |     let ref_term: &X = &mut term;
  |                        --------- help: consider changing this to be a mutable reference: `&mut mut term`
9 |     ref_term.mutate();
  |     ^^^^^^^^ `ref_term` is a `&` reference, so the data it refers to cannot be borrowed as mutable
