plain
travis_time:end:113b1a20:start=1542735617475246872,finish=1542735672224375716,duration=54749128844
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:01]   Downloaded quote v0.6.8
[00:02:01]   Downloaded ordermap v0.3.5
[00:02:01]    Compiling proc-macro2 v0.4.13
[00:02:01]    Compiling unicode-xid v0.1.0
[00:02:01]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:02:01]    Compiling serde v1.0.75
[00:02:01]    Compiling itoa v0.4.3
[00:02:02]    Compiling ordermap v0.3.5
[00:02:02]    Compiling cc v1.0.25
---
tidy check
[00:03:51] * 568 error codes
[00:03:51] * highest error code: E0721
[00:03:52] * 239 features
[00:03:52] invalid source: "git+https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9eed37338be41ef455cb1c43b49a4b786f"
[00:03:52] invalid source: "git+https://github.com/alexcrichton/dlmalloc-rs?branch=rustc-test#28ee12db813a3b650a7c25d1c36d2c17dcb88ae3"
[00:03:52] invalid source: "git+https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db29c691242c394f45f71eb55ac28a9057"

[00:03:52] travis_time:end:tidy:start=1542735912563234351,finish=1542735914104495051,duration=1541260700

[00:03:52] Build completed successfully in 0:00:45
---
[00:03:54] travis_fold:start:stage0-std
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:03:54]    Compiling cc v1.0.25
[00:03:54]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:03:54]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:57]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:57]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:01]    Compiling compiler_builtins v0.1.0 (https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9e)
[00:04:01]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:04:05]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:05]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:05]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:05]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:05]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:18]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:20]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:25]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:42]     Finished release [optimized] target(s) in 48.45s
[00:04:42] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
[00:04:55] travis_fold:start:stage0-rustc
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:04:56]    Compiling version_check v0.1.5
[00:04:56]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:04:56]    Compiling nodrop v0.1.12
[00:04:56]    Compiling void v1.0.2
[00:04:56]    Compiling memoffset v0.2.1
[00:04:56]    Compiling scopeguard v0.3.3
---

[00:18:12] travis_time:end:stage0-rustc:start=1542735976922803667,finish=1542736773729254016,duration=796806450349

[00:18:12] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:18:12]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:18:12]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:18:16]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:18:16]    Compiling rustc-demangle v0.1.9
[00:18:21]    Compiling num_cpus v1.8.0
---
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:19:17]    Compiling cc v1.0.25
[00:19:17]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:19:17]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:19:21]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:21]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:24]    Compiling compiler_builtins v0.1.0 (https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9e)
[00:19:24]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:19:27]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:19:27]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:19:28]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:19:28]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:19:28]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:20:00]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:20:02]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:20:09]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:20:29]     Finished release [optimized] target(s) in 1m 12s
[00:20:29] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:20:44]    Compiling version_check v0.1.5
[00:20:44]    Compiling nodrop v0.1.12
[00:20:44]    Compiling cfg-if v0.1.5
[00:20:44]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:20:44]    Compiling scopeguard v0.3.3
[00:20:44]    Compiling void v1.0.2
[00:20:44]    Compiling rustc-rayon-core v0.1.1
[00:20:45]    Compiling stable_deref_trait v1.1.0
---

[00:36:16] travis_time:end:stage1-rustc:start=1542736925340611536,finish=1542737857523448719,duration=932182837183

[00:36:16] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:36:16]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:36:16]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:36:16]    Compiling rustc-demangle v0.1.9
[00:36:18]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:36:24]    Compiling memmap v0.6.2
---
[00:37:30] Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:37:30] travis_fold:start:stage2-rustdoc
travis_time:start:stage2-rustdoc
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
[00:37:30]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:37:30]    Compiling rand_core v0.2.1
[00:37:30]    Compiling stable_deref_trait v1.1.0
[00:37:30]    Compiling scopeguard v0.3.3
[00:37:30]    Compiling pulldown-cmark v0.1.2
---

