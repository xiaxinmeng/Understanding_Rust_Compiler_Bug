plain
travis_time:end:07a7f814:start=1560014959062390806,finish=1560014959809358278,duration=746967472
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:47] 
[01:06:47] running 144 tests
[01:06:50] i..iii.....iii..i.iii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:06:52] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:06:52] 
[01:06:52]  finished in 4.723
[01:06:52] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:54] 
[01:06:54] running 9 tests
[01:06:54] iiiiiiiii
[01:06:54] 
[01:06:54]  finished in 0.154
[01:06:54] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:11] 
[01:07:11] running 122 tests
[01:07:36] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:41] .i.i......iii.i.....ii
[01:07:41] 
[01:07:41]  finished in 30.954
[01:07:41] travis_fold:end:test_debuginfo

---
[01:24:13] ....iiiii........................................................................................... 100/2328
[01:24:26] .......................................................................ii........................... 200/2328
[01:24:41] ...........................................................................................i........ 300/2328
[01:24:58] .................................................................................................... 400/2328
[01:25:11] ........................i..i............................F........................................... 500/2328
[01:25:24] F.F................................................................................................. 600/2328
[01:25:51] .................................................................................................... 800/2328
[01:26:05] .................................................................................................... 900/2328
[01:26:18] .................................................................................................... 1000/2328
[01:26:32] .................................................................................................... 1100/2328
---
[01:27:55] .................................................................................................... 1700/2328
[01:28:09] .................................................................................................... 1800/2328
[01:28:23] .................................................................................................... 1900/2328
[01:28:39] .................................................................................................... 2000/2328
[01:28:54] ...................................F................................................................ 2100/2328
[01:29:30] ...................................i................................................................ 2300/2328
[01:29:34] ............................
[01:29:34] failures:
[01:29:34] 
[01:29:34] 
[01:29:34] ---- marker.rs - marker::Sized (line 67) stdout ----
[01:29:34] error: trait objects without an explicit `dyn` are deprecated
[01:29:34]   --> marker.rs:76:9
[01:29:34]    |
[01:29:34] 12 | let x: &Foo = &Impl;    // OK
[01:29:34]    |
[01:29:34] note: lint level defined here
[01:29:34]   --> marker.rs:65:9
[01:29:34]    |
---
[01:29:34] ---- mem/mod.rs - mem::take (line 512) stdout ----
[01:29:34] error[E0658]: use of unstable library feature 'mem_take'
[01:29:34]  --> mem/mod.rs:517:13
[01:29:34]   |
[01:29:34] 8 | let old_v = mem::take(&mut v);
[01:29:34]   |
[01:29:34]   = note: for more information, see https://github.com/rust-lang/rust/issues/61129
[01:29:34]   = help: add #![feature(mem_take)] to the crate attributes to enable
[01:29:34] 
---
[01:29:34] ---- mem/mod.rs - mem::take (line 542) stdout ----
[01:29:34] error[E0658]: use of unstable library feature 'mem_take'
[01:29:34]   --> mem/mod.rs:549:9
[01:29:34]    |
[01:29:34] 10 |         mem::take(&mut self.buf)
[01:29:34]    |
[01:29:34]    = note: for more information, see https://github.com/rust-lang/rust/issues/61129
[01:29:34]    = help: add #![feature(mem_take)] to the crate attributes to enable
[01:29:34] 
---
[01:29:34] ---- raw.rs - raw::TraitObject (line 37) stdout ----
[01:29:34] error: trait objects without an explicit `dyn` are deprecated
[01:29:34]   --> raw.rs:56:14
[01:29:34]    |
[01:29:34] 22 | let object: &Foo = &value;
[01:29:34]    |
[01:29:34] note: lint level defined here
[01:29:34]   --> raw.rs:35:9
[01:29:34]    |
---
[01:29:34] 
[01:29:34] error: test failed, to rerun pass '--doc'
[01:29:34] 
[01:29:34] 
[01:29:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:29:34] 
[01:29:34] 
[01:29:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:34] Build completed unsuccessfully in 1:24:39
