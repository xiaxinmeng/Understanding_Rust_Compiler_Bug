plain
travis_time:end:0514d240:start=1542695844332022153,finish=1542695901405956600,duration=57073934447
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:01:59]    Compiling proc-macro2 v0.4.13
[00:01:59]    Compiling unicode-xid v0.1.0
[00:01:59]    Compiling ryu v0.2.6
[00:01:59]    Compiling serde v1.0.75
[00:01:59]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:01:59]    Compiling cc v1.0.25
[00:01:59]    Compiling ordermap v0.3.5
[00:02:00]    Compiling cfg-if v0.1.5
[00:02:00]    Compiling itoa v0.4.3
---
tidy check
[00:03:45] * 568 error codes
[00:03:45] * highest error code: E0721
[00:03:46] * 239 features
[00:03:46] invalid source: "git+https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9eed37338be41ef455cb1c43b49a4b786f"
[00:03:46] invalid source: "git+https://github.com/alexcrichton/dlmalloc-rs?branch=rustc-test#28ee12db813a3b650a7c25d1c36d2c17dcb88ae3"
[00:03:46] invalid source: "git+https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730f2b3939eaef106baa9e63449fc64da5e"

[00:03:46] travis_time:end:tidy:start=1542696135120865914,finish=1542696136742693862,duration=1621827948

[00:03:46] Build completed successfully in 0:00:47
---
[00:03:48] travis_fold:start:stage0-std
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:03:48]    Compiling cc v1.0.25
[00:03:48]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:03:48]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:49]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:49]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:55]    Compiling compiler_builtins v0.1.0 (https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9e)
[00:03:55]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:59]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:59]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:59]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:59]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:59]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:13]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:14]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:19]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:37]     Finished release [optimized] target(s) in 49.20s
[00:04:37] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:04:51]    Compiling version_check v0.1.5
[00:04:51]    Compiling cfg-if v0.1.5
[00:04:51]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:04:51]    Compiling scopeguard v0.3.3
[00:04:51]    Compiling void v1.0.2
[00:04:51]    Compiling memoffset v0.2.1
[00:04:51]    Compiling rand_core v0.2.1
---

[00:18:28] travis_time:end:stage0-rustc:start=1542696200952004900,finish=1542697018716184898,duration=817764179998

[00:18:28] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:18:29]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:18:29]    Compiling cc v1.0.25
[00:18:29]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:18:35]    Compiling rustc-demangle v0.1.9
[00:18:36]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
---
[00:19:34] travis_fold:start:stage1-std
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:19:34]    Compiling cc v1.0.25
[00:19:34]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:19:34]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:19:36]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:36]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:41]    Compiling compiler_builtins v0.1.0 (https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9e)
[00:19:41]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:19:44]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:19:44]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:19:45]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:19:45]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:19:45]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:20:19]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:20:21]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:20:28]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:20:49]     Finished release [optimized] target(s) in 1m 15s
[00:20:49] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
travis_time:start:stage1-rustc
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:21:05]    Compiling version_check v0.1.5
[00:21:05]    Compiling cfg-if v0.1.5
[00:21:05]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:21:05]    Compiling memoffset v0.2.1
[00:21:05]    Compiling void v1.0.2
[00:21:05]    Compiling scopeguard v0.3.3
[00:21:05]    Compiling rustc-rayon-core v0.1.1
---

[00:36:54] travis_time:end:stage1-rustc:start=1542697174665881242,finish=1542698124636002080,duration=949970120838

[00:36:54] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:36:55]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:36:55]    Compiling cc v1.0.25
[00:36:55]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:36:55]    Compiling rustc-demangle v0.1.9
[00:37:03]    Compiling num_cpus v1.8.0
---
[00:38:10] Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:38:10] travis_fold:start:stage2-rustdoc
travis_time:start:stage2-rustdoc
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
[00:38:10]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:38:10]    Compiling rand_core v0.2.1
[00:38:10]    Compiling stable_deref_trait v1.1.0
[00:38:11]    Compiling scopeguard v0.3.3
[00:38:11]    Compiling pulldown-cmark v0.1.2
---

