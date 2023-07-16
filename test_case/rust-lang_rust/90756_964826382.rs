plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0493]: destructors cannot be evaluated at compile-time
    --> library/core/src/option.rs:1431:24
     |
1431 |     pub const fn unzip(self) -> (Option<T>, Option<U>) {
     |                        ^^^^ constant functions cannot evaluate destructors
error[E0493]: destructors cannot be evaluated at compile-time
    --> library/core/src/option.rs:1623:28
     |
     |
1623 |     pub const fn transpose(self) -> Result<Option<T>, E> {
     |                            ^^^^ constant functions cannot evaluate destructors
error[E0493]: destructors cannot be evaluated at compile-time
    --> library/core/src/option.rs:2094:26
     |
     |
2094 |     pub const fn flatten(self) -> Option<T> {
     |                          ^^^^ constant functions cannot evaluate destructors
error[E0493]: destructors cannot be evaluated at compile-time
    --> library/core/src/result.rs:1531:28
     |
     |
1531 |     pub const fn transpose(self) -> Option<Result<T, E>> {
     |                            ^^^^ constant functions cannot evaluate destructors
error[E0493]: destructors cannot be evaluated at compile-time
    --> library/core/src/result.rs:1600:33
     |
1600 |     pub const fn into_ok_or_err(self) -> T {
