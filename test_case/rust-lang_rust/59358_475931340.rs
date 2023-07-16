plain
travis_time:end:0326c964:start=1553406409795269447,finish=1553406484147189549,duration=74351920102
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:13:33]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:13:49] error[E0308]: match arms have incompatible types
[00:13:49]    --> src/librustc_mir/const_eval.rs:656:29
[00:13:49]     |
[00:13:49] 652 |  /             match reported_err {
[00:13:49] 653 |  |                 Ok(v) => tcx.sess.delay_span_bug(err.span,
[00:13:49]     |  |__________________________-
[00:13:49] 654 | ||                                         &format!("static eval failure did not emit an error: {:#?}",
[00:13:49] 655 | ||                                         v)),
[00:13:49]     | ||___________________________________________- this is found to be of type `()`
[00:13:49] 656 |  |                 Err(err) => err,
[00:13:49]     |  |                             ^^^ expected (), found struct `rustc::util::common::ErrorReported`
[00:13:49] 657 |  |             }
[00:13:49]     |  |_____________- `match` arms have incompatible types
[00:13:49]     = note: expected type `()`
[00:13:49]                found type `rustc::util::common::ErrorReported`
[00:13:49] 
[00:13:50] error: aborting due to previous error
---
travis_time:end:018e2810:start=1553407546471130127,finish=1553407546476492478,duration=5362351
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15b45a97
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2a1c4b18
travis_time:start:2a1c4b18
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No s
