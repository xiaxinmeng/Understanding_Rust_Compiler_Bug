plain
travis_time:end:2487930c:start=1541988898209429937,finish=1541988951143632123,duration=52934202186
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:54]     Checking rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:48:54]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:48:54]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:48:55]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:49:01] error: cannot find macro `Result::Ok!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `Weak::upgrade!` in this scope
[00:49:01] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
[00:49:01]     --> /checkout/src/liballoc/rc.rs:1181:29
[00:49:01]      |
[00:49:01] 1181 |     /// Calling [`upgrade`][Weak::upgrade] on the return value always gives [`None`].
---
[00:49:01]      |                             ^^^^^^^^^^^^^ cannot be resolved, ignoring
[00:49:01]      |
[00:49:01]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:01] 
[00:49:01] error: cannot find macro `u8!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `Default!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `PartialEq!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `BufWriter!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `crate::sync::atomic::compiler_fence!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `crate::sync::atomic::fence!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `crate::sync::atomic!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `crate::sync::Arc!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `crate::sync::Barrier!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `crate::sync::Condvar!` in this scope
[00:49:01] 
[00:49:01] error: cannot find macro `crate::sync::mpsc!` in this scope
[00:49:01] error: cannot find macro `[0m     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:01] 
[00:49:01] error: cannot find macro `String::push_str!` in this scope
[00:49:01] 
[00:49:01] 
[00:49:01] error: cannot find macro `chunks!` in this scope
[00:49:01] warning: `[chunks]` cannot be resolved, ignoring it...
[00:49:01]    --> /checkout/src/libcore/slice/mod.rs:861:52
[00:49:01]     |
[00:49:01] 861 |     /// resulting code better than in the case of [`chunks`].
[00:49:01] 861 |     /// resulting code better than in the case of [`chunks`].
[00:49:01]     |                                                    ^^^^^^^^ cannot be resolved, ignoring
[00:49:01]     |
[00:49:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:01] 
[00:49:01] error: cannot find macro `chunks_mut!` in this scope
[00:49:01] warning: `[chunks_mut]` cannot be resolved, ignoring it...
[00:49:01]    --> /checkout/src/libcore/slice/mod.rs:901:52
[00:49:01]     |
[00:49:01] 901 |     /// resulting code better than in the case of [`chunks_mut`           ^^^^^^^^^^^^^^^ cannot be resolved, ignoring
[00:49:01] 901 |     /// resulting code better than in the case of [`chunks_mut`           ^^^^^^^^^^^^^^^ cannot be resolved, ignoring
[00:49:01]     |
[00:49:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:01] 
[00:49:01] error: Could not document `std`.
[00:49:01] 
[00:49:01] Caused by:
[00:49:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std libstd/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-3c6fe18e3f940a16.rmeta --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-b50a2c2614451fcc.rmeta --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-18eadef5741d6149.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d4e445232d9f4530.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-e99f7499326b08cb.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-1cb5d76dcefc713b.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-7f0db12775373ff1.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-c0e63a103f3008d6.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-64369f3f20969201.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-3faa3eaec1535088.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-2277b67d89a761ca.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-b11c67038d913a1c.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-7d8e6e32e8b540cb.rmeta` (exit code: 1)
[00:49:01] 
[00:49:01] 
[00:49:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libs2 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
35596 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
35588 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
34916 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/src
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
---
travis_time:end:06c19aa6:start=1541991903281908981,finish=1541991903286803878,duration=4894897
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08e2fefe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:171ce140
travis_time:start:171ce140
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02c9e544
$ dmesg | grep -i kill
