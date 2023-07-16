plain
travis_time:end:0bfba5bc:start=1561419821803874912,finish=1561419822618143134,duration=814268222
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    98% |███████████████████████████████▋| 1.7MB 43.0MB/s eta 0:00:01
    99% |███████████████████████████████▉| 1.7MB 48.7MB/s eta 0:00:01
    100% |████████████████████████████████| 1.7MB 9.6MB/s 
Collecting botocore==1.12.175 (from awscli)
  Downloading https://files.pythonhosted.org/packages/19/ff/fff69109c7f4f97f393b0b948eab16caf3464204fe5cf1955d9d1e1879fa/botocore-1.12.175-py2.py3-none-any.whl (5.6MB)
    0% |▏                               | 20kB 22.4MB/s eta 0:00:01
    0% |▏                               | 30kB 27.3MB/s eta 0:00:01
    0% |▎                               | 40kB 30.7MB/s eta 0:00:01
    0% |▎                               | 51kB 31.4MB/s eta 0:00:01
---

[00:04:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:51] tidy error: /checkout/src/librustc/ty/query/plumbing.rs:963: trailing whitespace
[00:04:53] some tidy checks failed
[00:04:53] 
[00:04:53] 
[00:04:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:04:53] 
[00:04:53] 
[00:04:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:53] Build completed unsuccessfully in 0:01:15
---
travis_time:end:00beb086:start=1561420128053136347,finish=1561420128058452516,duration=5316169
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:056c427f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:095ef340
travis_time:start:095ef340
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fe9575e
$ dmesg | grep -i kill