[00:39:30] travis_fold:start:stage0-rustbook
travis_time:start:stage0-rustbook
Building stage0 tool rustbook (x86_64-unknown-linux-gnu)
[00:39:30]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:39:30]    Compiling string_cache_shared v0.3.0
[00:39:30]    Compiling void v1.0.2
[00:39:30]    Compiling cc v1.0.25
[00:39:30]    Compiling version_check v0.1.5
---
[00:43:14] Documenting book index (x86_64-unknown-linux-gnu)
[00:43:14] Documenting book redirect pages (x86_64-unknown-linux-gnu)
[00:43:15] Documenting stage2 std (x86_64-unknown-linux-gnu)
[00:43:15]     Checking core v0.0.0 (/checkout/src/libcore)
[00:43:48]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:43:48]     Checking compiler_builtins v0.1.0 (https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9e)
[00:43:54]     Finished release [optimized] target(s) in 38.88s
[00:43:54]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:44:12] warning: `[chunks]` cannot be resolved, ignoring it...
[00:44:12]    --> libcore/slice/mod.rs:860:52
---
[00:44:12]     |
[00:44:12]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:44:12] 
[00:44:15]     Finished release [optimized] target(s) in 21.66s
[00:44:16]     Checking libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:44:16]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[00:44:17]     Checking unwind v0.0.0 (/checkout/src/libunwind)
[00:44:17]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:44:21]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
---
[00:44:38]     Checking log v0.4.5
[00:44:38]     Checking unreachable v1.0.0
[00:44:38]     Checking arrayvec v0.4.7
[00:44:38]     Checking owning_ref v0.3.3
[00:44:38]     Checking libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[00:44:38]     Checking smallvec v0.6.5
[00:44:38]     Checking lazy_static v1.1.0
[00:44:38]     Checking rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[00:44:38]     Checking ena v0.9.3
---
tidy check
[00:45:19] * 568 error codes
[00:45:19] * highest error code: E0721
[00:45:20] * 239 features
[00:45:20] invalid source: "git+https://github.com/alexcrichton/compiler-builtins?branch=rustc-test#1de78e9eed37338be41ef455cb1c43b49a4b786f"
[00:45:20] invalid source: "git+https://github.com/alexcrichton/dlmalloc-rs?branch=rustc-test#28ee12db813a3b650a7c25d1c36d2c17dcb88ae3"
[00:45:20] invalid source: "git+https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db29c691242c394f45f71eb55ac28a9057"

[00:45:20] travis_time:end:tidy:start=1542738400457607969,finish=1542738402145331449,duration=1687723480

[00:45:20] travis_fold:start:stage0-std
---
[00:48:52] .................................................................................................... 100/5041
[00:48:55] .................................................................................................... 200/5041
[00:48:58] .............................ii............................................ii...................ii.. 300/5041
[00:49:00] ..............................................................................................iii... 400/5041
[00:49:03] .....iiiiiiii.iii............................iii...........................................i........ 500/5041
[00:49:10] .................................................................................................... 700/5041
[00:49:16] ..................................................................................i...........i..... 800/5041
[00:49:19] .................................................................................................... 900/5041
[00:49:23] .iiiii..................ii.iiii..................................................................... 1000/5041
---
[00:49:57] .................................................................................................... 2200/5041
[00:50:01] .................................................................................................... 2300/5041
[00:50:04] .................................................................................................... 2400/5041
[00:50:08] .................................................................................................... 2500/5041
[00:50:11] .....................................................................................iiiiiiiii...... 2600/5041
[00:50:18] ...................................................ii............................................... 2800/5041
[00:50:21] .................................................................................................... 2900/5041
[00:50:24] .................................................................................................... 3000/5041
[00:50:28] ..............................................i..................................................... 3100/5041
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:55] 
[01:03:55] running 117 tests
[01:03:58] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:03:58] i.i.....iiii.....
[01:03:58] 
[01:03:58]  finished in 3.277
[01:03:58] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:12] 
[01:04:12] running 118 tests
[01:04:35] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:04:38] ......iii.i.....ii
[01:04:38] 
[01:04:38]  finished in 26.723
[01:04:38] travis_fold:end:test_debuginfo

---

[01:14:12] travis_fold:start:test_stage1-alloc
travis_time:start:test_stage1-alloc
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:12]    Compiling libc v0.2.43 (https://github.com/alexcrichton/libc?branch=rustc-test#5e44d7db)
[01:14:13] error[E0259]: the name `core` is defined multiple times
[01:14:13] error[E0259]: the name `core` is defined multiple times
[01:14:13]    --> /cargo/git/checkouts/libc-ada9724775c1b512/5e44d7d/src/lib.rs:105:1
[01:14:13] 102 | extern crate std as core;
[01:14:13] 102 | extern crate std as core;
[01:14:13]     | ------------------------- previous import of the extern crate `core` here
[01:14:13] ...
[01:14:13] 105 | extern crate rustc_std_workspace_core as core;
[01:14:13]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `core` reimported here
[01:14:13]     |
[01:14:13]     = note: `core` must be defined only once in the type namespace of this module
[01:14:13] help: you can use `as` to change the binding name of the import
[01:14:13]     |
[01:14:13] 105 | extern crate rustc_std_workspace_core as other_core;
[01:14:13] 
[01:14:14] error: aborting due to previous error
[01:14:14] 
[01:14:14] For more information about this error, try `rustc --explain E0259`.
[01:14:14] For more information about this error, try `rustc --explain E0259`.
[01:14:14] error: Could not compile `libc`.
[01:14:14] 
[01:14:14] To learn more, run the command again with --verbose.
[01:14:14] 
[01:14:14] 
[01:14:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:14:14] 
[01:14:14] 
[01:14:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:14] Build completed unsuccessfully in 0:28:56
[01:14:14] Build completed unsuccessfully in 0:28:56
[01:14:14] Makefile:58: recipe for target 'check' failed
[01:14:14] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:005f9bf1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 20 18:55:35 UTC 2018
