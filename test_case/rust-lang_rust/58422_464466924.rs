plain
travis_time:end:04940500:start=1550414973194700673,finish=1550415055424368629,duration=82229667956
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:10]    Compiling parking_lot v0.6.4
[00:05:11]    Compiling rustc-rayon v0.1.1
[00:05:15]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:05:15]    Compiling tempfile v3.0.5
[00:05:15] error[E0658]: use of unstable library feature 'seek_convenience'
[00:05:15]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tempfile-3.0.5/src/spooled.rs:90:50
[00:05:15]    |
[00:05:15] 90 |                 file.seek(SeekFrom::Start(cursor.position()))?;
[00:05:15]    |
[00:05:15]    |
[00:05:15]    = help: add #![feature(seek_convenience)] to the crate attributes to enable
[00:05:15] 
[00:05:15] error: internal compiler error: src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:05:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:588:9
[00:05:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:05:15] error: aborting due to 2 previous errors
[00:05:15] 
---
[00:05:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:05:15] 
[00:05:15] note: rustc 1.33.0-beta.1 (d1add9723 2019-01-17) running on x86_64-unknown-linux-gnu
[00:05:15] 
[00:05:15] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:05:15] note: some of the compiler flags provided by cargo are hidden
[00:05:15] 
[00:05:15] error: Could not compile `tempfile`.
[00:05:15] warning: build failed, waiting for other jobs to finish...
[00:05:15] warning: build failed, waiting for other jobs to finish...
[00:05:17] error: build failed
[00:05:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:17] expected success, got: exit code: 101
[00:05:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:17] Build completed unsuccessfully in 0:02:01
[00:05:17] Makefile:18: recipe for target 'all' failed
[00:05:17] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bdd40b4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 14:56:22 UTC 2019
