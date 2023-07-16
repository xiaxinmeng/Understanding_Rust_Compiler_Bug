
error[E0382]: capture of moved value: `t` (Ast)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrowck/borrowck-move-moved-value-into-closure.rs:24:22
   |
23 |     call_f(move|| { *t + 1 });
   |            ------ value moved (into closure) here
24 |     call_f(move|| { *t + 1 }); //[ast]~ ERROR capture of moved value
   |                      ^ value captured here after move
   |
   = note: move occurs because `t` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `t` (Mir)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrowck/borrowck-move-moved-value-into-closure.rs:24:12
   |
23 |     call_f(move|| { *t + 1 });
   |            ----------------- value moved here
24 |     call_f(move|| { *t + 1 }); //[ast]~ ERROR capture of moved value
   |            ^^^^^^^^^^^^^^^^^ value used here after move

error: aborting due to 2 previous errors
