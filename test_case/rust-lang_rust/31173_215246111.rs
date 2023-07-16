
error: mismatched types [--explain E0308]
 --> tmp/quickie.rs:3:5
3 |>     (0..10).filter(|i| expected)
  |>     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found struct `std::iter::Filter`
note: expected type `()`
note:    found type `std::iter::Filter<std::ops::Range<_>, [closure@tmp/quickie.rs:3:20: 3:32 expected:_]>`

error: aborting due to previous error
