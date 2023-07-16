plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0507]: cannot move out of `default_suggestion`, a captured variable in an `FnMut` closure
     |
     |
2036 |                       let (default_msg, default_suggestion) =
     |                                         ------------------ captured outer variable
...
2046 |                       .find_map(|(ty, _)| {
2047 | |                         match ty.kind() {
2047 | |                         match ty.kind() {
2048 | |                             ty::Adt(adt_def, _)
2049 | |                                 if self.tcx.is_diagnostic_item(sym::Option, adt_def.did)
2062 | |                                 default_suggestion,
2062 | |                                 default_suggestion,
     | |                                 ^^^^^^^^^^^^^^^^^^ move occurs because `default_suggestion` has type `std::string::String`, which does not implement the `Copy` trait
2067 | |                         }
2068 | |                     })
2068 | |                     })
     | |_____________________- captured by this `FnMut` closure
error[E0382]: use of moved value: `default_suggestion`
    --> compiler/rustc_typeck/src/check/pat.rs:2071:25
     |
     |
2036 |                     let (default_msg, default_suggestion) =
     |                                       ------------------ move occurs because `default_suggestion` has type `std::string::String`, which does not implement the `Copy` trait
...
2046 |                     .find_map(|(ty, _)| {
     |                               --------- value moved into closure here
2062 |                                 default_suggestion,
     |                                 ------------------ variable moved due to use in closure
...
2071 |                         default_suggestion,
2071 |                         default_suggestion,
     |                         ^^^^^^^^^^^^^^^^^^ value used here after move
Some errors have detailed explanations: E0382, E0507.
For more information about an error, try `rustc --explain E0382`.
error: could not compile `rustc_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
