rust
error[E0747]: type provided when a constant was expected
 --> ...\test.rs:5:15
  |
5 |     init::<N, _>(&mut foos);
  |               ^
  |
  = help: const arguments cannot yet be inferred with `_`
  = note: type arguments must be provided before constant arguments
  = help: reorder the arguments: consts: `<N, M>`
  