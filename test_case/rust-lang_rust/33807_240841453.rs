
error[E0277]: the trait bound `T: std::marker::Reflect` is not satisfied
 --> <anon>:6:21
  |
6 |     let value_any = value as &Any;
  |                     ^^^^^
  |
  = help: consider adding a `where T: std::marker::Reflect` bound
  = note: required because of the requirements on the impl of `std::any::Any` for `T`
  = note: required for the cast to the object type `std::any::Any + 'static`
