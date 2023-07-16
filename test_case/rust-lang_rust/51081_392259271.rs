plain
[00:45:51] ..........................................................i.........................................
[00:45:55] ....................................................................................................
[00:46:01] ....................................................................................................
[00:46:08] .....................................................................................i..............
[00:46:10] ...iiiiiiiii...................................................
[00:46:10] 
[00:46:10] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:59] ..........................................................i.........................................
[00:47:04] ....................................................................................................
[00:47:09] ....................................................................................................
[00:47:15] .....................................................................................i..............
[00:47:18] ...iiiiiiiii...................................................
[00:47:18] 
[00:47:18]  finished in 67.848
[00:47:18] travis_fold:end:test_ui_nll

---
[01:15:23] 
[01:15:23]    Doc-tests std
[01:15:25] 
[01:15:25] running 926 tests
[01:15:57] ii......F...........................................................................................
[01:16:26] ...i......i...i......i..............................................................................
[01:16:34] ....................................................................................................
[01:16:51] ................iiii........ii......................................................................
[01:16:59] ....................................................................................................
[01:16:59] ....................................................................................................
[01:17:15] ...............................................................iiii.................................
[01:17:38] ....................................................................................................
[01:17:48] ...........................................................................iiii.....................
[01:17:52] ..........................
[01:17:52] failures:
[01:17:52] 
[01:17:52] ---- collections/hash/map.rs - collections::hash::map::HashMap (line 274) stdout ----
[01:17:52] error[E0277]: the trait bound `std::string::String: std::borrow::Borrow<&str>` is not satisfied
[01:17:52]   --> collections/hash/map.rs:305:24
[01:17:52]    |
[01:17:52] 34 |     match book_reviews.get(book) {
[01:17:52]    |                        ^^^ the trait `std::borrow::Borrow<&str>` is not implemented for `std::string::String`
[01:17:52]    |
[01:17:52]    = help: the following implementations were found:
[01:17:52]              <std::string::String as std::borrow::Borrow<str>>
[01:17:52] thread 'collections/hash/map.rs - collections::hash::map::HashMap (line 274)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:17:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:17:52] 
[01:17:52] 
---
[01:17:52] 
[01:17:52] error: test failed, to rerun pass '--doc'
[01:17:52] 
[01:17:52] 
[01:17:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:17:52] 
[01:17:52] 
[01:17:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:52] Build completed unsuccessfully in 0:34:22
[01:17:52] Build completed unsuccessfully in 0:34:22
[01:17:52] Makefile:58: recipe for target 'check' failed
[01:17:52] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:015afa76
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
