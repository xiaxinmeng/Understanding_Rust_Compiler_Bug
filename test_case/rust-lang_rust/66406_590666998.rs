
error: `impl` item signature doesn't match `trait` item signature
  --> file5.rs:37:5
   |
31 | pub trait Bar<To> {
   |               -- this type parameter might not have a lifetime compatible with the `impl`
32 |     fn get_relation(&self) -> To;
   |     -----------------------------
   |     |                         |
   |     |                         you might want to borrow this type parameter in the trait to make it match the `impl`
   |     expected `fn(&Article) -> &ProofReader`
...
37 |     fn get_relation(&self) -> &ProofReader {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&Article) -> &ProofReader`
   |
   = note: expected `fn(&Article) -> &ProofReader`
              found `fn(&Article) -> &ProofReader`
   = note: the lifetime requirements from the `trait` could not be satisfied by the `impl`
   = help: verify the lifetime relationships in the `trait` and `impl` between the `self` argument, the other inputs and its output
