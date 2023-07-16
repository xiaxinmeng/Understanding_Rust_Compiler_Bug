plain
travis_time:end:0751d4fa:start=1545092174472777421,finish=1545092175697933545,duration=1225156124
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
    0% |▎                               | 10kB 30.4MB/s eta 0:00:01
    1% |▌                               | 20kB 2.0MB/s eta 0:00:01
    2% |▊                               | 30kB 3.0MB/s eta 0:00:01
    2% |█                               | 40kB 2.1MB/s eta 0:00:01
---
[00:03:04]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:08] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:08]     --> src/libcore/iter/adapters/mod.rs:1734:21
[00:03:08]      |
[00:03:08] 1734 |                     LoopState::from_try(fold(acc, x))
[00:03:08]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:08] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:08]     --> src/libcore/iter/adapters/mod.rs:1737:21
[00:03:08]      |
[00:03:08]      |
[00:03:08] 1737 |                     LoopState::Break(Try::from_ok(acc))
[00:03:08]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:08] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:08]     --> src/libcore/iter/adapters/mod.rs:1876:29
[00:03:08]      |
[00:03:08]      |
[00:03:08] 1876 |                 if n == 0 { LoopState::Break(r) }
[00:03:08]      |                             ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:08] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:08]     --> src/libcore/iter/adapters/mod.rs:1877:24
[00:03:08]      |
[00:03:08]      |
[00:03:08] 1877 |                 else { LoopState::from_try(r) }
[00:03:08]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:08] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:08]     --> src/libcore/iter/adapters/mod.rs:1958:30
[00:03:08]      |
[00:03:08]      |
[00:03:08] 1958 |                 if *n == 0 { LoopState::Break(r) }
[00:03:08]      |                              ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:08] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:08]     --> src/libcore/iter/adapters/mod.rs:1959:24
[00:03:08]      |
[00:03:08]      |
[00:03:08] 1959 |                 else { LoopState::from_try(r) }
[00:03:08]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:08] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:08]     --> src/libcore/iter/adapters/mod.rs:2026:25
[00:03:08]      |
[00:03:08]      |
[00:03:08] 2026 |                 None => LoopState::Break(Try::from_ok(acc)),
[00:03:08]      |                         ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:08] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:08]     --> src/libcore/iter/adapters/mod.rs:2027:28
[00:03:08]      |
[00:03:08]      |
[00:03:08] 2027 |                 Some(x) => LoopState::from_try(fold(acc, x)),
[00:03:08]      |                            ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:09]    Compiling compiler_builtins v0.1.2
[00:03:09]    Compiling cmake v0.1.33
[00:03:09]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:13]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
