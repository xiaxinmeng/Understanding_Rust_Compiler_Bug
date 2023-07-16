
error[E0308]: mismatched types
  --> src/main.rs:10:15
   |
10 |     let foo = FooConst::<FOO_ARR> {};
   |               ^^^^^^^^^^^^^^^^^^^^^^ expected `FOO_ARR`, found `FOO_ARR`
   |
   = note: expected type `_`
            found struct `FooConst<FOO_ARR>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `abc`
