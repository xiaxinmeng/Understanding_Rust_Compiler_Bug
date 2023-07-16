
error[E0277]: the trait bound `&str: Column` is not satisfied
  --> src/main.rs:43:29
   |
43 |     let predicate = name.eq("Sean");
   |                             ^^^^^^ the trait `Column` is not implemented for `&str`
   |
note: required because of the requirements on the impl of `Expression` for `&str`
  --> src/main.rs:29:1
   |
29 | Expression // Make sure that the type
   | ^^^^^^^^^^
30 | for        // of the value you're using
31 | C          // matches the table definition
   | ^
note: required because of the requirements on the impl of `AsExpression<String>` for `&str`
  --> src/main.rs:16:21
   |
16 | impl<E: Expression> AsExpression<E::SqlType> for E {
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^     ^
