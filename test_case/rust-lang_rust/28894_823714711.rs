
error[E0277]: the trait bound `&str: Column` is not satisfied
  --> src/main.rs:42:29
   |
42 |     let predicate = name.eq("Sean");
   |                             ^^^^^^ the trait `Column` is not implemented for `&str`
   |
note: required because of the requirements on the impl of `Expression` for `&str`
  --> src/main.rs:31:17
   |
31 | impl<C: Column> Expression for C {
   |                 ^^^^^^^^^^     ^
note: required because of the requirements on the impl of `AsExpression<String>` for `&str`
  --> src/main.rs:19:21
   |
19 | impl<E: Expression> AsExpression<E::SqlType> for E {
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^     ^
