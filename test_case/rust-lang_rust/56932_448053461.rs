plain
travis_time:end:0cc15788:start=1545093553521921823,finish=1545093554638029412,duration=1116107589
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/63/dc/c8bfd1bd77113c033161ce31730510d1c479cf9bcc8e99edf3c906f30cce/awscli-1.16.77-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 37.6MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.8MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:03:17]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:17]    Compiling libc v0.2.45
[00:03:17]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:18]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:21] error[E0432]: unresolved import `super::super::ZipImpl`
[00:03:21]    |
[00:03:21]    |
[00:03:21] 19 | use super::super::{FromIterator, ZipImpl};
[00:03:21]    |                                  ^^^^^^^ no `ZipImpl` in `iter`
[00:03:21] error[E0432]: unresolved import `super::traits`
[00:03:21]   --> src/libcore/iter/adapters/chain.rs:17:12
[00:03:21]    |
[00:03:21]    |
[00:03:21] 17 | use super::traits::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};
[00:03:21]    |            ^^^^^^ could not find `traits` in `super`
[00:03:21] 
[00:03:21] error[E0365]: `ZipImpl` is private, and cannot be re-exported
[00:03:21]   --> src/libcore/iter/adapters/mod.rs:24:26
[00:03:21]    |
[00:03:21] 24 | pub use self::zip::{Zip, ZipImpl};
[00:03:21]    |                          ^^^^^^^ re-export of private `ZipImpl`
[00:03:21]    |
[00:03:21]    = note: consider declaring type or module `ZipImpl` with `pub`
[00:03:21] error[E0432]: unresolved import `super::traits`
[00:03:21]   --> src/libcore/iter/adapters/zip.rs:13:12
[00:03:21]    |
[00:03:21]    |
[00:03:21] 13 | use super::traits::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};
[00:03:21]    |            ^^^^^^ could not find `traits` in `super`
[00:03:23]    Compiling compiler_builtins v0.1.2
[00:03:23]    Compiling cmake v0.1.33
[00:03:23]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:23]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:23] warning: unused import: `ZipImpl`
[00:03:23]    |
[00:03:23]    |
[00:03:23] 19 | use super::super::{FromIterator, ZipImpl};
[00:03:23]    |
[00:03:23]    = note: #[warn(unused_imports)] on by default
[00:03:23] 
[00:03:23] warning: unused import: `cmp`
---
[00:03:23] 
[00:03:23] warning: unused import: `ExactSizeIterator`
[00:03:23]   --> src/libcore/iter/adapters/chain.rs:17:52
[00:03:23]    |
[00:03:23] 17 | use super::traits::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};
[00:03:23] 
[00:03:23] warning: unused import: `super::LoopState`
[00:03:23]   --> src/libcore/iter/adapters/chain.rs:18:5
[00:03:23]    |
---
10820 ./src/tools/lldb/unittests
10508 ./src/llvm/test/MC/AMDGPU
10332 ./src/tools/clang/include
10188 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build
10140 ./src/tools/lldb/packages/Python/lldbsuite/test/functionalities/postmortem
10012 ./src/llvm-emscripten/test/MC/AMDGPU
9904 ./.git/modules/src/doc
travis_time:end:037e1333:start=1545093769871035329,finish=1545093770236838396,duration=365803067
travis_fold:end:after_failure.1
---
travis_time:end:107522f4:start=1545093770252379577,finish=1545093770258477212,duration=6097635
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e17e2a8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07670166
travis_time:start:07670166
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0827de1a
$ dmesg | grep -i kill
