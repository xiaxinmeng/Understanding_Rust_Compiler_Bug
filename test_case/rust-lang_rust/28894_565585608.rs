
error[E0277]: the trait bound `&str: Column` is not satisfied
  --> file12.rs:39:29
   |
39 |     let predicate = name.eq("Sean");
   |                             ^^^^^^ the trait `Column` is not implemented for `&str`
   |
   = note: required because of the requirements on the impl of `Expression` for `&str`
   = note: required because of the requirements on the impl of `AsExpression<std::string::String>` for `&str`
