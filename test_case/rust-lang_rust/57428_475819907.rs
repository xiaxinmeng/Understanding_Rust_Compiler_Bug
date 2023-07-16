plain
travis_time:end:0589abf1:start=1553298790159151759,finish=1553298793118542173,duration=2959390414
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:43]    Compiling synstructure v0.10.1
[00:06:54]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:11]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:08:12]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:30] error[E0599]: no method named `borrow` found for type `&syntax::feature_gate::Features` in the current scope
[00:08:30]    --> src/librustc/hir/lowering.rs:212:67
[00:08:30]     |
[00:08:30] 212 |         ImplTraitContext::Disallowed(if sess.features_untracked().borrow().associated_type_bounds {
[00:08:30]     |
[00:08:30]     = help: items from traits can only be used if the trait is in scope
[00:08:30]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:08:30]             `use std::borrow::Borrow;`
[00:08:30]             `use std::borrow::Borrow;`
[00:08:30] 
[00:08:30] error[E0599]: no variant named `OnlyAssocTy` found for type `hir::lowering::ImplTraitPosition` in the current scope
[00:08:30]    --> src/librustc/hir/lowering.rs:213:32
[00:08:30]     |
[00:08:30] 196 | enum ImplTraitPosition {
[00:08:30]     | ---------------------- variant `OnlyAssocTy` not found here
[00:08:30] ...
[00:08:30] 213 |             ImplTraitPosition::OnlyAssocTy
[00:08:30]     |             |
[00:08:30]     |             variant not found in `hir::lowering::ImplTraitPosition`
[00:08:30] 
[00:08:48] error: aborting due to 2 previous errors
---
travis_time:end:25224f17:start=1553299332106426123,finish=1553299332112019394,duration=5593271
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e7c2b0e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0257f44a
travis_time:start:0257f44a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11de7306
$ dmesg | grep -i kill
