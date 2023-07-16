
error[E0271]: type mismatch resolving `<Fi<u32> as F>::T == i8`
 --> src/main.rs:9:27
  |
9 |     let _: &dyn F<T=i8> = &Fi(1u32);
  |                           ^^^^^^^^^ expected i8, found u32
  |
  = note: expected type `i8`
             found type `u32`
  = note: required for the cast to the object type `dyn F<T = i8>`
