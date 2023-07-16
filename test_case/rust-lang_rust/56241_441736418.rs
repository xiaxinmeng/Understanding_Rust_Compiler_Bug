plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:05edbc7a
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:47:07]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:47:07]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:47:07]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:47:10] error[E0432]: unresolved import
[00:47:10]   --> src/libcore/num/dec2flt/rawfp.rs:33:11
[00:47:10]    |
[00:47:10] 33 | use fmt::{Debug, LowerExp};
[00:47:10] 
[00:47:10] error: cannot determine resolution for the derive macro `Debug`
[00:47:10] error: cannot determine resolution for the derive macro `Debug`
[00:47:10]   --> src/libcore/num/dec2flt/rawfp.rs:40:23
[00:47:10]    |
[00:47:10] 40 | #[derive(Copy, Clone, Debug)]
[00:47:10]    |
[00:47:10]    = note: import resolution is stuck, try simplifying macro imports
[00:47:10] 
[00:47:10] 
[00:47:10] error[E0416]: identifier `Acquire` is bound more than once in the same pattern
[00:47:10]      |
[00:47:10]      |
[00:47:10] 2091 |         (Acquire, Acquire) => intrinsics::atomic_cxchg_acq(dst, old, new),
[00:47:10]      |                   ^^^^^^^ used in a pattern more than once
[00:47:10] 
[00:47:10] error[E0416]: identifier `Relaxed` is bound more than once in the same pattern
[00:47:10]      |
[00:47:10]      |
[00:47:10] 2094 |         (Relaxed, Relaxed) => intrinsics::atomic_cxchg_relaxed(dst, old, new),
[00:47:10]      |                   ^^^^^^^ used in a pattern more than once
[00:47:10] 
[00:47:10] error[E0416]: identifier `SeqCst` is bound more than once in the same pattern
[00:47:10]      |
[00:47:10]      |
[00:47:10] 2095 |         (SeqCst, SeqCst) => intrinsics::atomic_cxchg(dst, old, new),
[00:47:10]      |                  ^^^^^^ used in a pattern more than once
[00:47:10] 
[00:47:10] error[E0416]: identifier `Acquire` is bound more than once in the same pattern
[00:47:10]      |
[00:47:10]      |
[00:47:10] 2116 |         (Acquire, Acquire) => intrinsics::atomic_cxchgweak_acq(dst, old, new),
[00:47:10]      |                   ^^^^^^^ used in a pattern more than once
[00:47:10] 
[00:47:10] error[E0416]: identifier `Relaxed` is bound more than once in the same pattern
[00:47:10]      |
[00:47:10]      |
[00:47:10] 2119 |         (Relaxed, Relaxed) => intrinsics::atomic_cxchgweak_relaxed(dst, old, new),
[00:47:10]      |                   ^^^^^^^ used in a pattern more than once
[00:47:10] 
[00:47:10] error[E0416]: identifier `SeqCst` is bound more than once in the same pattern
[00:47:10]      |
[00:47:10]      |
[00:47:10] 2120 |         (SeqCst, SeqCst) => intrinsics::atomic_cxchgweak(dst, old, new),
[00:47:10]      |                  ^^^^^^ used in a pattern more than once
[00:47:11] error: aborting due to 8 previous errors
[00:47:11] 
[00:47:11] Some errors occurred: E0416, E0432.
[00:47:11] For more information about an error, try `rustc --explain E0416`.
---
travis_time:end:0a9aeb56:start=1543254768289710638,finish=1543254768295832212,duration=6121574
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:056e60b6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c0c9ede
travis_time:start:1c0c9ede
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:171ff89d
$ dmesg | grep -i kill
