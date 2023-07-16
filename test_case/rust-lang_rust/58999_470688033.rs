plain
travis_time:end:0c874b30:start=1551991148975203239,finish=1551991149955647636,duration=980444397
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:27]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:28]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:33]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:55]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:55] error: expected `{`, found `;`
[00:07:55]     --> src/librustc/hir/lowering.rs:4586:22
[00:07:55]      |
[00:07:55] 4578 |                     let from_expr = if if self.catch_scopes.is_empty() && self.returns_impl {
[00:07:55]      |                                     -- this `if` statement has a condition, but no block
[00:07:55] 4586 |                     };
[00:07:55]      |                      ^ expected `{`
[00:07:55] 
[00:08:36] error: aborting due to previous error
---
travis_time:end:121475c7:start=1551991680721115275,finish=1551991680727163288,duration=6048013
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0abe724c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01a01bc0
travis_time:start:01a01bc0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05a43272
$ dmesg | grep -i kill
