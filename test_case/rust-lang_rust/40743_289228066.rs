
error[E0308]: mismatched types
  --> src/main.rs:17:9
   |
17 |     foo(bar);
   |         ^^^ expected concrete lifetime, found bound lifetime parameter 
   |
   = note: expected type `std::boxed::Box<for<'r> std::ops::Fn(&'r i32) + 'static>`
              found type `std::boxed::Box<std::ops::Fn(_)>`
