plain
travis_time:end:03b58678:start=1543251899547419502,finish=1543251953974008641,duration=54426589139
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:19:12]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:19:12]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:19:12]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:17] error[E0432]: unresolved import
[00:19:17]   --> src/libcore/num/dec2flt/rawfp.rs:33:11
[00:19:17]    |
[00:19:17] 33 | use fmt::{Debug, LowerExp};
[00:19:17] 
[00:19:17] error: cannot determine resolution for the derive macro `Debug`
[00:19:17] error: cannot determine resolution for the derive macro `Debug`
[00:19:17]   --> src/libcore/num/dec2flt/rawfp.rs:40:23
[00:19:17]    |
[00:19:17] 40 | #[derive(Copy, Clone, Debug)]
[00:19:17]    |
[00:19:17]    = note: import resolution is stuck, try simplifying macro imports
[00:19:17] 
[00:19:17] 
[00:19:17] error[E0416]: identifier `Acquire` is bound more than once in the same pattern
[00:19:17]      |
[00:19:17]      |
[00:19:17] 2091 |         (Acquire, Acquire) => intrinsics::atomic_cxchg_acq(dst, old, new),
[00:19:17]      |                   ^^^^^^^ used in a pattern more than once
[00:19:17] 
[00:19:17] error[E0416]: identifier `Relaxed` is bound more than once in the same pattern
[00:19:17]      |
[00:19:17]      |
[00:19:17] 2094 |         (Relaxed, Relaxed) => intrinsics::atomic_cxchg_relaxed(dst, old, new),
[00:19:17]      |                   ^^^^^^^ used in a pattern more than once
[00:19:17] 
[00:19:17] error[E0416]: identifier `SeqCst` is bound more than once in the same pattern
[00:19:17]      |
[00:19:17]      |
[00:19:17] 2095 |         (SeqCst, SeqCst) => intrinsics::atomic_cxchg(dst, old, new),
[00:19:17]      |                  ^^^^^^ used in a pattern more than once
[00:19:17] 
[00:19:17] error[E0416]: identifier `Acquire` is bound more than once in the same pattern
[00:19:17]      |
[00:19:17]      |
[00:19:17] 2116 |         (Acquire, Acquire) => intrinsics::atomic_cxchgweak_acq(dst, old, new),
[00:19:17]      |                   ^^^^^^^ used in a pattern more than once
[00:19:17] 
[00:19:17] error[E0416]: identifier `Relaxed` is bound more than once in the same pattern
[00:19:17]      |
[00:19:17]      |
[00:19:17] 2119 |         (Relaxed, Relaxed) => intrinsics::atomic_cxchgweak_relaxed(dst, old, new),
[00:19:17]      |                   ^^^^^^^ used in a pattern more than once
[00:19:17] 
[00:19:17] error[E0416]: identifier `SeqCst` is bound more than once in the same pattern
[00:19:17]      |
[00:19:17] 2120error: aborting due to 8 previous errors
[00:19:19] 
[00:19:19] Some errors occurred: E0416, E0432.
---
[00:19:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:19:19] expected success, got: exit code: 101
[00:19:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:19] Build completed unsuccessfully in 0:15:50
[00:19:19] Makefile:28: recipe for target 'all' failed
[00:19:19] make: *** [all] Error 1
57304 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
57300 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
57296 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
56896 ./src/llvm/test/MC
