plain
travis_time:end:01504910:start=1554252951726268766,finish=1554252952603197523,duration=876928757
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:07:07]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:07] error[E0583]: file not found for module `symbols`
[00:07:07]   --> src/librustc_macros/src/lib.rs:12:5
[00:07:07]    |
[00:07:07] 12 | mod symbols;
[00:07:07]    |
[00:07:07]    |
[00:07:07]    = help: name the file either symbols.rs or symbols/mod.rs inside the directory "src/librustc_macros/src"
[00:07:07] error: aborting due to previous error
[00:07:07] 
[00:07:07] For more information about this error, try `rustc --explain E0583`.
[00:07:07] error: Could not compile `rustc_macros`.
---
travis_time:end:017f782b:start=1554253391329039133,finish=1554253391337608513,duration=8569380
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06d40b80
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00116596
travis_time:start:00116596
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/
