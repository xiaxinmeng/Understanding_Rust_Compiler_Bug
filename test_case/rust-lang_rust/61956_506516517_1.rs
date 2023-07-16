
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
--> src/lib.rs:2:45
  |
2 | fn identity<T: Sized>(x: T) -> T { unsafe { core::mem::transmute(x) } }
  |                                             ^^^^^^^^^^^^^^^^^^^^
  | = note: `T` does not have a fixed size
