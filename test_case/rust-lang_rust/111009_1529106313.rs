plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the trait bound `char: ~const From<u8>` is not satisfied
   --> library/core/src/ascii/ascii_char.rs:509:9
509 |         char::from(self.as_u8())
509 |         char::from(self.as_u8())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `~const From<u8>` is not implemented for `char`
    |
note: the trait `From<u8>` is implemented for `char`, but that implementation is not `const`
   --> library/core/src/ascii/ascii_char.rs:509:9
509 |         char::from(self.as_u8())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^


error[E0015]: cannot call non-const fn `<char as From<u8>>::from` in constant functions
   --> library/core/src/ascii/ascii_char.rs:509:9
509 |         char::from(self.as_u8())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
