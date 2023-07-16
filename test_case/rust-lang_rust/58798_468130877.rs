plain
travis_time:end:0042f473:start=1551321847179267401,finish=1551321922843095456,duration=75663828055
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:19]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:58:19] 
[00:58:20]     Finished release [optimized] target(s) in 35.64s
[00:58:21]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:58:38] warning: `[mem::zeroed]` cannot be resolved, ignoring it...
[00:58:38]     |
[00:58:38]     |
[00:58:38] 911 | /// a `ManuallyDrop<&mut T>` with [`mem::zeroed`] is undefined behavior.
[00:58:38]     |
[00:58:38] note: lint level defined here
[00:58:38]    --> src/libcore/lib.rs:63:9
[00:58:38]     |
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:54] 
[01:13:54] running 120 tests
[01:14:19] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:14:23] .i......iii.i.....ii
[01:14:23] 
[01:14:23]  finished in 29.160
[01:14:23] travis_fold:end:test_debuginfo

---
[01:24:40] 
[01:24:40]    Doc-tests core
[01:24:45] 
[01:24:45] running 2277 tests
[01:24:56] ....iiiii........................................................................................... 100/2277
[01:25:08] .....................................................................ii.....F....................... 200/2277
[01:25:36] .................................................................................................... 400/2277
[01:25:47] .....................i..i........................................................................... 500/2277
[01:25:59] .................................................................................................... 600/2277
[01:26:10] .................................................................................................... 700/2277
---
[01:29:20] 
[01:29:20] error: test failed, to rerun pass '--doc'
[01:29:20] 
[01:29:20] 
[01:29:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:29:20] 
[01:29:20] 
[01:29:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:20] Build completed unsuccessfully in 0:27:01
[01:29:20] Build completed unsuccessfully in 0:27:01
[01:29:20] Makefile:48: recipe for target 'check' failed
[01:29:20] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:266ef21c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 28 04:14:51 UTC 2019
