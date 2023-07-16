
error[E0271]: type mismatch resolving `<i32 as Foo>::F == u32`
  --> src/main.rs:14:36
   |
14 |     let z: Box<dyn Bar<F = i32>> = Box::new(y);
   |                                    ^^^^^^^^^^^ expected `u32`, found `i32`
   |
   = note: required for the cast to the object type `dyn Bar<F = i32, F = u32>`
