plain
travis_time:end:21ad94c6:start=1560157429923116981,finish=1560157518466111594,duration=88542994613
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:24]    Compiling synstructure v0.10.2
[00:06:37]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:44]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:48]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:48] error[E0658]: `crate` visibility modifier is experimental
[00:06:48]     |
[00:06:48]     |
[00:06:48] 351 |     crate fn new_with_code(handler: &'a Handler,
[00:06:48]     |
[00:06:48]     = note: for more information, see https://github.com/rust-lang/rust/issues/53120
[00:06:48]     = note: for more information, see https://github.com/rust-lang/rust/issues/53120
[00:06:48]     = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:06:48] error: aborting due to previous error
[00:06:48] 
[00:06:48] For more information about this error, try `rustc --explain E0658`.
[00:06:48] error: Could not compile `rustc_errors`.
---
travis_time:end:00415a31:start=1560157949288741887,finish=1560157949293517436,duration=4775549
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00034b17
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08bbe3f3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/a
