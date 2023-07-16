
[00:22:33] error: internal compiler error: /checkout/src/librustc/hir/def.rs:162: attempted .def_id() on invalid def: SelfTy(None, Some(DefId { krate: CrateNum(0), node: DefIndex(3530) => rustc/3eeda9a::middle[0]::free_region[0]::{{impl}}[0] }))
[00:22:33] 
[00:22:34] note: the compiler unexpectedly panicked. this is a bug.
[00:22:34] 
[00:22:34] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:22:34] 
[00:22:34] note: rustc 1.21.0-dev (0bd7b6f4b 2017-08-06) running on x86_64-unknown-linux-gnu
[00:22:34] 
[00:22:34] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:486:8
