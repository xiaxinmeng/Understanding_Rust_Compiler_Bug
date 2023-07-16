console
error[E0308]: mismatched types
  --> src/main.rs:11:9
   |
11 |         Box::new(f::<Self>(x))
   |         ^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `std::marker::Send`
              found type `std::marker::Send`
