
error[E0308]: mismatched types
  --> src/a.rs:14:5
   |
14 |     Box::new(move |req| h.handle(req))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `for<'r, 's, 't0> std::ops::Fn<(&'r mut a::Request<'s, 't0>,)>`
              found type `std::ops::Fn<(&mut a::Request<'_, '_>,)>`
