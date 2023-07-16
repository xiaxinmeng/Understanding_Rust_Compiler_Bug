plain
travis_time:end:041e2d6e:start=1561240909618576078,finish=1561240998406062147,duration=88787486069
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:08]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:08:20]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:14:17]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:14:17]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:14:28] error[E0599]: no method named `len` found for type `rustc_data_structures::work_queue::WorkQueue<rustc::mir::BasicBlock>` in the current scope
[00:14:28]    |
[00:14:28]    |
[00:14:28] 98 |     debug_assert!(dirty_queue.len() == body.basic_blocks().len(),
[00:14:28] 
[00:14:31] error: aborting due to previous error
[00:14:31] 
[00:14:31] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:0762d8d6:start=1561241965208996959,finish=1561241965214962401,duration=5965442
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2cb593ac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f856e60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/li
