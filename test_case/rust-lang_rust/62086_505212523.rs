plain
travis_time:end:24216d0e:start=1561416256534419360,finish=1561416258984136813,duration=2449717453
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    98% |███████████████████████████████▋| 1.7MB 59.6MB/s eta 0:00:01
    99% |███████████████████████████████▉| 1.7MB 60.7MB/s eta 0:00:01
    100% |████████████████████████████████| 1.7MB 10.9MB/s 
Collecting botocore==1.12.175 (from awscli)
  Downloading https://files.pythonhosted.org/packages/19/ff/fff69109c7f4f97f393b0b948eab16caf3464204fe5cf1955d9d1e1879fa/botocore-1.12.175-py2.py3-none-any.whl (5.6MB)
    0% |▏                               | 20kB 31.3MB/s eta 0:00:01
    0% |▏                               | 30kB 39.8MB/s eta 0:00:01
    0% |▎                               | 40kB 42.7MB/s eta 0:00:01
    0% |▎                               | 51kB 44.6MB/s eta 0:00:01
---
travis_time:start:tidy
tidy check
[00:04:54] * 576 error codes
[00:04:54] * highest error code: E0731
[00:04:54] tidy error: /checkout/src/libcore/macros.rs:747: malformed stability attribute: missing `feature` key
[00:04:54] tidy error: /checkout/src/libcore/macros.rs:846: malformed stability attribute: missing `feature` key
[00:04:54] tidy error: /checkout/src/libcore/macros.rs:929: malformed stability attribute: missing `feature` key
[00:04:54] tidy error: /checkout/src/libcore/macros.rs:1199: malformed stability attribute: missing `feature` key
[00:04:54] tidy error: /checkout/src/libcore/macros.rs:1213: malformed stability attribute: missing `feature` key
[00:04:54] tidy error: /checkout/src/libcore/macros.rs:1223: malformed stability attribute: missing `feature` key
[00:04:54] tidy error: /checkout/src/libcore/macros.rs:1233: malformed stability attribute: missing `feature` key
[00:04:54] tidy error: /checkout/src/libcore/macros.rs:1258: malformed stability attribute: missing `feature` key
[00:04:54] tidy error: /checkout/src/libcore/macros.rs contains #[test]; libcore tests must be placed inside `src/libcore/tests/`
[00:04:55] some tidy checks failed
[00:04:55] 
[00:04:55] 
[00:04:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:04:55] 
[00:04:55] 
[00:04:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:55] Build completed unsuccessfully in 0:01:10
---
travis_time:end:1cdb6d6a:start=1561416565471653394,finish=1561416565476187693,duration=4534299
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:31275c00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10381be2
travis_time:start:10381be2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ad85050
$ dmesg | grep -i kill
