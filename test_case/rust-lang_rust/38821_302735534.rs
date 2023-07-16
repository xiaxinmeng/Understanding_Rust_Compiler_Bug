
error[E0277]: the trait bound `<Col as Expression>::SqlType: NotNull` is not satisfied
  --> <anon>:16:10
   |
16 | #[derive(Copy)]
   |          ^^^^ the trait `NotNull` is not implemented for `<Col as Expression>::SqlType`
   |
   = help: consider adding a `where <Col as Expression>::SqlType: NotNull` bound
   = note: required because of the requirements on the impl of `IntoNullable` for `<Col as Expression>::SqlType`

error: aborting due to previous error

