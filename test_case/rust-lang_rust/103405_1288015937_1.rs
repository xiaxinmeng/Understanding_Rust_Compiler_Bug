
error: unexpected `if` in the condition expression
 --> /home/gh-compiler-errors/test.rs:2:16
  |
2 |     if true && if true { true } else { false } {}
  |                ^^^
  |
help: remove the `if`
  |
2 -     if true && if true { true } else { false } {}
2 +     if true && true { true } else { false } {}
  |

error[E0308]: mismatched types
 --> /home/gh-compiler-errors/test.rs:2:26
  |
2 |     if true && if true { true } else { false } {}
  |     ---------------------^^^^-----------------
  |     |                    |
  |     |                    expected `()`, found `bool`
  |     expected this to be `()`

error[E0308]: mismatched types
 --> /home/gh-compiler-errors/test.rs:2:40
  |
2 |     if true && if true { true } else { false } {}
  |     -----------------------------------^^^^^--
  |     |                                  |
  |     |                                  expected `()`, found `bool`
  |     expected this to be `()`

error: aborting due to 3 previous errors
