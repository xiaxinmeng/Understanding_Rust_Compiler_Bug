plain
travis_time:end:05d2d6ae:start=1558320515393783471,finish=1558320602561461127,duration=87167677656
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:55:24]    Compiling parking_lot_core v0.4.0
[00:55:30]    Compiling tempfile v3.0.5
[00:55:32]    Compiling parking_lot v0.7.1
[00:55:35]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:55:35] error[E0583]: file not found for module `docfs`
[00:55:35]    |
[00:55:35]    |
[00:55:35] 61 | mod docfs;
[00:55:35]    |
[00:55:35]    |
[00:55:35]    = help: name the file either docfs.rs or docfs/mod.rs inside the directory "src/librustdoc"
[00:55:35] error: aborting due to previous error
[00:55:35] 
[00:55:35] For more information about this error, try `rustc --explain E0583`.
[00:55:35] error: Could not compile `rustdoc`.
---
travis_time:end:01a3b037:start=1558323947853311777,finish=1558323947857976465,duration=4664688
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c2fbb65
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20b656f8
travis_time:start:20b656f8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00895d93
$ dmesg | grep -i kill
