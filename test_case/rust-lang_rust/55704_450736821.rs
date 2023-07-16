plain
travis_time:end:0f236e00:start=1546350294455048962,finish=1546350349272760816,duration=54817711854
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
[01:05:44] 
[01:05:44] running 119 tests
[01:06:07] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:06:10] i......iii.i.....ii
[01:06:10] 
[01:06:10]  finished in 26.996
[01:06:10] travis_fold:end:test_debuginfo

---
[01:20:37] .................................................................................................... 1500/2226
[01:20:48] .................................................................................................... 1600/2226
[01:20:59] .................................................................................................... 1700/2226
[01:21:10] .................................................................................................... 1800/2226
[01:21:22] .............................F...................................................................... 1900/2226
[01:21:47] .................................................................................................... 2100/2226
[01:22:01] .........................................i.......................................................i.. 2200/2226
[01:22:04] ..........................
[01:22:04] failures:
[01:22:04] failures:
[01:22:04] 
[01:22:04] ---- ops/generator.rs - ops::generator::Generator (line 41) stdout ----
[01:22:04] error: the feature `pin` has been stable since 1.33.0 and no longer requires an attribute to enable
[01:22:04]  --> ops/generator.rs:42:41
[01:22:04]   |
[01:22:04] 3 | #![feature(generators, generator_trait, pin)]
[01:22:04]   |
[01:22:04] note: lint level defined here
[01:22:04]  --> ops/generator.rs:40:9
[01:22:04]   |
---
[01:22:04] 
[01:22:04] error: test failed, to rerun pass '--doc'
[01:22:04] 
[01:22:04] 
[01:22:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:22:04] 
[01:22:04] 
[01:22:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:04] Build completed unsuccessfully in 0:27:08
[01:22:04] Build completed unsuccessfully in 0:27:08
[01:22:04] Makefile:48: recipe for target 'check' failed
[01:22:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27e13a12
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan  1 15:08:02 UTC 2019
