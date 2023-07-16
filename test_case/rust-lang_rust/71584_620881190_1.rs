
-*- mode: compilation; default-directory: "~/src/rustc-misleading-error-message/src/" -*-
Compilation started at Wed Apr 29 00:02:23

cargo build --bin bug
   Compiling misleading v0.1.0 (/home/jonas/src/rustc-misleading-error-message)
error[E0284]: type annotations needed for `usize`
 --> src/main.rs:3:18
  |
3 |         let f = |number: usize| number;
  |                  ^^^^^^ consider giving this closure parameter a type
  |
  = note: cannot resolve `<usize as std::ops::Add<_>>::Output == usize`

error[E0284]: type annotations needed for `usize`
  --> src/main.rs:10:14
   |
10 |     let f = |number: usize| number;
   |              ^^^^^^ consider giving this closure parameter a type
   |
   = note: cannot resolve `<usize as std::ops::Add<_>>::Output == usize`

error[E0284]: type annotations needed for `usize`
  --> src/main.rs:15:27
   |
15 |     let x: usize = 0usize + 0usize.into();
   |         -                 ^ cannot infer type for type `usize`
   |         |
   |         consider giving `x` a type
   |
   = note: cannot resolve `<usize as std::ops::Add<_>>::Output == usize`

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0284`.
error: could not compile `misleading`.

To learn more, run the command again with --verbose.

Compilation exited abnormally with code 101 at Wed Apr 29 00:02:23
