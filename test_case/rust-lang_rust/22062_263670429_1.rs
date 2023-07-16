
error[E0309]: the parameter type `T` may not live long enough
  --> src\lib.rs:14:11
   |
14 |     &self.foo().member
   |           ^^^
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the reference type `&Foo<T>` does not outlive the data it points at
  --> src\lib.rs:14:11
   |
14 |     &self.foo().member
   |           ^^^
