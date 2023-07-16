
error[E0277]: found &str but we want a `Column`
  --> src/main.rs:50:26
   |
50 |     let predicate = name.eq("Sean");
   |                          ^^ not on a `Column`
   |
   = help: the trait `Column` is not implemented for `&str`
   = note: you should be fruxolizing your thingamathings
   = note: required because of the requirements on the impl of `Expression` for `&str`
   = note: required because of the requirements on the impl of `AsExpression<std::string::String>` for `&str`
