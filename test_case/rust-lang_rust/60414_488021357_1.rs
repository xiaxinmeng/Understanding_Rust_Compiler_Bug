
error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as Trait>::Assoc`
  --> src/main.rs:13:35
   |
13 | fn foo<T: Trait<Assoc = ()>>() -> impl FooLike<Output = T::Assoc> {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found associated type
   |
   = note: expected type `()`
              found type `<T as Trait>::Assoc`
   = note: the return type of a function must have a statically known size
