plain
[00:41:03]     Checking core v0.0.0 (file:///checkout/src/libcore)
[00:41:03]  Documenting core v0.0.0 (file:///checkout/src/libcore)
[00:41:03]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:41:46] warning: [1] cannot be resolved, ignoring it...
[00:41:46]   --> libcore/num/flt2dec/strategy/dragon.rs:11:1
[00:41:46] 11 | / /*!
[00:41:46] 11 | / /*!
[00:41:46] 12 | | Almost direct (but slightly optimized) Rust translation of Figure 3 of [1].
[00:41:46] 13 | |
[00:41:46] 14 | | [1] Burger, R. G. and Dybvig, R. K. 1996. Printing floating-point numbers
[00:41:46] 15 | |     quickly and accurately. SIGPLAN Not. 31, 5 (May. 1996), 108-116.
[00:41:46] 16 | | */
[00:41:46]    | |__^
[00:41:46] warning: [1] cannot be resolved, ignoring it...
[00:41:46] warning: [1] cannot be resolved, ignoring it...
[00:41:46]   --> libcore/num/flt2dec/strategy/grisu.rs:11:1
[00:41:46] 11 | / /*!
[00:41:46] 11 | / /*!
[00:41:46] 12 | | Rust adaptation of Grisu3 algorithm described in [1]. It uses about
[00:41:46] 13 | | 1KB of precomputed table, and in turn, it's very quick for most inputs.
[00:41:46] 14 | |
[00:41:46] 15 | | [1] Florian Loitsch. 2010. Printing floating-point numbers quickly and
[00:41:46] 16 | |     accurately with integers. SIGPLAN Not. 45, 6 (June 2010), 233-243.
[00:41:46] 17 | | */
[00:41:46]    | |__^
[00:41:47] warning: [x] cannot be resolved, ignoring it...
[00:41:47] warning: [x] cannot be resolved, ignoring it...
[00:41:47]  --> libcore/../stdsimd/coresimd/ppsv/api/mod.rs:1:1
[00:41:47]   |
[00:41:47] 1 | //! This module defines the API of portable vector types.
[00:41:47] 
[00:41:47] warning: [] cannot be resolved, ignoring it...
[00:41:47] warning: [] cannot be resolved, ignoring it...
[00:41:47]  --> libcore/../stdsimd/coresimd/ppsv/api/mod.rs:1:1
[00:41:47]   |
[00:41:47] 1 | //! This module defines the API of portable vector types.
[00:41:47] 
[00:41:47] warning: [arm_dat] cannot be resolved, ignoring it...
[00:41:47]  --> libcore/../stdsimd/coresimd/aarch64/mod.rs:1:1
[00:41:47]   |
[00:41:47]   |
[00:41:47] 1 | //! AArch64 intrinsics.
[00:41:47] 
[00:41:47] warning: [arm_dat] cannot be resolved, ignoring it...
[00:41:47]  --> libcore/../stdsimd/coresimd/arm/mod.rs:1:1
[00:41:47]   |
[00:41:47]   |
[00:41:47] 1 | //! ARM intrinsics.
[00:41:47] 
[00:41:59]     Checking compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:42:01]     Checking libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:42:01]     Checking alloc v0.0.0 (file:///checkout/src/liballoc)
---
[00:42:37]     Checking rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:42:39]     Checking syntax v0.0.0 (file:///checkout/src/libsyntax)
[00:42:55]  Documenting proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:42:56] warning: [cfg] cannot be resolved, ignoring it...
[00:42:56]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-0.1.2/src/lib.rs:1:1
[00:42:56]   |
[00:42:56] 1 | //! A macro for defining #[cfg] if-else statements.
[00:42:56] 
[00:42:56] warning: [rayon::prelude] cannot be resolved, ignoring it...
[00:42:56]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.1.0/src/lib.rs:7:1
[00:42:56]   |
[00:42:56]   |
[00:42:56] 7 | //! Data-parallelism library that makes it easy to convert sequential
[00:42:56] 
[00:42:56] warning: [Experimental] cannot be resolved, ignoring it...
[00:42:56] warning: [Experimental] cannot be resolved, ignoring it...
[00:42:56]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot-0.5.5/src/deadlock.rs:1:1
[00:42:56]   |
[00:42:56] 1 | //! [Experimental] Deadlock detection
[00:42:56] 
[00:42:56] warning: [Experimental] cannot be resolved, ignoring it...
[00:42:56] warning: [Experimental] cannot be resolved, ignoring it...
[00:42:56]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.2.14/src/parking_lot.rs:1071:1
[00:42:56]      |
[00:42:56] 1071 | /// [Experimental] Deadlock detection
[00:42:56] 
[00:42:56] warning: [plumbing] cannot be resolved, ignoring it...
[00:42:56]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.1.0/src/iter/mod.rs:1:1
[00:42:56]   |
[00:42:56]   |
[00:42:56] 1 | //! Traits for writing parallel programs using an iterator-style interface
[00:42:56] 
[00:42:56] warning: [Garbage] cannot be resolved, ignoring it...
[00:42:56]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-epoch-0.3.1/src/lib.rs:1:1
[00:42:56]   |
[00:42:56]   |
[00:42:56] 1 | //! Epoch-based memory reclamation.
[00:42:56] 
[00:42:56]     Finished release [optimized] target(s) in 30.88s
[00:42:57] Documenting stage2 compiler (x86_64-unknown-linux-gnu)
[00:42:57]  skipping - compiler docs disabled
---
[00:45:17] ..............................................................i.....................................
[00:45:21] ....................................................................................................
[00:45:26] ....................................................................................................
[00:45:32] ...........................................................................................i........
[00:45:35] .........iiiiiiiii...................................................
[00:45:35] 
[00:45:35] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:21] ..............................................................i.....................................
[00:46:25] ....................................................................................................
[00:46:30] ....................................................................................................
[00:46:36] ...........................................................................................i........
[00:46:38] .........iiiiiiiii...................................................
[00:46:38] 
[00:46:38]  finished in 63.447
[00:46:38] travis_fold:end:test_ui_nll

