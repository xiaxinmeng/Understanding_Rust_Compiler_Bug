
error[E0277]: the trait bound `for<'a> <_ as Yokeable<'a>>::Output: IsCovariant<'a>` is not satisfied
  --> test.rs:41:5
   |
41 |     upcast(y)
   |     ^^^^^^ the trait `for<'a> IsCovariant<'a>` is not implemented for `<_ as Yokeable<'a>>::Output`
   |
note: required by a bound in `upcast`
  --> test.rs:21:42
   |
19 | fn upcast<Y>(x: Yoke<Y>) -> Yoke<Box<dyn IsCovariant<'static> + 'static>> where
   |    ------ required by a bound in this
20 |     Y: for<'a> Yokeable<'a>,
21 |     for<'a> <Y as Yokeable<'a>>::Output: IsCovariant<'a>
   |                                          ^^^^^^^^^^^^^^^ required by this bound in `upcast`
