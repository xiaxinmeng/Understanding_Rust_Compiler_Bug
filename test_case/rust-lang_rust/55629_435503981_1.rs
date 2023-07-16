
error[E0308]: mismatched types
  --> foo.rs:16:26
   |
16 |     let a: Box<A<Bar>> = b;
   |                          ^ expected trait `A`, found trait `B`
   |
   = note: expected type `std::boxed::Box<dyn A<Bar>>`
              found type `std::boxed::Box<dyn B<Bar>>`
