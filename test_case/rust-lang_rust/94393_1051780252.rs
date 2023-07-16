plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0493]: destructors cannot be evaluated at compile-time
    --> library/core/src/option.rs:1366:31
     |
1366 |     pub const fn not<U>(self, some: U) -> Option<U>
     |                               ^^^^ constant functions cannot evaluate destructors
1372 |     }
     |     - value is dropped here

error[E0493]: destructors cannot be evaluated at compile-time
error[E0493]: destructors cannot be evaluated at compile-time
    --> library/core/src/option.rs:1366:25
     |
1366 |     pub const fn not<U>(self, some: U) -> Option<U>
     |                         ^^^^ constant functions cannot evaluate destructors
1372 |     }
     |     - value is dropped here

For more information about this error, try `rustc --explain E0493`.
