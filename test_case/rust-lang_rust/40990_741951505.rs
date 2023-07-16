
error[E0623]: lifetime mismatch
 --> src/lib.rs:5:16
  |
4 |     fn resolve_symbolic_reference(&self, reference: Option<Reference>) -> Option<Reference> {
  |                                                            ---------      -----------------
  |                                                            |
  |                                                            this parameter and the return type are declared with different lifetimes...
5 |         return reference;
  |                ^^^^^^^^^ ...but data from `reference` is returned here
  = note: by default the lifetime of the return type and `&self` are the same while the method arguments have a different lifetime.
  = note: consider giving `&self`, the arguments, and the return type explicit lifetime annotations.   
