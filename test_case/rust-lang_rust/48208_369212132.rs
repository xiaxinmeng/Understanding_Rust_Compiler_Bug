
[00:13:51]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:56] error[E0599]: no method named `nll` found for type `&rustc::session::Session` in the current scope
[00:13:56]     --> librustc_mir/borrow_check/nll/type_check/mod.rs:1590:21
[00:13:56]      |
[00:13:56] 1590 |         if tcx.sess.nll() {
[00:13:56]      |                     ^^^
[00:13:56] 
[00:14:03] error: aborting due to previous error
[00:14:03] 
[00:14:03] error: Could not compile `rustc_mir`.
