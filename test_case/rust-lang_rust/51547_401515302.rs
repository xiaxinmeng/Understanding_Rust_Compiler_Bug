rust
error[E0424]: expected unit struct/variant or constant, found local variable `self`
  --> expr.rs:18:50
   |
18 |     struct S; impl S { fn foo(self) { let self = 42; } }
   |                                           ^^^^ `self` value is only available in methods with `self` parameter
