
[00:12:10]    Compiling rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[00:12:10] error[E0425]: cannot find value `local_def` in this scope
[00:12:10]    --> /checkout/src/librustc_borrowck/borrowck/mod.rs:690:25
[00:12:10]     |
[00:12:10] 690 |                         local_def = err.cmt.get_def().map(|nid| self.tcx.hir.span(nid));
[00:12:10]     |                         ^^^^^^^^^ not found in this scope
[00:12:10] 
[00:12:11] error: no method named `get_def` found for type `std::rc::Rc<rustc::middle::mem_categorization::cmt_<'tcx>>` in the current scope
[00:12:11]    --> /checkout/src/librustc_borrowck/borrowck/mod.rs:690:45
[00:12:11]     |
[00:12:11] 690 |                         local_def = err.cmt.get_def().map(|nid| self.tcx.hir.span(nid));
[00:12:11]     |                                             ^^^^^^^
[00:12:11] 
[00:12:11] error[E0277]: the trait bound `str: core::marker::Sized` is not satisfied
[00:12:11]    --> /checkout/src/librustc_borrowck/borrowck/mod.rs:666:13
[00:12:11]     |
[00:12:11] 666 |         let msg = match err.code {
[00:12:11]     |             ^^^ the trait `core::marker::Sized` is not implemented for `str`
[00:12:11]     |
[00:12:11]     = note: `str` does not have a constant size known at compile-time
[00:12:11]     = note: all local variables must have a statically known size
[00:12:11] 
[00:12:13] error: aborting due to 2 previous errors
[00:12:13] 
[00:12:13] error: Could not compile `rustc_borrowck`.
[00:12:13] Build failed, waiting for other jobs to finish...
[00:13:06] error: build failed
[00:13:06] 
[00:13:06] 
[00:13:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml"
[00:13:06] expected success, got: exit code: 101
[00:13:06] 
[00:13:06] 
[00:13:06] Build completed unsuccessfully in 0:10:40
[00:13:06] Makefile:24: recipe for target 'all' failed
[00:13:06] make: *** [all] Error 1
