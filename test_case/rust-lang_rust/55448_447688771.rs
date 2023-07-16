plain
travis_time:end:08a0fe58:start=1545000783692634956,finish=1545000838410597843,duration=54717962887
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:47] 
[00:51:47] running 121 tests
[00:51:50] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:51:50] i..ii.i.....iiii.....
[00:51:50] 
[00:51:50]  finished in 3.364
[00:51:50] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:05] 
[00:52:05] running 119 tests
[00:52:27] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:52:31] i......iii.i.....ii
[00:52:31] 
[00:52:31]  finished in 26.212
[00:52:31] travis_fold:end:test_debuginfo

---
[01:06:53] .................................................................................................... 1700/2213
[01:07:04] .................................................................................................... 1800/2213
[01:07:16] .................................................................................................... 1900/2213
[01:07:28] .................................................................................................... 2000/2213
[01:07:43] ........................F........................................................................... 2100/2213
[01:07:58] .............
[01:07:58] failures:
[01:07:58] 
[01:07:58] ---- slice/mod.rs - slice::[T]::partition_at_index_by_key (line 1715) stdout ----
[01:07:58] ---- slice/mod.rs - slice::[T]::partition_at_index_by_key (line 1715) stdout ----
[01:07:58] error[E0658]: use of unstable library feature 'partition_at_index' (see issue #55300)
[01:07:58]  --> slice/mod.rs:1721:3
[01:07:58]   |
[01:07:58] 9 | v.partition_at_index_by_key(2, |a| a.abs());
[01:07:58]   |
[01:07:58]   |
[01:07:58]   = help: add #![feature(partition_at_index)] to the crate attributes to enable
[01:07:58] 
[01:07:58] thread 'slice/mod.rs - slice::[T]::partition_at_index_by_key (line 1715)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:07:58] 
[01:07:58] 
[01:07:58] failures:
[01:07:58]     slice/mod.rs - slice::[T]::partition_at_index_by_key (line 1715)
[01:07:58]     slice/mod.rs - slice::[T]::partition_at_index_by_key (line 1715)
[01:07:58] 
[01:07:58] test result: FAILED. 2209 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
[01:07:58] 
[01:07:58] error: test failed, to rerun pass '--doc'
[01:07:58] 
[01:07:58] 
[01:07:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:07:58] 
[01:07:58] 
[01:07:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:58] Build completed unsuccessfully 
