
error[E0596]: cannot borrow data in a `&` reference as mutable
 --> src/lib.rs:6:5
  |
6 | /     distances
7 | |         .as_slice()
8 | |         .sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  | |________________________________________________________________^ cannot borrow as mutable
