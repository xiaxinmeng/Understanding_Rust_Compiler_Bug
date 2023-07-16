
error: the trait bound `C: B` is not satisfied [--explain E0277]
  --> <anon>:18:5
18 |>     A::a(C);
   |>     ^^^^
note: required because of the requirements on the impl of `A` for `C`
note: required by `A::a`

error: aborting due to previous error
