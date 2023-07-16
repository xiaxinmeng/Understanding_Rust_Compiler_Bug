plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0433]: failed to resolve: use of undeclared type `LookupResult`
   --> compiler/rustc_mir_dataflow/src/impls/mod.rs:324:17
    |
324 |             let LookupResult::Exact(mpi) = self.move_data().rev_lookup.find(place.as_ref()) else { return };
    |                 ^^^^^^^^^^^^ use of undeclared type `LookupResult`
error[E0433]: failed to resolve: use of undeclared type `LookupResult`
   --> compiler/rustc_mir_dataflow/src/impls/mod.rs:346:17
    |
    |
346 |             let LookupResult::Exact(mpi) = self.move_data().rev_lookup.find(place.as_ref()) else { return };
    |                 ^^^^^^^^^^^^ use of undeclared type `LookupResult`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_mir_dataflow` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
