
error[E0308]: mismatched types
  --> src/lib.rs:14:5
   |
14 |     Box::new(move |req| h.handle(req))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `for<'r, 's, 't0> Fn<(&'r mut Request<'s, 't0>,)>`
              found type `Fn<(&mut Request<'_, '_>,)>`
note: this closure does not fulfill the lifetime requirements
  --> src/lib.rs:14:14
   |
14 |     Box::new(move |req| h.handle(req))
   |              ^^^^^^^^^^^^^^^^^^^^^^^^
