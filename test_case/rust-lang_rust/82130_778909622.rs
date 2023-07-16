plain
   Compiling libc v0.2.85
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
407 |     pub const fn unwrap_or(self, default: T) -> T {
    |                                  ^^^^^^^ constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
567 |     pub const fn ok_or<E>(self, err: E) -> Result<T, E> {
    |                                 ^^^ constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
700 |     pub const fn and<U>(self, optb: Option<U>) -> Option<U> {
    |                         ^^^^ constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
700 |     pub const fn and<U>(self, optb: Option<U>) -> Option<U> {
    |                               ^^^^ constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
797 |     pub const fn or(self, optb: Option<T>) -> Option<T> {
    |                           ^^^^ constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
797 |     pub const fn or(self, optb: Option<T>) -> Option<T> {
    |                     ^^^^ constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
851 |         match (self, optb) {
    |               ^^^^^^^^^^^^ constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
    |
983 |         match (self, other) {
    |               ^^^^^^^^^^^^^ constant functions cannot evaluate destructors

