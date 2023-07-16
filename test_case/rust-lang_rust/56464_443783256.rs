plain
travis_time:end:070486b0:start=1543852283535146173,finish=1543852370569137316,duration=87033991143
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:55] 
[00:52:55] running 120 tests
[00:52:58] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii...............i..ii 100/120
[00:52:59] ..ii.i.....iiii.....
[00:52:59] 
[00:52:59]  finished in 3.260
[00:52:59] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:12] 
[00:53:12] running 118 tests
[00:53:35] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:53:38] ......iii.i.....ii
[00:53:38] 
[00:53:38]  finished in 26.194
[00:53:38] travis_fold:end:test_debuginfo

---
[01:01:15]    Compiling rand_core v0.2.1
[01:01:15]    Compiling libc v0.2.43
[01:01:16]    Compiling rand v0.5.5
[01:01:20]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:01:25] error: function is never used: `replace_helper`
[01:01:25]     |
[01:01:25]     |
[01:01:25] 609 | fn replace_helper<'a, P: Pattern<'a>>(s: &'a str, pat: P, to: &str, lim: Option<usize>) -> String {
[01:01:25]     |
[01:01:25]     = note: `-D dead-code` implied by `-D warnings`
[01:01:25] 
[01:01:25] error: aborting due to previous error
[01:01:25] error: aborting due to previous error
[01:01:25] 
[01:01:25] error: Could not compile `alloc`.
[01:01:25] warning: build failed, waiting for other jobs to finish...
[01:02:29] error: build failed
[01:02:29] 
[01:02:29] 
[01:02:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:02:29] 
[01:02:29] 
[01:02:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:29] Build completed unsuccessfully in 0:19:40
[01:02:29] Build completed unsuccessfully in 0:19:40
[01:02:29] make: *** [check] Error 1
[01:02:29] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00239916
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 16:55:30 UTC 2018
