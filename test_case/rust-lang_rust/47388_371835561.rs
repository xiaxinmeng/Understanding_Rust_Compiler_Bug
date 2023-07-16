
error[E0594]: cannot assign through `&`-reference fancy_ref
  --> src/test/compile-fail/E0389.rs:18:5
   |
17 |     let fancy_ref = &(&mut fancy);
   |                     ------------- help: consider changing this to be a mutable reference: `&mut`
18 |     fancy_ref.num = 6; //~ ERROR E0389
   |     ^^^^^^^^^^^^^^^^^ cannot assign to field of `&`-reference

