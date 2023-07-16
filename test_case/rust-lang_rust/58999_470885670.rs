plain
travis_time:end:1bbcfb41:start=1552041190285209745,finish=1552041311422702831,duration=121137493086
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:16]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:30]    Compiling synstructure v0.10.1
[00:06:49]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:33]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:33] error: expected `{`, found `;`
[00:07:33]     --> src/librustc/hir/lowering.rs:4586:22
[00:07:33]      |
[00:07:33] 4578 |                     let from_expr = if if self.catch_scopes.is_empty() && self.returns_impl {
[00:07:33]      |                                     -- this `if` statement has a condition, but no block
[00:07:33] 4586 |                     };
[00:07:33]      |                      ^ expected `{`
[00:07:33] 
[00:08:06] error: aborting due to previous error
---
travis_time:end:2bc915c0:start=1552041807706312816,finish=1552041807711759702,duration=5446886
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:066c9e64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07479089
travis_time:start:07479089
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:069c1c17
$ dmesg | grep -i kill
