plain
   Compiling ansi_term v0.11.0
   Compiling difference v2.0.0
   Compiling pretty_assertions v0.6.1
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0445]: crate-private trait `builder::Step` in public interface
   --> src/bootstrap/cache.rs:274:5
    |
274 |     pub fn all<S: Ord + Step>(&mut self) -> Vec<(S, S::Output)> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak crate-private trait
   ::: src/bootstrap/builder.rs:54:1
    |
    |
54  | pub(crate) trait Step: 'static + Clone + Debug + PartialEq + Eq + Hash {
    | ---------------------------------------------------------------------- `builder::Step` declared as crate-private

error[E0445]: crate-private trait `builder::Step` in public interface
   --> src/bootstrap/cache.rs:286:5
    |
286 |     pub fn contains<S: Step>(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak crate-private trait
   ::: src/bootstrap/builder.rs:54:1
    |
    |
54  | pub(crate) trait Step: 'static + Clone + Debug + PartialEq + Eq + Hash {
    | ---------------------------------------------------------------------- `builder::Step` declared as crate-private
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0445`.
error: could not compile `bootstrap`
