text
error[E0119]: conflicting implementations of trait `std::borrow::Borrow<Foo>` for type `std::boxed::Box<Foo>`:
  --> src/main.rs:25:1
   |
25 | impl Borrow<Foo> for Box<Foo> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: conflicting implementation in crate `alloc`:
           - impl<T> std::borrow::Borrow<T> for std::boxed::Box<T>
             where T: ?Sized;

error[E0275]: overflow evaluating the requirement `<Foo as std::borrow::ToOwned>::Owned`
  |
  = note: required because it appears within the type `Foo`
  = note: required because of the requirements on the impl of `std::borrow::ToOwned` for `Foo`
  = note: required because it appears within the type `Foo`
