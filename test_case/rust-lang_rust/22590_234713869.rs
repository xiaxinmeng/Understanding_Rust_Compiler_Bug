
error[E0277]: the trait bound `C: A` is not satisfied
  --> <anon>:12:5
   |
12 |     A::a(C);
   |     ^^^^
   |
   = note: required by `A::a`
   = help: implementing `B` for `C` would satisfy `C: A` because of `impl<T: B> A for T`
   = ...
