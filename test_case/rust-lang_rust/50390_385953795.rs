plain
[00:50:25] ............................................................................ii......................
[00:51:12] ........................................i....................................................i.ii...
[00:51:23] ............................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:51:51] ........................................................................
[00:52:13] .iiiiiii............................................................................................
[00:52:52] ....................................................................................................
[00:53:09] ..........................................................................
[00:53:09] test result: ok. 2955 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[00:53:09] 
---
[01:25:31] 
[01:25:31] failures:
[01:25:31] 
[01:25:31] ---- /checkout/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::incoherent_fundamental_impls (line 125) stdout ----
[01:25:31]  error: conflicting implementations of trait `main::Trait1<std::boxed::Box<_>>` for type `main::A`: (E0119)
[01:25:31]   --> /checkout/src/doc/rustc/src/lints/listing/warn-by-default.md:138:1
[01:25:31]    |
[01:25:31] 11 | impl<X, T> Trait1<X> for T where T: Trait2<X> {
[01:25:31]    | --------------------------------------------- first implementation here
[01:25:31] ...
[01:25:31] 15 | impl<X> Trait1<Box<X>> for A {
[01:25:31]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `main::A`
[01:25:31]    |
[01:25:31]    = note: #[deny(incoherent_fundamental_impls)] on by default
[01:25:31]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[01:25:31]    = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[01:25:31]    = note: downstream crates may implement trait `main::Trait2<std::boxed::Box<_>>` for type `main::A`
[01:25:31] thread '/checkout/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::incoherent_fundamental_impls (line 125)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:25:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:25:31] 
[01:25:31] 
---
[01:25:31] 
[01:25:31] 
[01:25:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:31] Build completed unsuccessfully in 0:45:28
[01:25:31] Makefile:58: recipe for target 'check' failed
[01:25:31] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03aa0ab0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
