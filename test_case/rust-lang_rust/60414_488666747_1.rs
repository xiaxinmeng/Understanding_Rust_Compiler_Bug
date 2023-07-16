
error[E0271]: type mismatch resolving `<Foo as FooLike>::Output == <T as Trait>::Assoc`
  --> src/main.rs:15:12
   |
15 |     let _: impl FooLike<Output = T::Assoc> = Foo;
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found associated type
   |
   = note: expected type `u32`
              found type `<T as Trait>::Assoc`
   = note: the return type of a function must have a statically known size
