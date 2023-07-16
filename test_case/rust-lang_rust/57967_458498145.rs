plain
travis_time:end:2f13cf16:start=1548753095205365817,finish=1548753098023581145,duration=2818215328
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:05] 
######################################################################## 100.0%
[00:02:06] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:06]     Updating crates.io index
[00:02:16]     Updating git repository `https://github.com/eddyb/rustc-demangle`
[00:02:17]     Updating git repository `https://github.com/michaelwoerister/std-mangle-rs`
[00:02:19]   Downloaded serde_derive v1.0.81
[00:02:19]   Downloaded petgraph v0.4.13
[00:02:19]   Downloaded toml v0.4.10
[00:02:19]   Downloaded time v0.1.40
---
[00:03:15]   Downloaded markup5ever v0.7.2
[00:03:15]   Downloaded failure v0.1.3
[00:03:15]   Downloaded rand v0.5.5
[00:03:15]   Downloaded bitflags v0.9.1
[00:03:15]   Downloaded punycode v0.4.0
[00:03:15]   Downloaded derive_more v0.13.0
[00:03:15]   Downloaded rand_isaac v0.1.1
[00:03:15]   Downloaded termion v1.5.1
[00:03:15]   Downloaded pretty_assertions v0.5.1
---
[00:03:17]   Downloaded arc-swap v0.3.7
[00:03:17]   Downloaded miniz-sys v0.1.11
[00:03:17]   Downloaded num-integer v0.1.39
[00:03:17]   Downloaded unicode-normalization v0.1.7
[00:03:17]   Downloaded unic-idna-punycode v0.7.0
[00:03:17]   Downloaded core-foundation v0.6.3
[00:03:17]   Downloaded mdbook v0.1.7
[00:03:17]   Downloaded ena v0.11.0
[00:03:17]   Downloaded libnghttp2-sys v0.1.1
---
[00:04:26]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:27]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:49]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:51]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:51]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:04:56]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:13]     Finished release [optimized] target(s) in 58.05s
[00:05:13] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:05:13] travis_fold:end:stage0-std
---
[00:05:40]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[00:05:42]    Compiling remove_dir_all v0.5.1
[00:05:42]    Compiling rustc-demangle v0.1.10
[00:05:42]    Compiling rustc_incremental v0.0.0 (/checkout/src/librustc_incremental)
[00:05:42]    Compiling std-mangle-rs v0.1.0 (https://github.com/michaelwoerister/std-mangle-rs?rev=2336dcdfcc91db3cdda18eda73aca488773ac6fc#2336dcdf)
[00:05:44]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:05:44]    Compiling rustc_metadata v0.0.0 (/checkout/src/librustc_metadata)
[00:05:44]    Compiling rustc-serialize v0.3.24
[00:05:44]    Compiling rustc-serialize v0.3.24
[00:05:44]    Compiling unic-idna-punycode v0.7.0
[00:05:44]    Compiling quick-error v1.2.2
[00:05:45]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:05:45]    Compiling punycode v0.4.0
[00:05:47]    Compiling crossbeam-utils v0.2.2
[00:05:47]    Compiling log v0.4.6
[00:05:47]    Compiling arrayvec v0.4.7
[00:05:48]    Compiling unreachable v1.0.0
---
[00:25:48]    Compiling libc v0.2.46
[00:25:48]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:25:48]    Compiling cc v1.0.28
[00:25:48]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:25:49]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:25:57]    Compiling memmap v0.6.2
[00:25:57]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:27:19]     Finished release [optimized] target(s) in 1m 30s
[00:27:19] travis_fold:start:stage0-rustc_codegen_llvm
---
[00:27:30]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:28:00]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:28:03]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:28:03]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:28:03]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:28:32]     Finished release [optimized] target(s) in 1m 13s
[00:28:32] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:28:32] travis_fold:end:stage1-std

