plain
travis_time:end:1cb04f98:start=1543548113230384832,finish=1543548115587252914,duration=2356868082
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/aa/e5/ebd5896ad5ae353d23bea05ebb8edd3d49f1471784f6afa12a9cf11710de/awscli-1.16.67-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 11.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.3MB/s eta 0:00:02
    2% |▊                               | 30kB 1.7MB/s eta 0:00:01
    2% |█                               | 40kB 1.6MB/s eta 0:00:01
---
[00:04:57]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:02]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:25]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:06:35]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:40] error: unused imports: `GateIssue`, `emit_feature_err`
[00:06:40]   --> src/librustc/hir/lowering.rs:70:28
[00:06:40]    |
[00:06:40] 70 | use syntax::feature_gate::{emit_feature_err, GateIssue};
[00:06:40]    |
[00:06:40]    = note: `-D unused-imports` implied by `-D warnings`
[00:06:40] 
travis_time:end:023ee024:start=1543548552027569339,finish=1543548552033843704,duration=6274365
---
travis_time:end:281cd3bd:start=1543548552039093522,finish=1543548552044566912,duration=5473390
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0be58efc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05ae0524
travis_time:start:05ae0524
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
