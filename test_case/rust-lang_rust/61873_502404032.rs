plain
travis_time:end:138e0ab4:start=1560636922330806312,finish=1560636924688547365,duration=2357741053
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:20]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:07:22] error[E0308]: mismatched types
[00:07:22]    --> src/librustc_data_structures/sync.rs:229:35
[00:07:22]     |
[00:07:22] 229 |                 SharedWorkerLocal(OneThread::new(f(0)))
[00:07:22]     |                                   ^^^^^^^^^^^^^^^^^^^^ expected type parameter, found struct `sync::OneThread`
[00:07:22]     = note: expected type `T`
[00:07:22]     = note: expected type `T`
[00:07:22]                found type `sync::OneThread<T>`
[00:07:23] error: aborting due to previous error
[00:07:23] 
[00:07:23] For more information about this error, try `rustc --explain E0308`.
[00:07:23] error: Could not compile `rustc_data_structures`.
---
travis_time:end:0301eff5:start=1560637392091412071,finish=1560637392096760232,duration=5348161
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03a6275a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f9866ec
travis_time:start:2f9866ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:014422e0
$ dmesg | grep -i kill
