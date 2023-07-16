plain
[00:44:50] .................................................................i..................................
[00:44:54] ....................................................................................................
[00:45:00] ....................................................................................................
[00:45:06] ..............................................................................................i.....
[00:45:09] ............iiiiiiiii...................................................
[00:45:09] 
[00:45:09] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:45:57] .................................................................i..................................
[00:46:01] ....................................................................................................
[00:46:06] ....................................................................................................
[00:46:12] ..............................................................................................i.....
[00:46:15] ............iiiiiiiii...................................................
[00:46:15] 
[00:46:15]  finished in 65.646
[00:46:15] travis_fold:end:test_ui_nll

---
[01:25:24] test /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13) ... FAILED
[01:25:24] 
[01:25:24] failures:
[01:25:24] 
[01:25:24] ---- /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 73) stdout ----
[01:25:24] error[E0277]: the trait bound `Self: std::marker::Sized` is not satisfied
[01:25:24]  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:75:12
[01:25:24]   |
[01:25:24] 4 |     fn foo(self) {}
[01:25:24]   |            ^^^^ `Self` does not have a constant size known at compile-time
[01:25:24]   |
[01:25:24]   = help: the trait `std::marker::Sized` is not implemented for `Self`
[01:25:24]   = help: consider adding a `where Self: std::marker::Sized` bound
[01:25:24]   = note: all local variables must have a statically known size
[01:25:24]   = help: unsized locals are gated as an unstable feature
[01:25:24] thread '/checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 73)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:25:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:25:24] 
[01:25:24] ---- /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13) stdout ----
[01:25:24] ---- /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13) stdout ----
[01:25:24] error[E0412]: cannot find type `Any` in this scope
[01:25:24]  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:17:16
[01:25:24]   |
[01:25:24] 5 |     let x: Box<Any> = Box::new(42);
[01:25:24]   |                ^^^ not found in this scope
[01:25:24] help: possible candidate is found in another module, you can import it into scope
[01:25:24] 4 | use std::any::Any;
[01:25:24]   |
[01:25:24] 
[01:25:24] 
[01:25:24] error[E0412]: cannot find type `Any` in this scope
[01:25:24]  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:18:12
[01:25:24]   |
[01:25:24] 6 |     let x: Any = *x;
[01:25:24]   |            ^^^ not found in this scope
[01:25:24] help: possible candidate is found in another module, you can import it into scope
[01:25:24] 4 | use std::any::Any;
[01:25:24]   |
[01:25:24] 
[01:25:24] 
[01:25:24] error[E0412]: cannot find type `Any` in this scope
[01:25:24]   --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:24:11
[01:25:24]    |
[01:25:24] 12 | fn foo(_: Any) {}
[01:25:24]    |           ^^^ not found in this scope
[01:25:24] help: possible candidate is found in another module, you can import it into scope
[01:25:24] 4  | use std::any::Any;
[01:25:24]    |
[01:25:24] 
[01:25:24] thread '/checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
---
[01:25:24] 
[01:25:24] 
[01:25:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:24] Build completed unsuccessfully in 0:42:50
[01:25:24] make: *** [check] Error 1
[01:25:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c559ae4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
