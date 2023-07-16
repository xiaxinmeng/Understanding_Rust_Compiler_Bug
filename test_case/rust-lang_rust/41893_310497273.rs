
error[E0080]: constant evaluation error
  --> /home/mark/BuildDisk/rust/src/libcore/nonzero.rs:47:9
   |
47 |         NonZero(inner)
   |         ^^^^^^^ unimplemented constant expression: tuple struct constructors
   |
note: for pattern here
  --> test.rs:21:9
   |
21 |         a::FOO => true,
   |         ^^^^^^

error: aborting due to previous error(s)
