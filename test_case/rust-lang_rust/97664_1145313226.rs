plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0277]: the trait bound `rustc_middle::ty::Predicate<'_>: Ord` is not satisfied
   --> compiler/rustc_typeck/src/coherence/builtin.rs:155:58
    |
155 |             for ((ty, error_predicate), spans) in errors.into_sorted_vector() {
    |                                                          ^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_middle::ty::Predicate<'_>`
    |
    = help: the following other types implement trait `Ord`:
              ()
              (A, B, C, D, E, F, G, H, I, J, K, L)
              (B, C, D, E, F, G, H, I, J, K, L)
              (C, D, E, F, G, H, I, J, K, L)
              (D, E, F, G, H, I, J, K, L)
              (E, F, G, H, I, J, K, L)
              (F, G, H, I, J, K, L)
              (G, H, I, J, K, L)
            and 5 others
    = note: required because of the requirements on the impl of `Ord` for `(rustc_middle::ty::Ty<'_>, rustc_middle::ty::Predicate<'_>)`
note: required by a bound in `StableMap::<K, V>::into_sorted_vector`
    |
    |
62  |         K: Ord + Copy,
    |            ^^^ required by this bound in `StableMap::<K, V>::into_sorted_vector`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_typeck` due to previous error
