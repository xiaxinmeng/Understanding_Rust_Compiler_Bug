
[00:18:29] error[E0284]: type annotations required: cannot resolve `<[&[rustc::ty::Predicate<'_>]] as std::slice::SliceConcatExt<_>>::Output == std::vec::Vec<rustc::ty::Predicate<'_>>`
[00:18:29]     --> librustc_typeck/collect.rs:1303:75
[00:18:29]      |
[00:18:29] 1303 |         [&explicit.predicates[..], &tcx.inferred_outlives_of(def_id)[..]].concat()
[00:18:29] 
[00:18:30] error: aborting due to previous error
