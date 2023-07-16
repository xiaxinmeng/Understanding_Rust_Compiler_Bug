plain
travis_time:end:0335a315:start=1560875161431680230,finish=1560875253292074969,duration=91860394739
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:29]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:14:31] error[E0425]: cannot find function `make_target_blocks` in this scope
[00:14:31]    --> src/librustc_mir/build/matches/test.rs:182:37
[00:14:31]     |
[00:14:31] 182 |                 let target_blocks = make_target_blocks(self);
[00:14:31]     |                                     ^^^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `target_blocks`
[00:14:31] error[E0425]: cannot find function `make_target_blocks` in this scope
[00:14:31]    --> src/librustc_mir/build/matches/test.rs:283:37
[00:14:31]     |
[00:14:31]     |
[00:14:31] 283 |                 let target_blocks = make_target_blocks(self);
[00:14:31]     |                                     ^^^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `target_blocks`
[00:14:44] error: aborting due to 2 previous errors
[00:14:44] 
[00:14:44] For more information about this error, try `rustc --explain E0425`.
[00:14:44] error: Could not compile `rustc_mir`.
---
travis_time:end:2287ea92:start=1560876308080639675,finish=1560876308085644017,duration=5004342
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2ab71b3e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0433086f
travis_time:start:0433086f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0712dce8
$ dmesg | grep -i kill