---
[01:26:04] 
[01:26:04] ---- [ui] rustdoc-ui/intra-links-warning.rs stdout ----
[01:26:04] diff of stderr:
[01:26:04] 
[01:26:04] 1 warning: [Foo::baz] cannot be resolved, ignoring it...
[01:26:04] 3    |
[01:26:04] 3    |
[01:26:04] - LL | //! Test with [Foo::baz], [Bar::foo], [Uniooon::X]
[01:26:04] + 13 | //! Test with [Foo::baz], [Bar::foo], [Uniooon::X]
[01:26:04] 6 
[01:26:04] 6 
[01:26:04] 7 warning: [Bar::foo] cannot be resolved, ignoring it...
[01:26:04] 8   --> $DIR/intra-links-warning.rs:13:1
[01:26:04] 9    |
[01:26:04] 9    |
[01:26:04] - LL | //! Test with [Foo::baz], [Bar::foo], [Uniooon::X]
[01:26:04] + 13 | //! Test with [Foo::baz], [Bar::foo], [Uniooon::X]
[01:26:04] 12 
[01:26:04] 12 
[01:26:04] 13 warning: [Uniooon::X] cannot be resolved, ignoring it...
[01:26:04] 14   --> $DIR/intra-links-warning.rs:13:1
[01:26:04] 15    |
[01:26:04] 15    |
[01:26:04] - LL | //! Test with [Foo::baz], [Bar::foo], [Uniooon::X]
[01:26:04] + 13 | //! Test with [Foo::baz], [Bar::foo], [Uniooon::X]
[01:26:04] 18 
[01:26:04] 19 
[01:26:04] 
[01:26:04] 
---
[01:26:04] test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:26:04] 
[01:26:04] 
[01:26:04] 
[01:26:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0088127b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
