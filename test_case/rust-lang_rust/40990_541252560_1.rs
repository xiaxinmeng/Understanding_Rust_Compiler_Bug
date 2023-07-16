
error: lifetime may not live long enough
  --> src/lib.rs:10:12
   |
9  | fn resolve_symbolic_reference(&self, reference: Option<Reference>) -> Option<Reference> {
   |                               -      --------- has type `std::option::Option<Reference<'1>>`
   |                               |
   |                               let's call the lifetime of this reference `'2`
10 |     return reference;
   |            ^^^^^^^^^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
