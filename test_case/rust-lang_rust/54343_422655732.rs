plain
[00:15:32]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:15:32]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:15:32]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:15:33]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:15:34] error[E0425]: cannot find value `mois` in this scope
[00:15:34]   --> librustc_mir/borrow_check/error_reporting.rs:89:78
[00:15:34]    |
[00:15:34] 89 |             if let Some((reported_place, _)) = self.move_error_reported.get(&mois) {
[00:15:34] 
[00:15:34] 
[00:15:34] error[E0425]: cannot find value `mois` in this scope
[00:15:34]   --> librustc_mir/borrow_check/error_reporting.rs:92:40
[00:15:34]    |
[00:15:34] 92 |                            mois={:?}", mois);
[00:15:34] 
[00:15:34] 
[00:15:34] error[E0425]: cannot find value `mois` in this scope
[00:15:34]    --> librustc_mir/borrow_check/error_reporting.rs:185:17
[00:15:34] 185 |                 mois,
[00:15:34]     |                 ^^^^ not found in this scope
[00:15:34] 
[00:15:47] error: aborting due to 3 previous errors
