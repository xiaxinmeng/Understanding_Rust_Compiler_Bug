plain
travis_time:end:007dbd91:start=1542846733992343108,finish=1542846736204521992,duration=2212178884
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:27]    Compiling proc-macro2 v0.4.24
[00:02:27]    Compiling unicode-xid v0.1.0
[00:02:27]    Compiling ryu v0.2.6
[00:02:27]    Compiling serde v1.0.75
[00:02:28]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:02:28]    Compiling cfg-if v0.1.5
[00:02:28]    Compiling ordermap v0.3.5
[00:02:28]    Compiling itoa v0.4.3
[00:02:29]    Compiling fixedbitset v0.1.9
---
tidy check
[00:04:27] * 568 error codes
[00:04:27] * highest error code: E0721
[00:04:27] * 239 features
[00:04:28] invalid source: "git+https://github.com/rust-lang/libc?branch=auto-libc#03310f8ad668eb66b21db91ccb0b93e3ac4c5207"
[00:04:28] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:04:28] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:04:28] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:04:28] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:04:28] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:04:28] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:04:28] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"

[00:04:28] travis_time:end:tidy:start=1542847012038615301,finish=1542847013601155052,duration=1562539751

[00:04:28] Build completed successfully in 0:00:55
---
[00:04:29] travis_fold:start:stage0-std
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:04:30]    Compiling cc v1.0.25
[00:04:30]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:04:30]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:31]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:36]    Compiling compiler_builtins v0.1.1
[00:04:36]    Compiling cmake v0.1.33
[00:04:36]    Compiling cmake v0.1.33
[00:04:36]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:04:40]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:40]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:41]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:41]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:54]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:56]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:05:01]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:18]     Finished release [optimized] target(s) in 48.77s
[00:05:18] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:05:32]    Compiling version_check v0.1.5
[00:05:32]    Compiling cfg-if v0.1.5
[00:05:32]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:05:32]    Compiling scopeguard v0.3.3
[00:05:32]    Compiling memoffset v0.2.1
[00:05:32]    Compiling void v1.0.2
[00:05:32]    Compiling rustc-rayon-core v0.1.1
---

[00:19:04] travis_time:end:stage0-rustc:start=1542847077035982258,finish=1542847889826018614,duration=812790036356

[00:19:04] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:19:04]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:19:04]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:19:06]    Compiling rustc-demangle v0.1.9
[00:19:09]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:19:11]    Compiling num_cpus v1.8.0
---
[00:20:09] travis_fold:start:stage1-std
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:20:09]    Compiling cc v1.0.25
[00:20:09]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:20:09]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:20:11]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:20:17]    Compiling compiler_builtins v0.1.1
[00:20:17]    Compiling cmake v0.1.33
[00:20:17]    Compiling cmake v0.1.33
[00:20:17]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:20:20]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:20:20]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:20:21]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:20:21]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:20:52]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:20:55]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:21:01]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:21:22]     Finished release [optimized] target(s) in 1m 12s
[00:21:22] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:21:37]    Compiling version_check v0.1.5
[00:21:37]    Compiling cfg-if v0.1.5
[00:21:37]    Compiling nodrop v0.1.12
[00:21:37]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:21:37]    Compiling void v1.0.2
[00:21:37]    Compiling memoffset v0.2.1
[00:21:37]    Compiling rustc-rayon-core v0.1.1
[00:21:37]    Compiling stable_deref_trait v1.1.0
---

[00:37:16] travis_time:end:stage1-rustc:start=1542848042537003924,finish=1542848981488186752,duration=938951182828

[00:37:16] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:37:16]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:37:16]    Compiling cc v1.0.25
[00:37:18]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:37:24]    Compiling rustc-demangle v0.1.9
[00:37:24]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
---
[00:38:32] Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:38:32] travis_fold:start:stage2-rustdoc
travis_time:start:stage2-rustdoc
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
[00:38:32]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:38:32]    Compiling stable_deref_trait v1.1.0
[00:38:32]    Compiling rand_core v0.2.1
[00:38:32]    Compiling pulldown-cmark v0.1.2
[00:38:32]    Compiling scopeguard v0.3.3
---

[00:40:35] travis_fold:start:stage0-rustbook
travis_time:start:stage0-rustbook
Building stage0 tool rustbook (x86_64-unknown-linux-gnu)
[00:40:35]    Compiling libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:40:35]    Compiling void v1.0.2
[00:40:35]    Compiling cc v1.0.25
[00:40:36]    Compiling string_cache_shared v0.3.0
[00:40:36]    Compiling version_check v0.1.5
---

