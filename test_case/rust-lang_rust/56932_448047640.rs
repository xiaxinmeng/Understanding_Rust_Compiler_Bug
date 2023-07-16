plain
travis_time:end:03764f1f:start=1545091697349334428,finish=1545091698408246882,duration=1058912454
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
    0% |▎                               | 10kB 33.0MB/s eta 0:00:01
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.1MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:03:00]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:03] error[E0432]: unresolved import `super::super::flatten_compat`
[00:03:03]   --> src/libcore/iter/traits/iterator.rs:16:38
[00:03:03]    |
[00:03:03] 16 | use super::super::{Flatten, FlatMap, flatten_compat};
[00:03:03]    |                                      ^^^^^^^^^^^^^^ no `flatten_compat` in `iter`
[00:03:03] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:03]     --> src/libcore/iter/adapters/mod.rs:1734:21
[00:03:03]      |
[00:03:03]      |
[00:03:03] 1734 |                     LoopState::from_try(fold(acc, x))
[00:03:03]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:03] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:03]     --> src/libcore/iter/adapters/mod.rs:1737:21
[00:03:03]      |
[00:03:03]      |
[00:03:03] 1737 |                     LoopState::Break(Try::from_ok(acc))
[00:03:03]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:03] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:03]     --> src/libcore/iter/adapters/mod.rs:1876:29
[00:03:03]      |
[00:03:03]      |
[00:03:03] 1876 |                 if n == 0 { LoopState::Break(r) }
[00:03:03]      |                             ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:03] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:03]     --> src/libcore/iter/adapters/mod.rs:1877:24
[00:03:03]      |
[00:03:03]      |
[00:03:03] 1877 |                 else { LoopState::from_try(r) }
[00:03:03]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:03] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:03]     --> src/libcore/iter/adapters/mod.rs:1958:30
[00:03:03]      |
[00:03:03]      |
[00:03:03] 1958 |                 if *n == 0 { LoopState::Break(r) }
[00:03:03]      |                              ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:03] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:03]     --> src/libcore/iter/adapters/mod.rs:1959:24
[00:03:03]      |
[00:03:03]      |
[00:03:03] 1959 |                 else { LoopState::from_try(r) }
[00:03:03]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:03] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:03]     --> src/libcore/iter/adapters/mod.rs:2026:25
[00:03:03]      |
[00:03:03]      |
[00:03:03] 2026 |                 None => LoopState::Break(Try::from_ok(acc)),
[00:03:03]      |                         ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:03] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:03]     --> src/libcore/iter/adapters/mod.rs:2027:28
[00:03:03]      |
[00:03:03]      |
[00:03:03] 2027 |                 Some(x) => LoopState::from_try(fold(acc, x)),
[00:03:03]      |                            ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:05]    Compiling compiler_builtins v0.1.2
[00:03:05]    Compiling cmake v0.1.33
[00:03:05]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:08]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
---
[00:03:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:17] expected success, got: exit code: 101
[00:03:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:17] Build completed unsuccessfully in 0:00:20
[00:03:17] make: *** [all] Error 1
[00:03:17] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cdaf2da
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 18 00:11:44 UTC 2018
---
travis_time:end:02226aac:start=1545091904967708418,finish=1545091904972772687,duration=5064269
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07db3074
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:136be67d
travis_time:start:136be67d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a766824
$ dmesg | grep -i kill
