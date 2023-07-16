text
error[E0493]: destructors cannot be evaluated at compile-time
   --> library/core/src/option.rs:733:25
    |
733 |     pub const fn unwrap(self) -> T {
    |                         ^^^^ constant functions cannot evaluate destructors
...
738 |     }
    |     - value is dropped here
