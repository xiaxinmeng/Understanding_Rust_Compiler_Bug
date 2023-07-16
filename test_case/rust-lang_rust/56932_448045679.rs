plain
travis_time:end:173572aa:start=1545091155148095993,finish=1545091156293016708,duration=1144920715
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
    0% |▎                               | 10kB 35.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.8MB/s eta 0:00:01
    2% |█                               | 40kB 1.9MB/s eta 0:00:01
---
[00:02:58]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:01] error[E0432]: unresolved import `super::super::flatten_compat`
[00:03:01]   --> src/libcore/iter/traits/iterator.rs:16:38
[00:03:01]    |
[00:03:01] 16 | use super::super::{Flatten, FlatMap, flatten_compat};
[00:03:01]    |                                      ^^^^^^^^^^^^^^ no `flatten_compat` in `iter`
[00:03:01] 
[00:03:01] error[E0432]: unresolved imports `super::super::ChainState`, `super::super::ZipImpl`
[00:03:01]    |
[00:03:01]    |
[00:03:01] 19 | use super::super::{ChainState, FromIterator, ZipImpl};
[00:03:01]    |                    ^^^^^^^^^^                ^^^^^^^ no `ZipImpl` in `iter`
[00:03:01]    |                    no `ChainState` in `iter`
[00:03:01] 
[00:03:02] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:02]     --> src/libcore/iter/adapters.rs:1734:21
[00:03:02]     --> src/libcore/iter/adapters.rs:1734:21
[00:03:02]      |
[00:03:02] 1734 |                     LoopState::from_try(fold(acc, x))
[00:03:02]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:02] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:02]     --> src/libcore/iter/adapters.rs:1737:21
[00:03:02]      |
[00:03:02]      |
[00:03:02] 1737 |                     LoopState::Break(Try::from_ok(acc))
[00:03:02]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:02] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:02]     --> src/libcore/iter/adapters.rs:1876:29
[00:03:02]      |
[00:03:02]      |
[00:03:02] 1876 |                 if n == 0 { LoopState::Break(r) }
[00:03:02]      |                             ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:02] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:02]     --> src/libcore/iter/adapters.rs:1877:24
[00:03:02]      |
[00:03:02]      |
[00:03:02] 1877 |                 else { LoopState::from_try(r) }
[00:03:02]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:02] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:02]     --> src/libcore/iter/adapters.rs:1958:30
[00:03:02]      |
[00:03:02]      |
[00:03:02] 1958 |                 if *n == 0 { LoopState::Break(r) }
[00:03:02]      |                              ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:02] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:02]     --> src/libcore/iter/adapters.rs:1959:24
[00:03:02]      |
[00:03:02]      |
[00:03:02] 1959 |                 else { LoopState::from_try(r) }
[00:03:02]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:02] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:02]     --> src/libcore/iter/adapters.rs:2026:25
[00:03:02]      |
[00:03:02]      |
[00:03:02] 2026 |                 None => LoopState::Break(Try::from_ok(acc)),
[00:03:02]      |                         ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:02] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:02]     --> src/libcore/iter/adapters.rs:2027:28
[00:03:02]      |
[00:03:02]      |
[00:03:02] 2027 |                 Some(x) => LoopState::from_try(fold(acc, x)),
[00:03:02]      |                            ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:02] 
[00:03:03] warning: unused import: `ZipImpl`
[00:03:03]    |
[00:03:03]    |
[00:03:03] 19 | use super::super::{ChainState, FromIterator, ZipImpl};
[00:03:03]    |
[00:03:03]    = note: #[warn(unused_imports)] on by default
[00:03:03] 
[00:03:03]    Compiling compiler_builtins v0.1.2
[00:03:03]    Compiling compiler_builtins v0.1.2
[00:03:03]    Compiling cmake v0.1.33
[00:03:03]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:06]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:06]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:07]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:07]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:11] error[E0599]: no function or associated item named `new` found for type `iter::adapters::Zip<_, _>` in the current scope
[00:03:11]     |
[00:03:11] 516 |         Zip::new(self, other.into_iter())
[00:03:11] 516 |         Zip::new(self, other.into_iter())
[00:03:11]     |         ^^^^^^^^ function or associated item not found in `iter::adapters::Zip<_, _>`
[00:03:11]    ::: src/libcore/iter/adapters.rs:625:1
[00:03:11]     |
[00:03:11]     |
[00:03:11] 625 | pub struct Zip<A, B> {
[00:03:11]     | -------------------- function or associated item `new` not found for this
[00:03:11]     = help: items from traits can only be used if the trait is implemented and in scope
[00:03:11]     = help: items from traits can only be used if the trait is implemented and in scope
[00:03:11]     = note: the following trait defines an item `new`, perhaps you need to implement it:
[00:03:11]             candidate #1: `iter::adapters::ZipImpl`
[00:03:14] error: aborting due to 11 previous errors
[00:03:14] 
[00:03:14] Some errors occurred: E0432, E0433, E0599.
[00:03:14] For more information about an error, try `rustc --explain E0432`.
[00:03:14] For more information about an error, try `rustc --explain E0432`.
[00:03:14] error: Could not compile `core`.
[00:03:14] 
[00:03:14] To learn more, run the command again with --verbose.
[00:03:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:14] expected success, got: exit code: 101
[00:03:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:14] Build completed unsuccessfully in 0:00:19
[00:03:14] make: *** [all] Error 1
[00:03:14] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:109aa124
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 18 00:02:39 UTC 2018
---
travis_time:end:007ec368:start=1545091359948067883,finish=1545091359953964895,duration=5897012
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0079fbab
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:375da5aa
travis_time:start:375da5aa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
