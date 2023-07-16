
error[E0277]: the trait bound `&str: Column` is not satisfied
  --> src/main.rs:39:29
   |
39 |     let predicate = name.eq("Sean");
   |                          -- ^^^^^^ the trait `Column` is not implemented for `&str`
   |                          |
   |                          required by a bound introduced by this call
   |
   = help: the trait `Column` is implemented for `name`
note: required for `&str` to implement `Expression`
  --> src/main.rs:28:17
   |
28 | impl<C: Column> Expression for C {
   |         ------  ^^^^^^^^^^     ^
   |         |
   |         unsatisfied trait bound introduced here
note: required for `&str` to implement `AsExpression<String>`
  --> src/main.rs:16:21
   |
16 | impl<E: Expression> AsExpression<E::SqlType> for E {
   |         ----------  ^^^^^^^^^^^^^^^^^^^^^^^^     ^
   |         |
   |         unsatisfied trait bound introduced here
note: required by a bound in `Expression::eq`
  --> src/main.rs:4:14
   |
4  |     fn eq<T: AsExpression<Self::SqlType>>(self, other: T) {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Expression::eq`
