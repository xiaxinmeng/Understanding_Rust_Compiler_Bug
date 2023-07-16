
error[E0277]: the trait bound `&Self: T` is not satisfied
 --> <anon>:2:41
  |
2 |   fn t(&self) -> &T where Self: Sized { &self }
  |                                         ^^^^^
  |
  = note: required for the cast to the object type `T`

error: aborting due to previous error
