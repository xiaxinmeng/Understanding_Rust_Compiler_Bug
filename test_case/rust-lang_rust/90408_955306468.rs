plain
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error[E0277]: the trait bound `LocalDefId: std::cmp::Ord` is not satisfied
    |
    |
291 | impl Idx for LocalDefId {
    |      ^^^ the trait `std::cmp::Ord` is not implemented for `LocalDefId`
    |
note: required by a bound in `Idx`
    |
    |
15  | pub trait Idx: Copy + 'static + Ord + Debug + Hash {
    |                                 ^^^ required by this bound in `Idx`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_span` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
