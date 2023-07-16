plain
travis_time:end:0b970f60:start=1555875848728989835,finish=1555875849464803489,duration=735813654
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:46]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:46] error: incorrect close delimiter: `]`
[00:04:46]    --> src/libstd/sys/unix/rand.rs:112:61
[00:04:46]     |
[00:04:46] 112 | #[cfg(any(target_os = "ios", all(target_os = "macos", miri))]
[00:04:46]     |  -   - un-closed delimiter                                  ^ incorrect close delimiter
[00:04:46]     |  close delimiter possibly meant for this
[00:04:46] 
n/test/tools
22636 ./src/llvm-project/llgo/third_party/gofrontend/libgo
---
travis_time:end:1fa169da:start=1555876151384374689,finish=1555876151388962475,duration=4587786
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:27d4c692
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ba10953
travis_time:start:0ba10953
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_fa