[00:44:25] Documenting book redirect pages (x86_64-unknown-linux-gnu)
[00:44:27] Documenting stage2 std (x86_64-unknown-linux-gnu)
[00:44:27]     Checking core v0.0.0 (/checkout/src/libcore)
[00:45:00]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:45:02]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:45:06]     Finished release [optimized] target(s) in 39.36s
[00:45:06]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:45:24] warning: `[chunks]` cannot be resolved, ignoring it...
---
[00:45:24]     |
[00:45:24]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:45:24] 
[00:45:27]     Finished release [optimized] target(s) in 21.42s
[00:45:28]     Checking libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:45:28]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[00:45:29]     Checking unwind v0.0.0 (/checkout/src/libunwind)
[00:45:29]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:45:33]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
---
[00:45:50]     Checking crossbeam-utils v0.2.2
[00:45:50]     Checking log v0.4.5
[00:45:50]     Checking unreachable v1.0.0
[00:45:50]     Checking owning_ref v0.3.3
[00:45:50]     Checking libc v0.2.44 (https://github.com/rust-lang/libc?branch=auto-libc#03310f8a)
[00:45:51]     Checking smallvec v0.6.5
[00:45:51]     Checking lazy_static v1.1.0
[00:45:51]     Checking rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[00:45:51]     Checking ena v0.10.1
---
tidy check
[00:46:34] * 568 error codes
[00:46:34] * highest error code: E0721
[00:46:34] * 239 features
[00:46:35] invalid source: "git+https://github.com/rust-lang/libc?branch=auto-libc#03310f8ad668eb66b21db91ccb0b93e3ac4c5207"
[00:46:35] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:46:35] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:46:35] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:46:35] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:46:35] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:46:35] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"
[00:46:35] invalid source: "git+https://github.com/rust-random/rand#3eadab75c8a5871d1be729091795a6c4e1dc19bb"

[00:46:35] travis_time:end:tidy:start=1542849538763896422,finish=1542849540464977507,duration=1701081085

[00:46:35] travis_fold:start:stage0-std
---
[00:50:22] .................................................................................................... 100/5043
[00:50:25] .................................................................................................... 200/5043
[00:50:27] .............................ii............................................ii...................ii.. 300/5043
[00:50:30] ..............................................................................................iii... 400/5043
[00:50:33] .....iiiiiiii.iii............................iii...........................................i........ 500/5043
[00:50:40] .................................................................................................... 700/5043
[00:50:46] ..................................................................................i...........i..... 800/5043
[00:50:49] .................................................................................................... 900/5043
[00:50:52] .iiiii..................ii.iiii..................................................................... 1000/5043
---
[00:51:27] .................................................................................................... 2200/5043
[00:51:31] .................................................................................................... 2300/5043
[00:51:34] .................................................................................................... 2400/5043
[00:51:38] .................................................................................................... 2500/5043
[00:51:41] ......................................................................................iiiiiiiii..... 2600/5043
[00:51:48] ....................................................ii.............................................. 2800/5043
[00:51:50] .................................................................................................... 2900/5043
[00:51:54] .................................................................................................... 3000/5043
[00:51:57] ................................................i................................................... 3100/5043
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:34] 
[01:05:34] running 117 tests
[01:05:37] i..ii...iii..iiii.....i...i.........i..iii...........i......i....ii...i..i.ii..............i...ii..i 100/117
[01:05:38] i.i.....iiii.....
[01:05:38] 
[01:05:38]  finished in 3.376
[01:05:38] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:52] 
[01:05:52] running 118 tests
[01:06:15] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:06:19] ......iii.i.....ii
[01:06:19] 
[01:06:19]  finished in 27.394
[01:06:19] travis_fold:end:test_debuginfo

---
[01:16:00] travis_fold:start:test_stage1-alloc
travis_time:start:test_stage1-alloc
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:01]    Compiling semver-parser v0.7.0
[01:16:01]    Compiling rand_core v0.3.0 (https://github.com/rust-random/rand#3eadab75)
[01:16:01]    Compiling rand_hc v0.1.0 (https://github.com/rust-random/rand#3eadab75)
[01:16:01]    Compiling rand_isaac v0.1.0 (https://github.com/rust-random/rand#3eadab75)
[01:16:01]    Compiling rand_xorshift v0.1.0 (https://github.com/rust-random/rand#3eadab75)
[01:16:02]    Compiling semver v0.9.0
[01:16:04]    Compiling rand_chacha v0.1.0 (https://github.com/rust-random/rand#3eadab75)
[01:16:04]    Compiling rand_pcg v0.1.1 (https://github.com/rust-random/rand#3eadab75)
[01:16:04]    Compiling rand v0.6.0 (https://github.com/rust-random/rand#3eadab75)
onst GRND_NONBLOCK: libc::c_uint = 0x0001;
[01:16:05]     |
[01:16:05]     |
[01:16:05]     = help: add #![feature(libc)] to the crate attributes to enable
[01:16:05] 
[01:16:05] error[E0658]: use of unstable library feature 'libc': use `libc` from crates.io (see issue #27783)
[01:16:05]    --> /cargo/git/checkouts/rand-cc8f5b7ec2d3b6d9/3eadab7/src/rngs/os.rs:478:29
[01:16:05]     |
[01:16:05] 478 |                 err != Some(libc::ENOSYS)
[01:16:05]     |
[01:16:05]     |
[01:16:05]     = help: add #![feature(libc)] to the crate attributes to enable
[01:16:05] 
[01:16:05] error[E0658]: use of unstable library feature 'libc': use `libc` from crates.io (see issue #27783)
[01:16:05]    --> /cargo/git/checkouts/rand-cc8f5b7ec2d3b6d9/3eadab7/src/rngs/os.rs:376:64
[01:16:05]     |
[01:16:05] 376 |                         .custom_flags(if blocking { 0 } else { libc::O_NONBLOCK })
