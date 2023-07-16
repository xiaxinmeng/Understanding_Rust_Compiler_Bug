
> error[E0277]: the trait bound `Self: std::marker::Sized` is not satisfied
  --> <anon>:10:5
   |
10 |       fn res(&self) -> A<Self> {
   |  _____^ starting here...
11 | |         A { a: self.s() }
12 | |     }
   | |_____^ ...ending here: the trait `std::marker::Sized` is not implemented for `Self`
   |
   = help: consider adding a `where Self: std::marker::Sized` bound
   = note: required by `A`
