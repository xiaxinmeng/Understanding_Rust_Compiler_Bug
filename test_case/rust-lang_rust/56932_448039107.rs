plain
travis_time:end:065e6897:start=1545089273722259309,finish=1545089274757273864,duration=1035014555
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:28]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:31] error[E0432]: unresolved import `iter::flatten_compat`
[00:03:31]   --> src/libcore/iter/traits/iterator.rs:16:30
[00:03:31]    |
[00:03:31] 16 | use iter::{Flatten, FlatMap, flatten_compat};
[00:03:31]    |                              ^^^^^^^^^^^^^^ no `flatten_compat` in `iter`
[00:03:31] 
[00:03:31] error[E0432]: unresolved imports `iter::ChainState`, `iter::ZipImpl`
[00:03:31]    |
[00:03:31]    |
[00:03:31] 19 | use iter::{ChainState, FromIterator, ZipImpl};
[00:03:31]    |            ^^^^^^^^^^                ^^^^^^^ no `ZipImpl` in `iter`
[00:03:31]    |            no `ChainState` in `iter`
[00:03:31] 
[00:03:32] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:32]     --> src/libcore/iter/adapters.rs:1734:21
[00:03:32]     --> src/libcore/iter/adapters.rs:1734:21
[00:03:32]      |
[00:03:32] 1734 |                     LoopState::from_try(fold(acc, x))
[00:03:32]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:32] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:32]     --> src/libcore/iter/adapters.rs:1737:21
[00:03:32]      |
[00:03:32]      |
[00:03:32] 1737 |                     LoopState::Break(Try::from_ok(acc))
[00:03:32]      |                     ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:32] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:32]     --> src/libcore/iter/adapters.rs:1876:29
[00:03:32]      |
[00:03:32]      |
[00:03:32] 1876 |                 if n == 0 { LoopState::Break(r) }
[00:03:32]      |                             ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:32] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:32]     --> src/libcore/iter/adapters.rs:1877:24
[00:03:32]      |
[00:03:32]      |
[00:03:32] 1877 |                 else { LoopState::from_try(r) }
[00:03:32]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:32] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:32]     --> src/libcore/iter/adapters.rs:1958:30
[00:03:32]      |
[00:03:32]      |
[00:03:32] 1958 |                 if *n == 0 { LoopState::Break(r) }
[00:03:32]      |                              ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:32] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:32]     --> src/libcore/iter/adapters.rs:1959:24
[00:03:32]      |
[00:03:32]      |
[00:03:32] 1959 |                 else { LoopState::from_try(r) }
[00:03:32]      |                        ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:32] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:32]     --> src/libcore/iter/adapters.rs:2026:25
[00:03:32]      |
[00:03:32]      |
[00:03:32] 2026 |                 None => LoopState::Break(Try::from_ok(acc)),
[00:03:32]      |                         ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:32] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:03:32]     --> src/libcore/iter/adapters.rs:2027:28
[00:03:32]      |
[00:03:32]      |
[00:03:32] 2027 |                 Some(x) => LoopState::from_try(fold(acc, x)),
[00:03:32]      |                            ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:03:32] error[E0603]: trait `Iterator` is private
[00:03:32]    --> src/libcore/iter/mod.rs:322:23
[00:03:32]     |
[00:03:32]     |
[00:03:32] 322 | pub use self::traits::Iterator;
[00:03:32] 
[00:03:32] error[E0603]: trait `FromIterator` is private
[00:03:32]    --> src/libcore/iter/mod.rs:341:24
[00:03:32]     |
[00:03:32]     |
[00:03:32] 341 | pub use self::traits::{FromIterator, IntoIterator, DoubleEndedIterator, Extend};
[00:03:32] 
[00:03:32] error[E0603]: trait `IntoIterator` is private
[00:03:32]    --> src/libcore/iter/mod.rs:341:38
[00:03:32]     |
[00:03:32]     |
[00:03:32] 341 | pub use self::traits::{FromIterator, IntoIterator, DoubleEndedIterator, Extend};
[00:03:32] 
[00:03:32] error[E0603]: trait `DoubleEndedIterator` is private
[00:03:32]    --> src/libcore/iter/mod.rs:341:52
[00:03:32]     |
[00:03:32]     |
[00:03:32] 341 | pub use self::traits::{FromIterator, IntoIterator, DoubleEndedIterator, Extend};
[00:03:32] 
[00:03:32] error[E0603]: trait `Extend` is private
[00:03:32]    --> src/libcore/iter/mod.rs:341:73
[00:03:32]     |
[00:03:32]     |
[00:03:32] 341 | pub use self::traits::{FromIterator, IntoIterator, DoubleEndedIterator, Extend};
[00:03:32] 
[00:03:32] error[E0603]: trait `ExactSizeIterator` is private
[00:03:32]    --> src/libcore/iter/mod.rs:343:24
[00:03:32]     |
[00:03:32]     |
[00:03:32] 343 | pub use self::traits::{ExactSizeIterator, Sum, Product};
[00:03:32] 
[00:03:32] 
[00:03:32] error[E0603]: trait `Sum` is private
[00:03:32]     |
[00:03:32]     |
[00:03:32] 343 | pub use self::traits::{ExactSizeIterator, Sum, Product};
[00:03:32] 
[00:03:32] 
[00:03:32] error[E0603]: trait `Product` is private
[00:03:32]     |
[00:03:32]     |
[00:03:32] 343 | pub use self::traits::{ExactSizeIterator, Sum, Product};
[00:03:32] 
[00:03:32] error[E0603]: trait `FusedIterator` is private
[00:03:32]    --> src/libcore/iter/mod.rs:345:23
[00:03:32]     |
[00:03:32]     |
[00:03:32] 345 | pub use self::traits::FusedIterator;
[00:03:32] 
[00:03:32] 
[00:03:32] error[E0603]: trait `TrustedLen` is private
[00:03:32]     |
[00:03:32]     |
[00:03:32] 347 | pub use self::traits::TrustedLen;
[00:03:32] 
[00:03:32] error[E0603]: trait `Iterator` is private
[00:03:32]   --> src/libcore/iter/adapters.rs:17:21
[00:03:32]    |
[00:03:32]    |
[00:03:32] 17 | use super::traits::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};
[00:03:32] 
[00:03:32] error[E0603]: trait `DoubleEndedIterator` is private
[00:03:32]   --> src/libcore/iter/adapters.rs:17:31
[00:03:32]    |
[00:03:32]    |
[00:03:32] 17 | use super::traits::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};
[00:03:32] 
[00:03:32] error[E0603]: trait `ExactSizeIterator` is private
[00:03:32]   --> src/libcore/iter/adapters.rs:17:52
[00:03:32]    |
[00:03:32]    |
[00:03:32] 17 | use super::traits::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};
[00:03:32] 
[00:03:32] error[E0603]: trait `FusedIterator` is private
[00:03:32]   --> src/libcore/iter/adapters.rs:17:71
[00:03:32]    |
[00:03:32]    |
[00:03:32] 17 | use super::traits::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};
[00:03:32] 
[00:03:32] 
[00:03:32] error[E0603]: trait `TrustedLen` is private
[00:03:32]   --> src/libcore/iter/adapters.rs:17:86
[00:03:32]    |
[00:03:32] 17 | use super::traits::{Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, TrustedLen};
[00:03:32] 
[00:03:33]    Compiling compiler_builtins v0.1.2
[00:03:33]    Compiling cmake v0.1.33
[00:03:33]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:33]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:33] warning: unused import: `ZipImpl`
[00:03:33]    |
[00:03:33]    |
[00:03:33] 19 | use iter::{ChainState, FromIterator, ZipImpl};
[00:03:33]    |
[00:03:33]    = note: #[warn(unused_imports)] on by default
[00:03:33] 
[00:03:35] error: aborting due to 25 previous errors
---
[00:03:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:36] expected success, got: exit code: 101
[00:03:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:36] Build completed unsuccessfully in 0:00:11
[00:03:36] Makefile:28: recipe for target 'all' failed
[00:03:36] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:230dfa74
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 17 23:31:40 UTC 2018
---
travis_time:end:017f28a3:start=1545089500835017952,finish=1545089500839933199,duration=4915247
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a7c9718
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:387321cc
travis_time:start:387321cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0057418f
$ dmesg | grep -i kill
