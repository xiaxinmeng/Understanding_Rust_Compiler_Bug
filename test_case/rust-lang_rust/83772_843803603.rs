plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.43
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
803 |         R: Try<Ok = B>;
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
811 |         R: Try<Ok = B>;
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
838 |         R: Try<Ok = B>,
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
883 |         R: Try<Ok = B>,
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
930 |         R: Try<Ok = B>,
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
975 |         R: Try<Ok = B>,
    |                ^^ associated type `Ok` not found
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0220`.
error: could not compile `core`
