plain
[00:50:30] .......................................................................i............................
[00:50:35] ....................................................................................................
[00:50:41] ....................................................................................................
[00:50:47] ....................................................................................................
[00:50:51] i.................iiiiiiiii...................................................
[00:50:51] 
[00:50:51] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:51:41] .......................................................................i............................
[00:51:46] ....................................................................................................
[00:51:51] ....................................................................................................
[00:51:57] ....................................................................................................
[00:52:00] i..................iiiiiiiii..................................................
[00:52:00] 
[00:52:00]  finished in 69.392
[00:52:00] travis_fold:end:test_ui_nll

---
[01:35:50] test /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13) ... FAILED
[01:35:50] 
[01:35:50] failures:
[01:35:50] 
[01:35:50] ---- /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13) stdout ----
[01:35:50] error[E0412]: cannot find type `Any` in this scope
[01:35:50]  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:17:16
[01:35:50]   |
[01:35:50] 5 |     let x: Box<Any> = Box::new(42);
[01:35:50]   |                ^^^ not found in this scope
[01:35:50] help: possible candidate is found in another module, you can import it into scope
[01:35:50] 4 | use std::any::Any;
[01:35:50]   |
[01:35:50] 
[01:35:50] 
[01:35:50] error[E0412]: cannot find type `Any` in this scope
[01:35:50]  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:18:12
[01:35:50]   |
[01:35:50] 6 |     let x: Any = *x;
[01:35:50]   |            ^^^ not found in this scope
[01:35:50] help: possible candidate is found in another module, you can import it into scope
[01:35:50] 4 | use std::any::Any;
[01:35:50]   |
[01:35:50] 
[01:35:50] 
[01:35:50] error[E0412]: cannot find type `Any` in this scope
[01:35:50]   --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:24:11
[01:35:50]    |
[01:35:50] 12 | fn foo(_: Any) {}
[01:35:50]    |           ^^^ not found in this scope
[01:35:50] help: possible candidate is found in another module, you can import it into scope
[01:35:50] 4  | use std::any::Any;
[01:35:50]    |
[01:35:50] 
[01:35:50] thread '/checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
---
[01:35:50] 
[01:35:50] 
[01:35:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:50] Build completed unsuccessfully in 0:47:47
[01:35:50] make: *** [check] Error 1
[01:35:50] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:004fb200
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
