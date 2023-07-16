plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1f8f5510
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:05:22]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:26] warning: unused variable: `additional`
[00:05:26]    --> src/libstd/collections/hash/map.rs:412:35
[00:05:26]     |
[00:05:26] 412 |     pub fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr> {
[00:05:26]     |                                   ^^^^^^^^^^ help: consider using `_additional` instead
[00:05:26]     = note: #[warn(unused_variables)] on by default
[00:05:26] 
[00:05:40]     Finished release [optimized] target(s) in 50.48s
[00:05:40] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
[00:49:56]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:49:56]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:49:56]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:50:00] error[E0432]: unresolved import
[00:50:00]   --> src/libcore/num/dec2flt/rawfp.rs:33:11
[00:50:00]    |
[00:50:00] 33 | use fmt::{Debug, LowerExp};
[00:50:00] 
[00:50:00] error: cannot determine resolution for the derive macro `Debug`
[00:50:00] error: cannot determine resolution for the derive macro `Debug`
[00:50:00]   --> src/libcore/num/dec2flt/rawfp.rs:40:23
[00:50:00]    |
[00:50:00] 40 | #[derive(Copy, Clone, Debug)]
[00:50:00]    |
[00:50:00]    = note: import resolution is stuck, try simplifying macro imports
[00:50:00] 
[00:50:00] 
[00:50:01] error[E0416]: identifier `Acquire` is bound more than once in the same pattern
[00:50:01]      |
[00:50:01]      |
[00:50:01] 2091 |         (Acquire, Acquire) => intrinsics::atomic_cxchg_acq(dst, old, new),
[00:50:01]      |                   ^^^^^^^ used in a pattern more than once
[00:50:01] 
[00:50:01] error[E0416]: identifier `Relaxed` is bound more than once in the same pattern
[00:50:01]      |
[00:50:01]      |
[00:50:01] 2094 |         (Relaxed, Relaxed) => intrinsics::atomic_cxchg_relaxed(dst, old, new),
[00:50:01]      |                   ^^^^^^^ used in a pattern more than once
[00:50:01] 
[00:50:01] error[E0416]: identifier `SeqCst` is bound more than once in the same pattern
[00:50:01]      |
[00:50:01]      |
[00:50:01] 2095 |         (SeqCst, SeqCst) => intrinsics::atomic_cxchg(dst, old, new),
[00:50:01]      |                  ^^^^^^ used in a pattern more than once
[00:50:01] 
[00:50:01] error[E0416]: identifier `Acquire` is bound more than once in the same pattern
[00:50:01]      |
[00:50:01]      |
[00:50:01] 2116 |         (Acquire, Acquire) => intrinsics::atomic_cxchgweak_acq(dst, old, new),
[00:50:01]      |                   ^^^^^^^ used in a pattern more than once
[00:50:01] 
[00:50:01] error[E0416]: identifier `Relaxed` is bound more than once in the same pattern
[00:50:01]      |
[00:50:01]      |
[00:50:01] 2119 |         (Relaxed, Relaxed) => intrinsics::atomic_cxchgweak_relaxed(dst, old, new),
[00:50:01]      |                   ^^^^^^^ used in a pattern more than once
[00:50:01] 
[00:50:01] error[E0416]: identifier `SeqCst` is bound more than once in the same pattern
[00:50:01]      |
[00:50:01]      |
[00:50:01] 2120 |         (SeqCst, SeqCst) => intrinsics::atomic_cxchgweak(dst, old, new),
[00:50:01]      |                  ^^^^^^ used in a pattern more than once
[00:50:02] error: aborting due to 8 previous errors
[00:50:02] 
[00:50:02] Some errors occurred: E0416, E0432.
[00:50:02] For more information about an error, try `rustc --explain E0416`.
---
travis_time:end:0d9aa0f5:start=1543252380560909682,finish=1543252380570961872,duration=10052190
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:009923ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2056231c
travis_time:start:2056231c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14508654
$ dmesg | grep -i kill
