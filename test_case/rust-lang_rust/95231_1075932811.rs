plain
    Checking dirs-sys-next v0.1.2
    Checking num_cpus v1.13.1
    Checking parking_lot_core v0.8.5
    Checking filetime v0.2.14
error[E0004]: non-exhaustive patterns: `Trivial(_)` not covered
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:28:19
    |
28  |             match predicate.kind().skip_binder() {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `Trivial(_)` not covered
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:633:5
    |
    |
633 |     Trivial(Ty<'tcx>),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `PredicateKind`

