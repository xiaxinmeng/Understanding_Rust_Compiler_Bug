plain
travis_time:end:183ceb62:start=1545275902307015717,finish=1545275962779502606,duration=60472486889
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    100% |████████████████████████████████| 51kB 9.2MB/s 
Collecting botocore==1.12.69 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/72/ba/a188505f67a78a686aa24d8511a18cb5a8bb27705c9d1b1bb81bee97a138/botocore-1.12.69-py2.py3-none-any.whl (5.2MB)
    0% |                                | 10kB 36.8MB/s eta 0:00:01
    0% |▏                               | 20kB 34.1MB/s eta 0:00:01
    0% |▏                               | 30kB 33.4MB/s eta 0:00:01
    0% |▎                               | 40kB 35.0MB/s eta 0:00:01
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:53] 
[01:05:53] running 119 tests
[01:06:17] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[01:06:21] i......iii.i.....ii
[01:06:21] 
[01:06:21]  finished in 27.274
[01:06:21] travis_fold:end:test_debuginfo

---
[01:18:13] 
[01:18:13] running 916 tests
[01:18:13] .................................................................................................... 100/916
[01:18:13] .................................................................................................... 200/916
[01:18:13] ...............F.................................................................................... 300/916
[01:18:13] .................................................................................................... 500/916
travis_time:end:14053035:start=1545275970834783913,finish=1545280665435716516,duration=4694600932603
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:236df6bc
---
travis_time:end:13f4cb2c:start=1545280668073976106,finish=1545280668081186770,duration=7210664
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03ec3adf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02ee30f6
travis_time:start:02ee30f6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:064abdf1
$ dmesg | grep -i kill