---
[00:29:07]    Compiling remove_dir_all v0.5.1
[00:29:07]    Compiling rustc_fs_util v0.0.0 (/checkout/src/librustc_fs_util)
[00:29:08]    Compiling rustc_metadata v0.0.0 (/checkout/src/librustc_metadata)
[00:29:08]    Compiling rustc_incremental v0.0.0 (/checkout/src/librustc_incremental)
[00:29:09]    Compiling std-mangle-rs v0.1.0 (https://github.com/michaelwoerister/std-mangle-rs?rev=2336dcdfcc91db3cdda18eda73aca488773ac6fc#2336dcdf)
[00:29:09]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:29:09]    Compiling unic-idna-punycode v0.7.0
[00:29:11]    Compiling rustc-serialize v0.3.24
[00:29:11]    Compiling punycode v0.4.0
[00:29:11]    Compiling quick-error v1.2.2
[00:29:11]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:29:13]    Compiling crossbeam-utils v0.2.2
[00:29:14]    Compiling log v0.4.6
[00:29:17]    Compiling arrayvec v0.4.7
[00:29:17]    Compiling log_settings v0.1.2
---
[00:54:21]    Compiling libc v0.2.46
[00:54:21]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:54:21]    Compiling cc v1.0.28
[00:54:21]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:54:23]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:54:32]    Compiling memmap v0.6.2
[00:54:33]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:56:13]     Finished release [optimized] target(s) in 1m 52s
[00:56:13] travis_fold:start:stage1-rustc_codegen_llvm
---
[01:04:26]     Finished release [optimized] target(s) in 22.36s
[01:04:28]     Checking libc v0.2.46
[01:04:28]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:04:28]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[01:04:28]     Checking rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[01:04:29]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[01:04:29]     Checking backtrace-sys v0.1.27
[01:04:33]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[01:04:33]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:48] 
[01:17:48] running 119 tests
[01:18:18] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:18:23] i......iii.i.....ii
[01:18:23] 
[01:18:23]  finished in 35.218
[01:18:23] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:46:06] 
[01:46:06] running 193 tests
[01:46:37] F................................................................................................... 100/193
[01:47:26] ................................................................F............F...............
[01:47:26] 
[01:47:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:47:26] ---- [run-make] run-make-fulldeps/a-b-a-linker-guard stdout ----
[01:47:26] 
[01:47:26] 
[01:47:26] error: make failed
[01:47:26] status: exit code: 2
[01:47:26] command: "make"
[01:47:26] stdout:
[01:47:26] ------------------------------------------
[01:47:26] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/a-b-a-linker-guard'
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard  a.rs --cfg x -C prefer-dynamic
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard  b.rs -C prefer-dynamic
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard/b
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard  a.rs --cfg y -C prefer-dynamic
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard/b && exit 1 || exit 0
[01:47:26] Makefile:8: recipe for target 'all' failed
[01:47:26] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/a-b-a-linker-guard'
[01:47:26] ------------------------------------------
[01:47:26] stderr:
[01:47:26] ------------------------------------------
[01:47:26] warning: unused variable: `x`
[01:47:26] warning: unused variable: `x`
[01:47:26]  --> a.rs:5:12
[01:47:26]   |
[01:47:26] 5 | pub fn foo(x: u32) { }
[01:47:26]   |            ^ help: consider prefixing with an underscore: `_x`
[01:47:26]   = note: #[warn(unused_variables)] on by default
[01:47:26] 
[01:47:26] warning: unused variable: `x`
[01:47:26]  --> a.rs:8:12
[01:47:26]  --> a.rs:8:12
[01:47:26]   |
[01:47:26] 8 | pub fn foo(x: i32) { }
[01:47:26]   |            ^ help: consider prefixing with an underscore: `_x`
[01:47:26]   = note: #[warn(unused_variables)] on by default
[01:47:26] 
[01:47:26] 
[01:47:26] make[1]: *** [all] Error 1
[01:47:26] ------------------------------------------
[01:47:26] 
[01:47:26] thread '[run-make] run-make-fulldeps/a-b-a-linker-guard' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3288:9
[01:47:26] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:47:26] status: exit code: 2
[01:47:26] command: "make"
[01:47:26] stdout:
[01:47:26] ------------------------------------------
[01:47:26] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/stable-symbol-names'
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names  stable-symbol-names1.rs
[01:47:26] nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/libstable_symbol_names1.rlib" | grep -E "generic_|mono_" | sed "s/.*\(_ZN.*E\).*/\1/" | sort > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names1_v1.nm"
[01:47:26] rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/libstable_symbol_names1.rlib
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names  stable-symbol-names1.rs
[01:47:26] nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/libstable_symbol_names1.rlib" | grep -E "generic_|mono_" | sed "s/.*\(_ZN.*E\).*/\1/" | sort > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names1_v2.nm"
[01:47:26] cmp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names1_v1.nm" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names1_v2.nm"
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names  stable-symbol-names2.rs
[01:47:26] nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/libstable_symbol_names2.rlib" | grep -E "generic_|mono_" | sed "s/.*\(_ZN.*E\).*/\1/" | sort > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names2_v1.nm"
[01:47:26] rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/libstable_symbol_names2.rlib
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names  stable-symbol-names2.rs
[01:47:26] nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/libstable_symbol_names2.rlib" | grep -E "generic_|mono_" | sed "s/.*\(_ZN.*E\).*/\1/" | sort > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names2_v2.nm"
[01:47:26] cmp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names2_v1.nm" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names2_v2.nm"
[01:47:26] nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/libstable_symbol_names1.rlib" | grep -E "mono_" | sed "s/.*\(_ZN.*E\).*/\1/" | sort > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names1_cross.nm"
[01:47:26] nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/libstable_symbol_names2.rlib" | grep -E "mono_" | sed "s/.*\(_ZN.*E\).*/\1/" | sort > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names2_cross.nm"
[01:47:26] cmp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names1_cross.nm" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names2_cross.nm"
[01:47:26] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names1_cross.nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/stable-symbol-names/stable-symbol-names/stable_symbol_names2_cross.nm differ: char 1, line 1
[01:47:26] Makefile:24: recipe for target 'all' failed
[01:47:26] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/stable-symbol-names'
[01:47:26] ------------------------------------------
[01:47:26] stderr:
[01:47:26] ------------------------------------------
[01:47:26] ------------------------------------------
[01:47:26] nm: rust.metadata.bin: File format not recognized
[01:47:26] nm: stable-symbol-names1.stable_symbol_names1.3a1fbbbh-cgu.0.rcgu.bc.z: File format not recognized
[01:47:26] nm: rust.metadata.bin: File format not recognized
[01:47:26] nm: stable-symbol-names1.stable_symbol_names1.3a1fbbbh-cgu.0.rcgu.bc.z: File format not recognized
[01:47:26] nm: rust.metadata.bin: File format not recognized
[01:47:26] nm: stable-symbol-names2.stable_symbol_names2.3a1fbbbh-cgu.0.rcgu.bc.z: File format not recognized
[01:47:26] nm: rust.metadata.bin: File format not recognized
[01:47:26] nm: stable-symbol-names2.stable_symbol_names2.3a1fbbbh-cgu.0.rcgu.bc.z: File format not recognized
[01:47:26] nm: rust.metadata.bin: File format not recognized
[01:47:26] nm: stable-symbol-names1.stable_symbol_names1.3a1fbbbh-cgu.0.rcgu.bc.z: File format not recognized
[01:47:26] nm: rust.metadata.bin: File format not recognized
[01:47:26] nm: stable-symbol-names2.stable_symbol_names2.3a1fbbbh-cgu.0.rcgu.bc.z: File format not recognized
[01:47:26] make[1]: *** [all] Error 1
[01:47:26] ------------------------------------------
[01:47:26] 
[01:47:26] thread '[run-make] run-make-fulldeps/stable-symbol-names' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3288:9
[01:47:26] 
---
[01:47:26] command: "make"
[01:47:26] stdout:
[01:47:26] ------------------------------------------
[01:47:26] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/symbol-visibility'
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no an_rlib.rs
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no a_cdylib.rs
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no a_rust_dylib.rs
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no an_executable.rs
[01:47:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility  -Zshare-generics=no a_cdylib.rs --crate-name combined_rlib_dylib --crate-type=rlib,cdylib
[01:47:26] # Check that a cdylib exports its public #[no_mangle] functions
[01:47:26] [ "$(nm -D /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_cdylib.so | grep -c public_c_function_from_cdylib)" -eq "1" ]
[01:47:26] # Check that a cdylib exports the public #[no_mangle] functions of dependencies
[01:47:26] [ "$(nm -D /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_cdylib.so | grep -c public_c_function_from_rlib)" -eq "1" ]
[01:47:26] # Check that a cdylib DOES NOT export any public Rust functions
[01:47:26] [ "$(nm -D /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_cdylib.so | grep -c _ZN.*h.*E)" -eq "0" ]
[01:47:26] # Check that a Rust dylib exports its monomorphic functions
[01:47:26] [ "$(nm -D /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_rust_dylib.so | grep -c public_c_function_from_rust_dylib)" -eq "1" ]
[01:47:26] [ "$(nm -D /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/symbol-visibility/symbol-visibility/liba_rust_dylib.so | grep -c _ZN.*public_rust_function_from_rust_dylib.*E)" -eq "1" ]
[01:47:26] Makefile:26: recipe for target 'all' failed
[01:47:26] 
[01:47:26] ------------------------------------------
[01:47:26] stderr:
[01:47:26] ------------------------------------------
[01:47:26] ------------------------------------------
[01:47:26] make[1]: *** [all] Error 1
[01:47:26] ------------------------------------------
[01:47:26] 
[01:47:26] thread '[run-make] run-make-fulldeps/symbol-visibility' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3288:9
[01:47:26] 
---
[01:47:26] test result: FAILED. 190 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:47:26] 
[01:47:26] 
[01:47:26] 
[01:47:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:47:26] 
[01:47:26] 
[01:47:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:47:26] Build completed unsuccessfully in 0:42:19
[01:47:26] Build completed unsuccessfully in 0:42:19
[01:47:26] make: *** [check] Error 1
[01:47:26] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00a7da6e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 29 10:59:14 UTC 2019
---
travis_time:end:067829a0:start=1548759557057189775,finish=1548759557063875475,duration=6685700
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0078a9c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed '
