plain
[00:12:27]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:27]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:28]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:13:29]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:13:30] error[E0491]: in type `&'set mut std::collections::HashSet<rustc::traits::Clause<'tcx>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`, reference has a longer lifetime than the data it references
[00:13:30]   --> librustc_traits/lowering/environment.rs:26:5
[00:13:30]    |
[00:13:30] 26 |     round: &'set mut FxHashSet<Clause<'tcx>>,
[00:13:30]    |
[00:13:30]    |
[00:13:30] note: the pointer is valid for the lifetime 'set as defined on the struct at 24:22
[00:13:30]   --> librustc_traits/lowering/environment.rs:24:22
[00:13:30]    |
[00:13:30] 24 | struct ClauseVisitor<'set, 'a, 'tcx: 'a> {
[00:13:30]    |                      ^^^^
[00:13:30] note: but the referenced data is only valid for the lifetime 'tcx as defined on the struct at 24:32
[00:13:30]   --> librustc_traits/lowering/environment.rs:24:32
[00:13:30]    |
[00:13:30] 24 | struct ClauseVisitor<'set, 'a, 'tcx: 'a> {
[00:13:30] 
[00:13:30] error: aborting due to previous error
[00:13:30] 
[00:13:30] For more information about this error, try `rustc --explain E0491`.
