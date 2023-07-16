plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied in `(std::string::String, std::string::String)`
   --> compiler/rustc_typeck/src/coherence/builtin.rs:155:58
    |
155 |             for ((ty, error_predicate), spans) in errors.into_sorted_vector() {
    |                                                          ^^^^^^^^^^^^^^^^^^ within `(std::string::String, std::string::String)`, the trait `std::marker::Copy` is not implemented for `std::string::String`
    = note: required because it appears within the type `(std::string::String, std::string::String)`
note: required by a bound in `StableMap::<K, V>::into_sorted_vector`
   --> /checkout/compiler/rustc_data_structures/src/stable_map.rs:62:18
    |
    |
62  |         K: Ord + Copy,
    |                  ^^^^ required by this bound in `StableMap::<K, V>::into_sorted_vector`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_typeck` due to previous error
