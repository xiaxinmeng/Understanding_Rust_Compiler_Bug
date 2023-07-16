
error: unresolved name `unfound`. Did you mean `found`? [--explain E0425]
 --> tmp/quickie2.rs:3:24
3 |>     (0..10).filter(|i| unfound)
  |>                        ^^^^^^^

error: mismatched types [--explain E0308]
 --> tmp/quickie2.rs:3:5
3 |>     (0..10).filter(|i| unfound)
  |>     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found struct `std::iter::Filter`
note: expected type `()`
note:    found type `std::iter::Filter<std::ops::Range<_>, [closure@tmp/quickie2.rs:3:20: 3:31]>`
