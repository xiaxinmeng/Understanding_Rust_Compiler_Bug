

error[E0308]: mismatched types
  --> src/lib.rs:10:5
   |
9  | fn foo<T: ?Sized + Object<U>, U>(x: <T as Object<U>>::Output) -> U {
   |                                                                  - expected `U` because of return type
10 |     x
   |     ^ expected type parameter, found associated type
   |
   = note: expected type `U`
              found type `<T as Object<U>>::Output`
