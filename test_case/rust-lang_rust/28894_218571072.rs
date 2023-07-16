
error: the trait bound `(): Column` is not satisfied [--explain E0277]
  --> <anon>:11:5
11 |>     assert_expression::<()>();
   |>     ^^^^^^^^^^^^^^^^^^^^^^^
note: required because of the requirements on the impl of `Expression` for `()`
note: required by `assert_expression`

error: aborting due to previous error
playpen: application terminated with error code 101
