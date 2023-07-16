plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: variable does not need to be mutable
   --> library/core/src/char/methods.rs:333:27
    |
333 |     pub fn to_digit(self, mut radix: u32) -> Option<u32> {
    |                           |
    |                           help: remove this `mut`
    |
    |
    = note: `-D unused-mut` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `core`

