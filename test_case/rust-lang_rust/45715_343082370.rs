
[01:19:46] error[E0608]: cannot index into a value of type `std::rc::Rc<std::collections::HashSet<rustc::hir::ItemLocalId, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>>`
[01:19:46]    --> src/tools/clippy/clippy_lints/src/methods.rs:893:26
[01:19:46]     |
[01:19:46] 893 |         let promotable = cx.tcx.rvalue_promotable_map(owner_def)[&arg.hir_id.local_id];
[01:19:46]     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:19:46] 
[01:19:47] error: aborting due to previous error
[01:19:47] 
[01:19:47] error: Could not compile `clippy_lints`.
[01:19:47] warning: build failed, waiting for other jobs to finish...
[01:19:55] error: build failed
