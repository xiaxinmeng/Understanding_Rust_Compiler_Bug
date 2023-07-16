plain
travis_time:end:1529e080:start=1543249255992989136,finish=1543249321851190115,duration=65858200979
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:36]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:03:40] warning: unused variable: `additional`
[00:03:40]    --> src/libstd/collections/hash/map.rs:412:35
[00:03:40]     |
[00:03:40] 412 |     pub fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr> {
[00:03:40]     |                                   ^^^^^^^^^^ help: consider using `_additional` instead
[00:03:40]     = note: #[warn(unused_variables)] on by default
[00:03:40] 
[00:03:52]     Finished release [optimized] target(s) in 45.76s
[00:03:52] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
[00:18:31]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:18:31]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:18:31]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:18:36] error[E0432]: unresolved import
[00:18:36]   --> src/libcore/num/dec2flt/rawfp.rs:33:11
[00:18:36]    |
[00:18:36] 33 | use fmt::{Debug, LowerExp};
[00:18:36] 
[00:18:36] error: cannot determine resolution for the derive macro `Debug`
[00:18:36] error: cannot determine resolution for the derive macro `Debug`
[00:18:36]   --> src/libcore/num/dec2flt/rawfp.rs:40:23
[00:18:36]    |
[00:18:36] 40 | #[derive(Copy, Clone, Debug)]
[00:18:36]    |
[00:18:36]    = note: import resolution is stuck, try simplifying macro imports
[00:18:36] 
[00:18:36] 
[00:18:37] error[E0416]: identifier `Acquire` is bound more than once in the same pattern
[00:18:37]      |
[00:18:37]      |
[00:18:37] 2091 |         (Acquire, Acquire) => intrinsics::atomic_cxchg_acq(dst, old, new),
[00:18:37]      |                   ^^^^^^^ used in a pattern more than once
[00:18:37] 
[00:18:37] error[E0416]: identifier `Relaxed` is bound more than once in the same pattern
[00:18:37]      |
[00:18:37]      |
[00:18:37] 2094 |         (Relaxed, Relaxed) => intrinsics::atomic_cxchg_relaxed(dst, old, new),
[00:18:37]      |                   ^^^^^^^ used in a pattern more than once
[00:18:37] 
[00:18:37] error[E0416]: identifier `SeqCst` is bound more than once in the same pattern
[00:18:37]      |
[00:18:37]      |
[00:18:37] 2095 |         (SeqCst, SeqCst) => intrinsics::atomic_cxchg(dst, old, new),
[00:18:37]      |                  ^^^^^^ used in a pattern more than once
[00:18:37] 
[00:18:37] error[E0416]: identifier `Acquire` is bound more than once in the same pattern
[00:18:37]      |
[00:18:37]      |
[00:18:37] 2116 |         (Acquire, Acquire) => intrinsics::atomic_cxchgweak_acq(dst, old, new),
[00:18:37]      |                   ^^^^^^^ used in a pattern more than once
[00:18:37] 
[00:18:37] error[E0416]: identifier `Relaxed` is bound more than once in the same pattern
[00:18:37]      |
[00:18:37]      |
[00:18:37] 2119 |         (Relaxed, Relaxed) => intrinsics::atomic_cxchgweak_relaxed(dst, old, new),
[00:18:37]      |                   ^^^^^^^ used in a pattern more than once
[00:18:37] 
[00:18:37] error[E0416]: identifier `SeqCst` is bound more than once in the same pattern
[00:18:37]      |
[00:18:37]      |
[00:18:37] 2120 |         (SeqCst, SeqCst) => intrinsics::atomic_cxchgweak(dst, old, new),
[00:18:37]      |                  ^^^^^^ used in a pattern more than once
[00:18:37] error: aborting due to 8 previous errors
[00:18:37] 
[00:18:37] Some errors occurred: E0416, E0432.
[00:18:37] For more information about an error, try `rustc --explain E0416`.
[00:18:37] For more information about an error, try `rustc --explain E0416`.
[00:18:37] error: Could not compile `core`.
[00:18:37] warning: build failed, waiting for other jobs to finish...
[00:18:38] error: build failed
[00:18:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:18:38] expected success, got: exit code: 101
[00:18:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:38] Build completed unsuccessfully in 0:15:32
[00:18:38] Makefile:28: recipe for target 'all' failed
[00:18:38] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04a839fb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 26 16:40:49 UTC 2018
