plain
travis_time:end:3c3b2fa6:start=1551125333789844912,finish=1551125439750742419,duration=105960897507
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
$ pip install --user awscli; export PATH=$PATH:$HOME/.local/bin:$HOME/Library/Python/2.7/bin/
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/60/ca/a1778f623313ac7c90a40fed3f2fd2cb7538d0ca71ba2240c9c3a4df90b2/awscli-1.16.112-py2.py3-none-any.whl (1.4MB)
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.1MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
    3% |█▏                              | 51kB 2.5MB/s eta 0:00:01
---

[00:04:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:12: trailing whitespace
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:13: trailing whitespace
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:14: trailing whitespace
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:69: TODO is deprecated; use FIXME
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:81: TODO is deprecated; use FIXME
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:116: line longer than 100 chars
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:242: line longer than 100 chars
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:285: TODO is deprecated; use FIXME
[00:04:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:333: TODO is deprecated; use FIXME
[00:04:03] some tidy checks failed
[00:04:03] 
[00:04:03] 
[00:04:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:03] 
[00:04:03] 
[00:04:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:03] Build completed unsuccessfully in 0:00:51
[00:04:03] Build completed unsuccessfully in 0:00:51
[00:04:03] Makefile:68: recipe for target 'tidy' failed
[00:04:03] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12e018eb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 25 20:14:54 UTC 2019
---
travis_time:end:12c00374:start=1551125695850010793,finish=1551125695856198959,duration=6188166
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:049b1dd9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01a61a02
travis_time:start:01a61a02
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d8264c8
$ dmesg | grep -i kill
