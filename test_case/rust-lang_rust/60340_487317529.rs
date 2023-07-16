plain
travis_time:end:027a071e:start=1556396909133286972,finish=1556396909921163385,duration=787876413
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:23]    Compiling hashbrown v0.3.0
[00:04:30] error: method is never used: `cap`
[00:04:30]    --> src/libstd/sync/mpsc/sync.rs:479:5
[00:04:30]     |
[00:04:30] 479 |     fn cap(&self) -> usize { self.capacity() }  // For backwards compatibility
[00:04:30]     |
[00:04:30]     = note: `-D dead-code` implied by `-D warnings`
[00:04:30] 
[00:04:30] error: aborting due to previous error
---
travis_time:end:335d9f1b:start=1556397191938661426,finish=1556397191944353508,duration=5692082
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04fdf1c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a9bf30d
travis_time:start:1a9bf30d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07f090c2
$ dmesg | grep -i kill
