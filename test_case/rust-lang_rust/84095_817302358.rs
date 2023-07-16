plain
   Compiling toml v0.5.7
error[E0308]: mismatched types
   --> src/bootstrap/compile.rs:477:29
    |
477 |                 builder.run(cmd);
    |                             |
    |                             |
    |                             expected `&mut Command`, found struct `Command`
    |                             help: consider mutably borrowing here: `&mut cmd`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `bootstrap`
