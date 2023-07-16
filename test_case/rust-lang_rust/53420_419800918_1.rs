text
error[E0308]: mismatched types
 --> src/main.rs:9:20
  |
9 |     let _:fn(()) = |_:<() as Lt<'_>>::T| {};
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
  |
  = note: expected type `fn(())`
             found type `[closure@src/main.rs:9:20: 9:44]`
