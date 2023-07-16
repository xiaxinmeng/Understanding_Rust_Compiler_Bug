plain
travis_time:end:19a15e80:start=1551175424037414733,finish=1551175503155178470,duration=79117763737
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:39] 
[01:26:39] running 119 tests
[01:27:05] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:27:09] i......iii.i.....ii
[01:27:09] 
[01:27:09]  finished in 29.790
[01:27:09] travis_fold:end:test_debuginfo

---
[01:38:23] 
[01:38:23]    Doc-tests core
[01:38:29] 
[01:38:29] running 2278 tests
[01:38:41] ....iiiii.....................................F..................................................... 100/2278
[01:39:09] ..........................................................................................i......... 300/2278
[01:39:27] .................................................................................................... 400/2278
[01:39:41] ......................i..i.......................................................................... 500/2278
[01:39:55] .................................................................................................... 600/2278
---
[01:44:04] ---- cell.rs - cell::DowngradedRef::map (line 1445) stdout ----
[01:44:04] error[E0599]: no method named `get_or_insert` found for type `std::collections::hash_map::Entry<'_, i32, i32>` in the current scope
[01:44:04]   --> cell.rs:1455:21
[01:44:04]    |
[01:44:04] 13 |     |h| h.entry(12).get_or_insert(1)).into());
[01:44:04]    |                     ^^^^^^^^^^^^^ help: did you mean: `or_insert`
[01:44:04] error[E0599]: no method named `into` found for type `std::cell::RefMut<'_, _>` in the current scope
[01:44:04]   --> cell.rs:1455:39
[01:44:04]    |
[01:44:04]    |
[01:44:04] 13 |     |h| h.entry(12).get_or_insert(1)).into());
[01:44:04]    |
[01:44:04]    = note: RefMut::map(
[01:44:04]                hash.borrow_mut(),
[01:44:04]                hash.borrow_mut(),
[01:44:04]                |h| h.entry(12).get_or_insert(1)) is a function, perhaps you wish to call it
[01:44:04] error[E0308]: mismatched types
[01:44:04]   --> cell.rs:1457:44
[01:44:04]    |
[01:44:04]    |
[01:44:04] 15 | let r2 = Ref::map(hash.borrow(), |h| h.get(12).unwrap());
[01:44:04]    |                                            ^^ expected &i32, found integer
[01:44:04]    = note: expected type `&i32`
[01:44:04]               found type `{integer}`
[01:44:04] 
[01:44:04] 
[01:44:04] thread 'cell.rs - cell::DowngradedRef::map (line 1445)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:44:04] 
[01:44:04] 
[01:44:04] failures:
[01:44:04]     cell.rs - cell::DowngradedRef::map (line 1445)
[01:44:04]     cell.rs - cell::DowngradedRef::map (line 1445)
[01:44:04] 
[01:44:04] test result: FAILED. 2266 passed; 1 failed; 11 ignored; 0 measured; 0 filtered out
[01:44:04] 
[01:44:04] error: test failed, to rerun pass '--doc'
[01:44:04] 
[01:44:04] 
[01:44:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:44:04] 
[01:44:04] 
[01:44:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:04] Build completed unsuccessfully in 0:29:17
[01:44:04] Build completed unsuccessfully in 0:29:17
[01:44:04] Makefile:48: recipe for target 'check' failed
[01:44:04] make: *** [check] Error 1
141172 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141172 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141168 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn/s-f9uglu5j0e-3kn5yf-okq6jwyp562e
138820 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
136000 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123516 ./src/llvm-project/llvm/test/CodeGen
118360 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib
