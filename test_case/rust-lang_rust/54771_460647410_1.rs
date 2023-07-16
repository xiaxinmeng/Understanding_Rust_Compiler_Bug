
error[E0308]: mismatched types
 --> file.rs:3:12
  |
3 |     bar(|| { 5; })
  |            ^^^-^^
  |            |  |
  |            |  help: consider removing this semicolon
  |            expected u8, found ()
  |
  = note: expected type `u8`
             found type `()`
