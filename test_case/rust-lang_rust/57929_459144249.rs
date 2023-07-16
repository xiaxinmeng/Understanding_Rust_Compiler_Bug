plain
travis_time:end:1dff12b8:start=1548888795561254379,finish=1548888878808506257,duration=83247251878
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:45]    Compiling serde_derive v1.0.81
[00:01:46]    Compiling toml v0.4.10
[00:01:46]    Compiling serde_json v1.0.33
[00:01:54]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:01:57] error[E0599]: no method named `qrg` found for type `&mut std::process::Command` in the current scope
[00:01:57]     |
[00:01:57]     |
[00:01:57] 590 |              .qrg("--")
[00:01:57]     |
[00:01:57]     = help: did you mean `arg`?
[00:01:57] 
[00:01:58] error: aborting due to previous error
---
[00:01:58] make: *** [prepare] Error 1
[00:01:58] Makefile:70: recipe for target 'prepare' failed
[00:01:59] Command failed. Attempt 2/5:
[00:01:59]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:03] error[E0599]: no method named `qrg` found for type `&mut std::process::Command` in the current scope
[00:02:03]     |
[00:02:03]     |
[00:02:03] 590 |              .qrg("--")
[00:02:03]     |
[00:02:03]     = help: did you mean `arg`?
[00:02:03] 
[00:02:03] error: aborting due to previous error
---
[00:02:03] make: *** [prepare] Error 1
[00:02:03] Makefile:70: recipe for target 'prepare' failed
[00:02:05] Command failed. Attempt 3/5:
[00:02:05]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:09] error[E0599]: no method named `qrg` found for type `&mut std::process::Command` in the current scope
[00:02:09]     |
[00:02:09]     |
[00:02:09] 590 |              .qrg("--")
[00:02:09]     |
[00:02:09]     = help: did you mean `arg`?
[00:02:09] 
[00:02:09] error: aborting due to previous error
---
[00:02:09] Makefile:70: recipe for target 'prepare' failed
[00:02:09] make: *** [prepare] Error 1
[00:02:12] Command failed. Attempt 4/5:
[00:02:13]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:16] error[E0599]: no method named `qrg` found for type `&mut std::process::Command` in the current scope
[00:02:16]     |
[00:02:16]     |
[00:02:16] 590 |              .qrg("--")
[00:02:16]     |
[00:02:16]     = help: did you mean `arg`?
[00:02:16] 
[00:02:17] error: aborting due to previous error
---
[00:02:17] make: *** [prepare] Error 1
[00:02:17] Makefile:70: recipe for target 'prepare' failed
[00:02:21] Command failed. Attempt 5/5:
[00:02:21]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:24] error[E0599]: no method named `qrg` found for type `&mut std::process::Command` in the current scope
[00:02:24]     |
[00:02:24]     |
[00:02:24] 590 |              .qrg("--")
[00:02:24]     |
[00:02:24]     = help: did you mean `arg`?
[00:02:24] 
[00:02:25] error: aborting due to previous error
---
travis_time:end:2a18b2e2:start=1548889034154737382,finish=1548889034160053687,duration=5316305
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2163ad71
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25d6b605
travis_time:start:25d6b605
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1730613e
$ dmesg | grep -i kill
