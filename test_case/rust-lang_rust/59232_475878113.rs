plain
travis_time:end:2f7af79f:start=1553352610681796655,finish=1553352685026031560,duration=74344234905
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:28:04]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:28:04]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:28:05]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:28:05]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:28:24] error: internal compiler error: src/librustc_mir/borrow_check/nll/mod.rs:354: region is not an ReVar: ReStatic
[00:28:24] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[00:28:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:28:24] error: aborting due to previous error
[00:28:24] 
[00:28:24] 
[00:28:24] 
[00:28:24] note: the compiler unexpectedly panicked. this is a bug.
[00:28:24] 
[00:28:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:28:24] 
[00:28:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:28:24] 
[00:28:24] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:28:24] note: some of the compiler flags provided by cargo are hidden
[00:28:24] 
[00:28:25] error: Could not compile `core`.
[00:28:25] 
---
travis_time:end:274e8e4a:start=1553354400212355337,finish=1553354400219776250,duration=7420913
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:106db90c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b51f4bc
travis_time:start:0b51f4bc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6

