
error: mismatched types [--explain E0308]
 --> tmp/quickie2.rs:3:5
3 |>     (0..10).filter(|i| found)
  |>     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found struct `std::iter::Filter`
note: expected type `()`
note:    found type `std::iter::Filter<std::ops::Range<_>, [closure@tmp/quickie2.rs:3:20: 3:29 found:_]>`

error: aborting due to previous error
