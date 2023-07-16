plain
travis_time:end:191ebc50:start=1550494922604814971,finish=1550495030179799176,duration=107574984205
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:21]   Downloaded digest v0.7.6
[00:03:21]   Downloaded bytes v0.4.11
[00:03:21]   Downloaded fuchsia-zircon-sys v0.3.3
[00:03:21]   Downloaded mdbook v0.2.3
[00:03:21]   Downloaded blake2-rfc v0.2.18
[00:03:21]   Downloaded compiler_builtins v0.1.5
[00:03:21]   Downloaded cloudabi v0.0.3
[00:03:21]   Downloaded unreachable v1.0.0
[00:03:21]   Downloaded xattr v0.2.2
---
[00:03:27]   Downloaded if_chain v0.1.3
[00:03:27]   Downloaded signal-hook v0.1.7
[00:03:27]   Downloaded ucd-trie v0.1.1
[00:03:27]   Downloaded parking_lot v0.6.4
[00:03:27]   Downloaded constant_time_eq v0.1.3
[00:03:27]   Downloaded chalk-macros v0.1.0
[00:03:27]   Downloaded clap v2.32.0
[00:03:27]   Downloaded racer v2.1.19
[00:03:27]   Downloaded miow v0.3.3
---
[00:03:28]   Downloaded hex v0.3.2
[00:03:28]   Downloaded winapi-util v0.1.1
[00:03:28]   Downloaded mio-uds v0.6.7
[00:03:28]   Downloaded termcolor v1.0.4
[00:03:28]   Downloaded argon2rs v0.2.5
[00:03:29]   Downloaded pest_derive v2.1.0
[00:03:29]   Downloaded rand_core v0.2.2
[00:03:29]   Downloaded xz2 v0.1.5
[00:03:29]   Downloaded ansi_term v0.11.0
---
[00:04:57] 
[00:04:57] error[E0433]: failed to resolve: use of undeclared type or module `AtomicU64`
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/cache.rs:87:15
[00:04:57]    |
[00:04:57] 87 |         Cache(AtomicU64::new(u64::max_value()))
[00:04:57] 
[00:04:57] error[E0433]: failed to resolve: use of undeclared type or module `Ordering`
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/cache.rs:92:21
[00:04:57]    |
[00:04:57]    |
[00:04:57] 92 |         self.0.load(Ordering::Relaxed) == u64::max_value()
[00:04:57] 
[00:04:57] error[E0433]: failed to resolve: use of undeclared type or module `Ordering`
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/cache.rs:98:31
[00:04:57]    |
[00:04:57]    |
[00:04:57] 98 |         test_bit(CACHE.0.load(Ordering::Relaxed), bit)
[00:04:57] 
[00:04:57] error[E0433]: failed to resolve: use of undeclared type or module `Ordering`
[00:04:57]    --> src/libstd/../stdsimd/crates/std_detect/src/detect/cache.rs:104:31
[00:04:57]     |
[00:04:57]     |
[00:04:57] 104 |         self.0.store(value.0, Ordering::Relaxed);
[00:04:57] 
[00:04:57] error[E0433]: failed to resolve: use of undeclared type or module `mem`
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:59:13
[00:04:57]    |
[00:04:57]    |
[00:04:57] 59 |             mem::transmute(ebx),
[00:04:57]    |             ^^^ use of undeclared type or module `mem`
[00:04:57] error[E0433]: failed to resolve: use of undeclared type or module `mem`
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:60:13
[00:04:57]    |
[00:04:57]    |
[00:04:57] 60 |             mem::transmute(edx),
[00:04:57]    |             ^^^ use of undeclared type or module `mem`
[00:04:57] error[E0433]: failed to resolve: use of undeclared type or module `mem`
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:61:13
[00:04:57]    |
[00:04:57]    |
[00:04:57] 61 |             mem::transmute(ecx),
[00:04:57]    |             ^^^ use of undeclared type or module `mem`
[00:04:57] error[E0433]: failed to resolve: use of undeclared type or module `mem`
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:63:35
[00:04:57]    |
[00:04:57]    |
[00:04:57] 63 |         let vendor_id: [u8; 12] = mem::transmute(vendor_id);
[00:04:57]    |                                   ^^^ use of undeclared type or module `mem`
[00:04:57] error[E0412]: cannot find type `AtomicU64` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/cache.rs:76:14
[00:04:57]    |
[00:04:57]    |
[00:04:57] 76 | struct Cache(AtomicU64);
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57] 6  | use core::sync::atomic::AtomicU64;
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::sync::atomic::AtomicU64;
[00:04:57]    |
[00:04:57] 
[00:04:57] error[E0425]: cannot find function `has_cpuid` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:39:9
[00:04:57]    |
[00:04:57] 39 |     if !has_cpuid() {
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::has_cpuid;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::has_cpuid;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0425]: cannot find function `__cpuid` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:57:13
[00:04:57]    |
[00:04:57] 57 |         } = __cpuid(0);
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::__cpuid;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::__cpuid;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0422]: cannot find struct, variant or union type `CpuidResult` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:52:13
[00:04:57]    |
[00:04:57] 52 |         let CpuidResult {
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::CpuidResult;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::CpuidResult;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0425]: cannot find function `__cpuid` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:78:18
[00:04:57]    |
[00:04:57] 78 |     } = unsafe { __cpuid(0x0000_0001_u32) };
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::__cpuid;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::__cpuid;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0422]: cannot find struct, variant or union type `CpuidResult` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:74:9
[00:04:57]    |
[00:04:57] 74 |     let CpuidResult {
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::CpuidResult;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::CpuidResult;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0425]: cannot find function `__cpuid` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:84:53
[00:04:57]    |
[00:04:57] 84 |         let CpuidResult { ebx, ecx, .. } = unsafe { __cpuid(0x0000_0007_u32) };
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::__cpuid;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::__cpuid;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0422]: cannot find struct, variant or union type `CpuidResult` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:84:13
[00:04:57]    |
[00:04:57] 84 |         let CpuidResult { ebx, ecx, .. } = unsafe { __cpuid(0x0000_0007_u32) };
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::CpuidResult;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::CpuidResult;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0425]: cannot find function `__cpuid` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:96:18
[00:04:57]    |
[00:04:57] 96 |     } = unsafe { __cpuid(0x8000_0000_u32) };
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::__cpuid;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::__cpuid;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0422]: cannot find struct, variant or union type `CpuidResult` in this scope
[00:04:57]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:93:9
[00:04:57]    |
[00:04:57] 93 |     let CpuidResult {
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]    |
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::CpuidResult;
[00:04:57]    |
[00:04:57] 6  | use core::arch::x86_64::CpuidResult;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0425]: cannot find function `__cpuid` in this scope
[00:04:57]    --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:101:48
[00:04:57]     |
[00:04:57] 101 |         let CpuidResult { ecx, .. } = unsafe { __cpuid(0x8000_0001_u32) };
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]     |
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::__cpuid;
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::__cpuid;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0422]: cannot find struct, variant or union type `CpuidResult` in this scope
[00:04:57]    --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:101:13
[00:04:57]     |
[00:04:57] 101 |         let CpuidResult { ecx, .. } = unsafe { __cpuid(0x8000_0001_u32) };
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]     |
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::CpuidResult;
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::CpuidResult;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0425]: cannot find function `_xgetbv` in this scope
[00:04:57]    --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:164:37
[00:04:57]     |
[00:04:57] 164 |                 let xcr0 = unsafe { _xgetbv(0) };
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]     |
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::_xgetbv;
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::_xgetbv;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0425]: cannot find function `__cpuid_count` in this scope
[00:04:57]    --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:192:38
[00:04:57]     |
[00:04:57] 192 |                         } = unsafe { __cpuid_count(0xd_u32, 1) };
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]     |
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::__cpuid_count;
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::__cpuid_count;
[00:04:57] 
[00:04:57] 
[00:04:57] error[E0422]: cannot find struct, variant or union type `CpuidResult` in this scope
[00:04:57]    --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:189:29
[00:04:57]     |
[00:04:57] 189 |                         let CpuidResult {
[00:04:57] help: possible candidates are found in other modules, you can import them into scope
[00:04:57]     |
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::CpuidResult;
[00:04:57]     |
[00:04:57] 6   | use core::arch::x86_64::CpuidResult;
[00:04:57] 
[00:04:57] error: unused import: `sync::atomic::Ordering`
[00:04:57]  --> src/libstd/../stdsimd/crates/std_detect/src/detect/cache.rs:6:5
[00:04:57]   |
---
[00:04:57]   |
[00:04:57] 9 | use sync::atomic::AtomicU64;
[00:04:57]   |     ^^^^^^^^^^^^^^^^^^^^^^^
[00:04:57] 
[00:04:57] error: unused import: `arch::x86_64::*`
[00:04:57]  --> src/libstd/../stdsimd/crates/std_detect/src/detect/os/x86.rs:6:5
[00:04:57] 6 | use arch::x86_64::*;
[00:04:57]   |     ^^^^^^^^^^^^^^^
[00:04:57] 
[00:04:57] error: unused import: `mem`
---
[00:05:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:00] expected success, got: exit code: 101
[00:05:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:00] Build completed unsuccessfully in 0:00:44
[00:05:00] Makefile:18: recipe for target 'all' failed
[00:05:00] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05b6bd12
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 18 13:09:00 UTC 2019
---
199324 ./obj/build/cache/2019-02-17
156148 ./src/llvm-project/clang
155960 ./obj/build/bootstrap/debug/incremental
141180 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141176 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn/s-f9lrnxqm42-bhi434-1wm2zcqodn409
108528 ./src/llvm-project/lldb
97552 ./src/llvm-project/clang/test
89964 ./src/llvm-emscripten/test/CodeGen
79632 ./.git/modules
---
travis_time:end:0831b2ce:start=1550495340878672293,finish=1550495340883390341,duration=4718048
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ab71fd3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17173948
travis_time:start:17173948
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00d66f2e
$ dmesg | grep -i kill
