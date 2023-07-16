plain
travis_time:end:0a1fbb34:start=1545276305527646940,finish=1545276389072489264,duration=83544842324
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    100% |████████████████████████████████| 51kB 10.6MB/s 
Collecting botocore==1.12.69 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/72/ba/a188505f67a78a686aa24d8511a18cb5a8bb27705c9d1b1bb81bee97a138/botocore-1.12.69-py2.py3-none-any.whl (5.2MB)
    0% |                                | 10kB 39.7MB/s eta 0:00:01
    0% |▏                               | 20kB 39.0MB/s eta 0:00:01
    0% |▏                               | 30kB 43.4MB/s eta 0:00:01
    0% |▎                               | 40kB 38.4MB/s eta 0:00:01
---
187088 ./obj/build/cache/2018-12-09
160388 ./obj/build/bootstrap/debug/incremental
153280 ./src/tools/clang
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7r67yu4ze-rpr10o-3g9jcdhvsk79m
111164 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107420 ./src/tools/lldb
95108 ./src/tools/clang/test
89968 ./src/llvm-emscripten/test/CodeGen
---
travis_time:end:23d82a0a:start=1545276692104189928,finish=1545276692109010868,duration=4820940
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:066e9370
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01349e5c
travis_time:start:01349e5c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
