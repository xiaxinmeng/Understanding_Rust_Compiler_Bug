plain
travis_time:end:1a254572:start=1549403686974300723,finish=1549403687855527144,duration=881226421
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:50:48]    Compiling parking_lot_core v0.3.0
[00:50:50]    Compiling parking_lot v0.6.4
[00:50:52]    Compiling tempfile v3.0.5
[00:50:53]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:50:57] error[E0609]: no field `deprecation` on type `std::option::Option<clean::Stability>`
[00:50:57]     --> src/librustdoc/html/render.rs:2804:45
[00:50:57]      |
[00:50:57] 2804 |         if let Some(since) = item.stability.deprecation.since {
[00:50:57] 
[00:50:57] 
[00:50:57] error[E0609]: no field `deprecation` on type `std::option::Option<clean::Stability>`
[00:50:57]     --> src/librustdoc/html/render.rs:2843:63
[00:50:57]      |
[00:50:57] 2843 |         let mut message = if let Some(since) = item.stability.deprecation.since {
[00:50:57] 
[00:50:58] error: aborting due to 2 previous errors
[00:50:58] 
[00:50:58] For more information about this error, try `rustc --explain E0609`.
---
[00:50:58] 
[00:50:58] 
[00:50:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:50:58] Build completed unsuccessfully in 0:46:32
[00:50:58] make: *** [all] Error 1
[00:50:58] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:081c0dce
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 22:45:58 UTC 2019
---
travis_time:end:017c1bbc:start=1549406759051864515,finish=1549406759057132226,duration=5267711
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2e85b9e6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis
