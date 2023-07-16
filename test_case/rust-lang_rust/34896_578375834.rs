
error[E0596]: cannot borrow `sm.x` as mutable, as it is behind a `&` reference
  --> src/main.rs:10:9
   |
9  |     let mut sm = sr.clone();
   |         ------ help: consider changing this to be a mutable reference: `&mut Str`
10 |     foo(&mut sm.x);
   |         ^^^^^^^^^ `sm` is a `&` reference, so the data it refers to cannot be borrowed as mutable
