plain
travis_time:end:07b279a8:start=1561320736996039083,finish=1561320825944139186,duration=88948100103
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:07]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:07] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:04:07]    --> src/libcore/macros.rs:833:17
[00:04:07]     |
[00:04:07] 827 | /     macro_rules! doit {
[00:04:07] 828 | |         ($($name:ident)*) => (
[00:04:07] 830 | |                 /// Doc
[00:04:07] ...   |
[00:04:07] 833 | |                 #[allow_internal_unstable]
[00:04:07]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
---
[00:26:12]    Compiling autocfg v0.1.4
[00:26:13] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:13]    --> src/libcore/macros.rs:833:17
[00:26:13]     |
[00:26:13] 827 | /     macro_rules! doit {
[00:26:13] 828 | |         ($($name:ident)*) => (
[00:26:13] 830 | |                 /// Doc
[00:26:13] ...   |
[00:26:13] 833 | |                 #[allow_internal_unstable]
[00:26:13]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
---
[00:29:43]     |
[00:29:43] 833 |                 #[allow_internal_unstable]
[00:29:43]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:29:43] 
[00:29:43] error[E0433]: failed to resolve: could not find `rt` in `$crate`
[00:29:43]  --> src/libsyntax_pos/edition.rs:7:75
[00:29:43]   |
[00:29:43] 7 | #[derive(Clone, Copy, Hash, PartialEq, PartialOrd, Debug, RustcEncodable, RustcDecodable, Eq)]
[00:29:43]   |                                                                           ^^^^^^^^^^^^^^ could not find `rt` in `$crate`
[00:29:43] 
[00:29:43] error[E0433]: failed to resolve: could not find `rt` in `$crate`
[00:29:43]    |
[00:29:43]    |
[00:29:43] 67 | #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Debug, RustcEncodable, RustcDecodable)]
[00:29:43]    |                                                                               ^^^^^^^^^^^^^^ could not find `rt` in `$crate`
[00:29:43] 
[00:29:43] error[E0433]: failed to resolve: could not find `rt` in `$crate`
[00:29:43]     |
[00:29:43]     |
[00:29:43] 701 | #[derive(Clone, Hash, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
[00:29:43]     |                                                             ^^^^^^^^^^^^^^ could not find `rt` in `$crate`
[00:29:43] 
[00:29:43] error[E0433]: failed to resolve: could not find `rt` in `$crate`
[00:29:43]     |
[00:29:43]     |
[00:29:43] 721 | #[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
[00:29:43]     |                                                                   ^^^^^^^^^^^^^^ could not find `rt` in `$crate`
[00:29:43] 
[00:29:43] error[E0433]: failed to resolve: could not find `rt` in `$crate`
[00:29:43]    |
[00:29:43]    |
[00:29:43] 73 | #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, RustcDecodable, RustcEncodable)]
[00:29:43]    |                                                              ^^^^^^^^^^^^^^ could not find `rt` in `$crate`
[00:29:43] 
[00:29:43] error[E0433]: failed to resolve: could not find `rt` in `$crate`
[00:29:43]     |
[00:29:43]     |
[00:29:43] 770 | #[derive(Copy, Clone, RustcEncodable, RustcDecodable, Eq, PartialEq, Debug)]
[00:29:43]     |                                       ^^^^^^^^^^^^^^ could not find `rt` in `$crate`
[00:29:44] error: aborting due to 6 previous errors
[00:29:44] 
[00:29:44] For more information about this error, try `rustc --explain E0433`.
[00:29:44] error: Could not compile `syntax_pos`.
---
travis_time:end:000c9a80:start=1561322620619978827,finish=1561322620625207596,duration=5228769
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1905da34
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0085a494
travis_time:start:0085a494
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:014151d2
$ dmesg | grep -i kill
