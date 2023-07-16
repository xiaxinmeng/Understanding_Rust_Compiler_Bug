plain
travis_time:end:0014c298:start=1554059834676115786,finish=1554059836897934299,duration=2221818513
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:57]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:57]    Compiling rustc-demangle v0.1.10
[00:04:57]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:05:01]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:03] error: unused imports: `Consumer`, `ReverseConsumer`, `ReverseSearcher`, `Searcher`, `Span`
[00:05:03]   --> src/libstd/sys_common/os_str_bytes.rs:17:26
[00:05:03]    |
[00:05:03] 17 | use crate::needle::{Hay, Span, Searcher, ReverseSearcher, Consumer, ReverseConsumer};
[00:05:03]    |
[00:05:03]    = note: `-D unused-imports` implied by `-D warnings`
[00:05:03] 
[00:05:05] error: aborting due to previous error
---
198084 ./obj/build/cache/2019-03-20
158088 ./obj/build/bootstrap/debug/incremental
156496 ./src/llvm-project/clang
143172 ./obj/build/bootstrap/debug/incremental/bootstrap-2bzajeq8agxyg
143168 ./obj/build/bootstrap/debug/incremental/bootstrap-2bzajeq8agxyg/s-fav9bf7cd6-1sgm0mp-drn7e21xssa3
108532 ./src/llvm-project/lldb
97584 ./src/llvm-project/clang/test
94552 ./.git
89964 ./src/llvm-emscripten/test/CodeGen
---
travis_time:end:0441edcc:start=1554060154033927521,finish=1554060154038538123,duration=4610602
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0096118e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:237209a6
travis_time:start:237209a6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0757972c
$ dmesg | grep -i kill
