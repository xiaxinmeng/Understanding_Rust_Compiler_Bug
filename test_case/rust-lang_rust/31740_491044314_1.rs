
error[E0382]: use of moved value: `b2`
  --> src/main.rs:15:14
   |
14 |     if b1 == b2 {}
   |              -- value moved here
15 |     if b1 == b2 {}
   |              ^^ value used here after move
   |
   = note: move occurs because `b2` has type `std::boxed::Box<(dyn Mine + 'static)>`, which does not implement the `Copy` trait
