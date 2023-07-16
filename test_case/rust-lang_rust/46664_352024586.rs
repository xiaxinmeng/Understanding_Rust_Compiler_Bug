
[01:02:25]  error[E0038]: the trait `Foo` cannot be made into an object
[01:02:25]    --> $DIR/arbitrary-self-types-not-object-safe.rs:40:33
[01:02:25]     |
[01:02:25]  40 |     let x = Box::new(5usize) as Box<Foo>;
[01:02:25]     |                                 ^^^^^^^^ the trait `Foo` cannot be made into an object
[01:02:25]     |
[01:02:25] -   = note: method `foo` has a non-standard `self` type. Only `&self`, `&mut self`, and `Box<Self>` are currently supported for trait objects
[01:02:25] +   = note: method `foo` has a non-standard `self` type
[01:02:25]  
[01:02:25]  error[E0038]: the trait `Foo` cannot be made into an object
[01:02:25]    --> $DIR/arbitrary-self-types-not-object-safe.rs:40:13
[01:02:25]     |
[01:02:25]  40 |     let x = Box::new(5usize) as Box<Foo>;
[01:02:25]     |             ^^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
[01:02:25]     |
[01:02:25] -   = note: method `foo` has a non-standard `self` type. Only `&self`, `&mut self`, and `Box<Self>` are currently supported for trait objects
[01:02:25] +   = note: method `foo` has a non-standard `self` type
[01:02:25]     = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<std::boxed::Box<Foo>>` for `std::boxed::Box<usize>`
[01:02:25]  
[01:02:25]  error: aborting due to 2 previous errors
[01:02:25]  
[01:02:25]  
[01:02:25] failures:
[01:02:25]     [ui] ui/arbitrary-self-types-not-object-safe.rs
