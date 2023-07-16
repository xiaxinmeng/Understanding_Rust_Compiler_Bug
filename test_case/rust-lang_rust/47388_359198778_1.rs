
error[E0389]: cannot assign to `fancy_ref.num`
  --> src/test/compile-fail/E0389.rs:18:5
   |
18 |     let fancy_ref = &(&mut fancy);
   |                     ^not declared `mut`

