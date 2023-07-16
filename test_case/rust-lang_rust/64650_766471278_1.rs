
error[E0308]: mismatched types
  --> src/main.rs:19:9
   |
19 |     foo.box_and_set_data(a);
   |         ^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected mutable reference `&mut BoxHolder<Box<for<'r> fn(&'r str) -> &str>>`
              found mutable reference `&mut BoxHolder<Box<for<'r> fn(&'r str) -> &'r str>>`
