bash
error[E0277]: the trait bound `X: std::marker::Sized` is not satisfied
  --> /home/vagrant/repos/rust/src/test/compile-fail/unsized6.rs:37:18
   |
37 | fn g1<X: ?Sized>(x: X) {} //~ERROR `X: std::marker::Sized` is not satisfied
   |                  ^ `X` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `X`
   = help: consider adding a `where X: std::marker::Sized` bound
   = note: all local variables must have a statically known size

error[E0277]: the trait bound `X: std::marker::Sized` is not satisfied
  --> /home/vagrant/repos/rust/src/test/compile-fail/unsized6.rs:37:24
   |
37 | fn g1<X: ?Sized>(x: X) {} //~ERROR `X: std::marker::Sized` is not satisfied
   |                        ^ `X` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `X`
   = help: consider adding a `where X: std::marker::Sized` bound