[00:40:13] travis_fold:start:stage0-rustbook
travis_time:start:stage0-rustbook
Building stage0 tool rustbook (x86_64-unknown-linux-gnu)
[00:40:13]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:40:13]    Compiling string_cache_shared v0.3.0
[00:40:13]    Compiling cc v1.0.25
[00:40:13]    Compiling version_check v0.1.5
[00:40:14]    Compiling void v1.0.2
---
[00:44:03] Documenting book index (x86_64-unknown-linux-gnu)
[00:44:03] Documenting book redirect pages (x86_64-unknown-linux-gnu)
[00:44:04] Documenting stage2 std (x86_64-unknown-linux-gnu)
[00:44:04]     Checking core v0.0.0 (/checkout/src/libcore)
[00:44:39]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:44:39]     Checking compiler_builtins v0.1.0 (https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9e)
[00:44:45]     Finished release [optimized] target(s) in 41.08s
[00:44:45]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:45:04] warning: `[chunks]` cannot be resolved, ignoring it...
[00:45:04]    --> libcore/slice/mod.rs:860:52
---
[00:45:04]     |
[00:45:04]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:04] 
[00:45:07]     Finished release [optimized] target(s) in 22.54s
[00:45:08]     Checking libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:45:08]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[00:45:09]     Checking unwind v0.0.0 (/checkout/src/libunwind)
[00:45:09]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:45:13]     Checking rustc_msan v0.0.0 (/checkout/src/librustc_msan)
---
[00:45:31]     Checking log v0.4.5
[00:45:31]     Checking arrayvec v0.4.7
[00:45:32]     Checking unreachable v1.0.0
[00:45:32]     Checking owning_ref v0.3.3
[00:45:32]     Checking libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730)
[00:45:32]     Checking rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[00:45:32]     Checking rustc-hash v1.0.1
[00:45:32]     Checking lazy_static v1.1.0
[00:45:32]     Checking smallvec v0.6.5
---
tidy check
[00:46:14] * 568 error codes
[00:46:14] * highest error code: E0721
[00:46:15] * 239 features
[00:46:15] invalid source: "git+https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9eed37338be41ef455cb1c43b49a4b786f"
[00:46:15] invalid source: "git+https://github.com/alexcrichton/dlmalloc-rs?branch=rustc-test#28ee12db813a3b650a7c25d1c36d2c17dcb88ae3"
[00:46:15] invalid source: "git+https://github.com/alexcrichton/libc?branch=rustc-test#d3aa5730f2b3939eaef106baa9e63449fc64da5e"

[00:46:15] travis_time:end:tidy:start=1542698683887190144,finish=1542698685666461457,duration=1779271313

[00:46:15] travis_fold:start:stage0-std
---
[00:47:39] ..................................................................................i...........i..... 800/5041
[00:47:42] .................................................................................................... 900/5041
[00:47:46] .iiiii.............................................................................................. 1000/5041
[00:47:48] .................................................................................................... 1100/5041
[00:47:50] .................F.................................................................................. 1200/5041
[00:47:53] .................................................................................................... 1300/5041
[00:47:55] .......F............................................................................................ 1400/5041
[00:48:00] ..i.....................................................................i........................... 1600/5041
[00:48:04] .................................................................................................... 1700/5041
[00:48:07] .................................................................................................... 1800/5041
[00:48:07] .................................................................................................... 1800/5041
[00:48:11] ............F....................................................................................... 1900/5041
[00:48:14] ....i............................................................................................... 2000/5041
[00:48:17] ..............................................................................................F..... 2100/5041
[00:48:26] .................................................................................................... 2300/5041
[00:48:29] .................................................................................................... 2400/5041
[00:48:29] .................................................................................................... 2400/5041
[00:48:32] .....................................................F.............................................. 2500/5041
[00:48:40] .................................................................................................... 2700/5041
[00:48:43] .................................................................................................... 2800/5041
[00:48:46] .................................................................................................... 2900/5041
[00:48:46] .................................................................................................... 2900/5041
[00:48:50] .......F............................................................................................ 3000/5041
[00:48:57] .................................................................................................... 3200/5041
[00:48:59] .........ii..i..ii.................................................................................. 3300/5041
[00:49:03] .................................................................................................... 3400/5041
[00:49:03] .................................................................................................... 3400/5041
[00:49:07] ........................................................................................F.ii........ 3500/5041
[00:49:11] .........i.......................................................................................... 3700/5041
[00:49:12] ................................................................i................................... 3800/5041
[00:49:14] .................................................................................................... 3900/5041
[00:49:17] .................................................................................................... 4000/5041
