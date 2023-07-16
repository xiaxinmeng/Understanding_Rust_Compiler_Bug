plain

 finished in 93.675 seconds
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/checkout/library/core)
error[E0271]: type mismatch resolving `<! as std::ops::Not>::Output == bool`
    |
    |
239 |     if !return () {}
    |        ^^^^^^^^^^ expected `!`, found `bool`
    = note: expected type `!`
               found type `bool`

For more information about this error, try `rustc --explain E0271`.
