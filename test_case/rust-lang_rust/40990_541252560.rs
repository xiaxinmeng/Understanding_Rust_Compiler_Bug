
error[E0623]: lifetime mismatch
 --> src/lib.rs:8:12
  |
7 | fn resolve_symbolic_reference(&self, reference: Option<Reference>) -> Option<Reference> {
  |                                                        ---------      -----------------
  |                                                        |
  |                                                        this parameter and the return type are declared with different lifetimes...
8 |     return reference;
  |            ^^^^^^^^^ ...but data from `reference` is returned here
