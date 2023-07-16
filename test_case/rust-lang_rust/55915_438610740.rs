plain
travis_time:end:0f7be300:start=1542189622385761784,finish=1542189685128900722,duration=62743138938
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:13:54]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:56] error[E0433]: failed to resolve. Use of undeclared type or module `AllocationExtra`
[00:13:56]    --> librustc_mir/interpret/memory.rs:228:9
[00:13:56]     |
[00:13:56] 228 |         AllocationExtra::memory_deallocated(&mut alloc, ptr, size)?;
[00:13:56]     |         ^^^^^^^^^^^^^^^ Use of undeclared type or module `AllocationExtra`
[00:14:13] error: aborting due to previous error
[00:14:13] 
[00:14:13] For more information about this error, try `rustc --explain E0433`.
[00:14:13] error: Could not compile `rustc_mir`.
---
travis_time:end:0e69423a:start=1542190598647787771,finish=1542190598654310687,duration=6522916
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2e6f8bd6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a1fdb82
travis_time:start:0a1fdb82
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2bd3aa0c
$ dmesg | grep -i kill
