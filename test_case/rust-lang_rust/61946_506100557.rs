plain
travis_time:end:16f92038:start=1561593310197961733,finish=1561593401524844463,duration=91326882730
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:54:49] travis_fold:start:test_ui
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:50] configuration:
[00:54:50] compile_lib_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib"
[00:54:50] run_lib_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib"
[00:54:50] rustc_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc"
[00:54:50] rustdoc_path: None
[00:54:50] src_base: "/checkout/src/test/ui"
[00:54:50] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui"
[00:54:50] stage_id: stage2-x86_64-unknown-linux-gnu
[00:54:50] mode: ui
[00:54:50] run_ignored: false
[00:54:50] filter: (none)
[00:54:50] filter_exact: false
[00:54:50] runtool: (none)
[00:54:50] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:54:50] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:54:50] host: x86_64-unknown-linux-gnu
[00:54:50] android-cross-path: ""
[00:54:50] android-cross-path: ""
[00:54:50] adb_path: "adb"
[00:54:50] adb_test_dir: "/data/tmp/work"
[00:54:50] adb_device_status: false
[00:54:50] ar: ar
[00:54:50] linker: None
[00:54:50] verbose: true
[00:54:50] quiet: false
[00:54:50] 
[00:54:50] 
[00:54:50] running 5709 tests
[00:54:50] test [ui] ui/alloc-error/alloc-error-handler-bad-signature-1.rs ... ok
---
[00:58:38] src_base: "/checkout/src/test/run-pass"
[00:58:38] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass"
[00:58:38] stage_id: stage2-x86_64-unknown-linux-gnu
[00:58:38] mode: run-pass
[00:58:38] run_ignored: false
[00:58:38] filter: (none)
[00:58:38] filter_exact: false
[00:58:38] runtool: (none)
[00:58:38] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:58:38] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:58:38] host: x86_64-unknown-linux-gnu
[00:58:38] android-cross-path: ""
[00:58:38] android-cross-path: ""
[00:58:38] adb_path: "adb"
[00:58:38] adb_test_dir: "/data/tmp/work"
[00:58:38] adb_device_status: false
[00:58:38] ar: ar
[00:58:38] linker: None
[00:58:38] verbose: true
[00:58:38] quiet: false
[00:58:38] 
[00:58:39] 
[00:58:39] running 2920 tests
[00:58:39] test [run-pass] run-pass/alias-uninit-value.rs ... ok
---
[01:05:10] src_base: "/checkout/src/test/compile-fail"
[01:05:10] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail"
[01:05:10] stage_id: stage2-x86_64-unknown-linux-gnu
[01:05:10] mode: compile-fail
[01:05:10] run_ignored: false
[01:05:10] filter: (none)
[01:05:10] filter_exact: false
[01:05:10] runtool: (none)
[01:05:10] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:10] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:10] host: x86_64-unknown-linux-gnu
[01:05:10] android-cross-path: ""
[01:05:10] android-cross-path: ""
[01:05:10] adb_path: "adb"
[01:05:10] adb_test_dir: "/data/tmp/work"
[01:05:10] adb_device_status: false
[01:05:10] ar: ar
[01:05:10] linker: None
[01:05:10] verbose: true
[01:05:10] quiet: false
[01:05:10] 
[01:05:10] 
[01:05:10] running 30 tests
[01:05:10] test [compile-fail] compile-fail/chalkify/chalk_initial_program.rs ... ok
---
[01:05:11] src_base: "/checkout/src/test/run-fail"
[01:05:11] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail"
[01:05:11] stage_id: stage2-x86_64-unknown-linux-gnu
[01:05:11] mode: run-fail
[01:05:11] run_ignored: false
[01:05:11] filter: (none)
[01:05:11] filter_exact: false
[01:05:11] runtool: (none)
[01:05:11] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:11] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:11] host: x86_64-unknown-linux-gnu
[01:05:11] android-cross-path: ""
[01:05:11] android-cross-path: ""
[01:05:11] adb_path: "adb"
[01:05:11] adb_test_dir: "/data/tmp/work"
[01:05:11] adb_device_status: false
[01:05:11] ar: ar
[01:05:11] linker: None
[01:05:11] verbose: true
[01:05:11] quiet: false
[01:05:11] 
[01:05:11] 
[01:05:11] running 144 tests
[01:05:11] test [run-fail] run-fail/args-panic.rs ... ok
---
[01:05:24] src_base: "/checkout/src/test/run-pass-valgrind"
[01:05:24] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind"
[01:05:24] stage_id: stage2-x86_64-unknown-linux-gnu
[01:05:24] mode: run-pass-valgrind
[01:05:24] run_ignored: false
[01:05:24] filter: (none)
[01:05:24] filter_exact: false
[01:05:24] runtool: (none)
[01:05:24] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:24] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:24] host: x86_64-unknown-linux-gnu
[01:05:24] android-cross-path: ""
[01:05:24] android-cross-path: ""
[01:05:24] adb_path: "adb"
[01:05:24] adb_test_dir: "/data/tmp/work"
[01:05:24] adb_device_status: false
[01:05:24] ar: ar
[01:05:24] linker: None
[01:05:24] verbose: true
[01:05:24] quiet: false
[01:05:24] 
[01:05:24] 
[01:05:24] running 17 tests
[01:05:24] test [run-pass-valgrind] run-pass-valgrind/coerce-match-calls.rs ... ok
---
[01:05:26] travis_fold:start:test_mir-opt
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:26] configuration:
[01:05:26] compile_lib_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib"
[01:05:26] run_lib_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib"
[01:05:26] rustc_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc"
[01:05:26] rustdoc_path: None
[01:05:26] src_base: "/checkout/src/test/mir-opt"
[01:05:26] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt"
[01:05:26] stage_id: stage2-x86_64-unknown-linux-gnu
[01:05:26] mode: mir-opt
[01:05:26] run_ignored: false
[01:05:26] filter: (none)
[01:05:26] filter_exact: false
[01:05:26] runtool: (none)
[01:05:26] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:26] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:26] host: x86_64-unknown-linux-gnu
[01:05:26] android-cross-path: ""
[01:05:26] android-cross-path: ""
[01:05:26] adb_path: "adb"
[01:05:26] adb_test_dir: "/data/tmp/work"
[01:05:26] adb_device_status: false
[01:05:26] ar: ar
[01:05:26] linker: None
[01:05:26] verbose: true
[01:05:26] quiet: false
[01:05:26] 
[01:05:26] 
[01:05:26] running 56 tests
[01:05:27] test [mir-opt] mir-opt/byte_slice.rs ... ok
---
[01:05:49] src_base: "/checkout/src/test/codegen"
[01:05:49] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen"
[01:05:49] stage_id: stage2-x86_64-unknown-linux-gnu
[01:05:49] mode: codegen
[01:05:49] run_ignored: false
[01:05:49] filter: (none)
[01:05:49] filter_exact: false
[01:05:49] runtool: (none)
[01:05:49] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:49] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:49] host: x86_64-unknown-linux-gnu
[01:05:49] android-cross-path: ""
[01:05:49] android-cross-path: ""
[01:05:49] adb_path: "adb"
[01:05:49] adb_test_dir: "/data/tmp/work"
[01:05:49] adb_device_status: false
[01:05:49] ar: ar
[01:05:49] linker: None
[01:05:49] verbose: true
[01:05:49] quiet: false
[01:05:49] 
[01:05:49] 
[01:05:49] running 146 tests
[01:05:49] test [codegen] codegen/abi-main-signature-16bit-c-int.rs ... ignored
---
[01:05:54] src_base: "/checkout/src/test/codegen-units"
[01:05:54] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units"
[01:05:54] stage_id: stage2-x86_64-unknown-linux-gnu
[01:05:54] mode: codegen-units
[01:05:54] run_ignored: false
[01:05:54] filter: (none)
[01:05:54] filter_exact: false
[01:05:54] runtool: (none)
[01:05:54] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:54] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:54] host: x86_64-unknown-linux-gnu
[01:05:54] android-cross-path: ""
[01:05:54] android-cross-path: ""
[01:05:54] adb_path: "adb"
[01:05:54] adb_test_dir: "/data/tmp/work"
[01:05:54] adb_device_status: false
[01:05:54] ar: ar
[01:05:54] linker: None
[01:05:54] verbose: true
[01:05:54] quiet: false
[01:05:54] 
[01:05:54] 
[01:05:54] running 39 tests
[01:05:54] test [codegen-units] codegen-units/item-collection/cross-crate-closures.rs ... ignored
---
[01:05:55] travis_fold:start:test_assembly
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:56] configuration:
[01:05:56] compile_lib_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib"
[01:05:56] run_lib_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib"
[01:05:56] rustc_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc"
[01:05:56] rustdoc_path: None
[01:05:56] src_base: "/checkout/src/test/assembly"
[01:05:56] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly"
[01:05:56] stage_id: stage2-x86_64-unknown-linux-gnu
[01:05:56] mode: assembly
[01:05:56] run_ignored: false
[01:05:56] filter: (none)
[01:05:56] filter_exact: false
[01:05:56] runtool: (none)
[01:05:56] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:56] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:56] host: x86_64-unknown-linux-gnu
[01:05:56] android-cross-path: ""
[01:05:56] android-cross-path: ""
[01:05:56] adb_path: "adb"
[01:05:56] adb_test_dir: "/data/tmp/work"
[01:05:56] adb_device_status: false
[01:05:56] ar: ar
[01:05:56] linker: None
[01:05:56] verbose: true
[01:05:56] quiet: false
[01:05:56] 
[01:05:56] 
[01:05:56] running 9 tests
[01:05:56] test [assembly] assembly/nvptx-arch-default.rs ... ignored
---
[01:05:56] src_base: "/checkout/src/test/incremental"
[01:05:56] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental"
[01:05:56] stage_id: stage2-x86_64-unknown-linux-gnu
[01:05:56] mode: incremental
[01:05:56] run_ignored: false
[01:05:56] filter: (none)
[01:05:56] filter_exact: false
[01:05:56] runtool: (none)
[01:05:56] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:56] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:05:56] host: x86_64-unknown-linux-gnu
[01:05:56] android-cross-path: ""
[01:05:56] android-cross-path: ""
[01:05:56] adb_path: "adb"
[01:05:56] adb_test_dir: "/data/tmp/work"
[01:05:56] adb_device_status: false
[01:05:56] ar: ar
[01:05:56] linker: None
[01:05:56] verbose: true
[01:05:56] quiet: false
[01:05:56] 
[01:05:56] 
[01:05:56] running 104 tests
[01:05:56] test [incremental] incremental/cache_file_headers.rs ... ok
---
[01:06:12] src_base: "/checkout/src/test/debuginfo"
[01:06:12] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo"
[01:06:12] stage_id: stage2-x86_64-unknown-linux-gnu
[01:06:12] mode: debuginfo-gdb+lldb
[01:06:12] run_ignored: false
[01:06:12] filter: (none)
[01:06:12] filter_exact: false
[01:06:12] runtool: (none)
[01:06:12] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:06:12] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:06:12] host: x86_64-unknown-linux-gnu
[01:06:12] android-cross-path: ""
[01:06:12] android-cross-path: ""
[01:06:12] adb_path: "adb"
[01:06:12] adb_test_dir: "/data/tmp/work"
[01:06:12] adb_device_status: false
[01:06:12] ar: ar
[01:06:12] linker: None
[01:06:12] verbose: true
[01:06:12] quiet: false
[01:06:12] 
[01:06:12] 
[01:06:12] running 122 tests
[01:06:12] test [debuginfo-gdb+lldb] debuginfo/associated-types.rs ... ok
---
[01:06:43] travis_fold:start:test_ui-fulldeps
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:43] configuration:
[01:06:43] compile_lib_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib"
[01:06:43] run_lib_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib"
[01:06:43] rustc_path: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc"
[01:06:43] rustdoc_path: None
[01:06:43] src_base: "/checkout/src/test/ui-fulldeps"
[01:06:43] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps"
[01:06:43] stage_id: stage2-x86_64-unknown-linux-gnu
[01:06:43] mode: ui
[01:06:43] run_ignored: false
[01:06:43] filter: (none)
[01:06:43] filter_exact: false
[01:06:43] runtool: (none)
[01:06:43] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:06:43] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:06:43] host: x86_64-unknown-linux-gnu
[01:06:43] android-cross-path: ""
[01:06:43] android-cross-path: ""
[01:06:43] adb_path: "adb"
[01:06:43] adb_test_dir: "/data/tmp/work"
[01:06:43] adb_device_status: false
[01:06:43] ar: ar
[01:06:43] linker: None
[01:06:43] verbose: true
[01:06:43] quiet: false
[01:06:43] 
[01:06:43] 
[01:06:43] running 24 tests
[01:06:43] test [ui] ui-fulldeps/dropck-tarena-unsound-drop.rs ... ok
---
[01:06:51] src_base: "/checkout/src/test/run-pass-fulldeps"
[01:06:51] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps"
[01:06:51] stage_id: stage2-x86_64-unknown-linux-gnu
[01:06:51] mode: run-pass
[01:06:51] run_ignored: false
[01:06:51] filter: (none)
[01:06:51] filter_exact: false
[01:06:51] runtool: (none)
[01:06:51] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:06:51] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:06:51] host: x86_64-unknown-linux-gnu
[01:06:51] android-cross-path: ""
[01:06:51] android-cross-path: ""
[01:06:51] adb_path: "adb"
[01:06:51] adb_test_dir: "/data/tmp/work"
[01:06:51] adb_device_status: false
[01:06:51] ar: ar
[01:06:51] linker: None
[01:06:51] verbose: true
[01:06:51] quiet: false
[01:06:51] 
[01:06:51] 
[01:06:51] running 44 tests
[01:06:51] test [run-pass] run-pass-fulldeps/derive-no-std-not-supported.rs ... ok
---
[01:10:09] src_base: "/checkout/src/test/rustdoc"
[01:10:09] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc"
[01:10:09] stage_id: stage2-x86_64-unknown-linux-gnu
[01:10:09] mode: rustdoc
[01:10:09] run_ignored: false
[01:10:09] filter: (none)
[01:10:09] filter_exact: false
[01:10:09] runtool: (none)
[01:10:09] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:10:09] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:10:09] host: x86_64-unknown-linux-gnu
[01:10:09] android-cross-path: ""
[01:10:09] android-cross-path: ""
[01:10:09] adb_path: "adb"
[01:10:09] adb_test_dir: "/data/tmp/work"
[01:10:09] adb_device_status: false
[01:10:09] ar: ar
[01:10:09] linker: None
[01:10:09] verbose: true
[01:10:09] quiet: false
[01:10:09] 
[01:10:09] 
[01:10:09] running 313 tests
[01:10:12] test [rustdoc] rustdoc/all.rs ... ok
---
[01:13:34] src_base: "/checkout/src/test/pretty"
[01:13:34] build_base: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty"
[01:13:34] stage_id: stage2-x86_64-unknown-linux-gnu
[01:13:34] mode: pretty
[01:13:34] run_ignored: false
[01:13:34] filter: (none)
[01:13:34] filter_exact: false
[01:13:34] runtool: (none)
[01:13:34] host-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:13:34] target-rustcflags: -Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[01:13:34] host: x86_64-unknown-linux-gnu
[01:13:34] android-cross-path: ""
[01:13:34] android-cross-path: ""
[01:13:34] adb_path: "adb"
[01:13:34] adb_test_dir: "/data/tmp/work"
[01:13:34] adb_device_status: false
[01:13:34] ar: ar
[01:13:34] linker: None
[01:13:34] verbose: true
[01:13:34] quiet: false
[01:13:34] 
[01:13:34] 
[01:13:34] running 52 tests
[01:13:34] test [pretty] pretty/asm-options.rs ... ok
---
[01:20:06] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:06]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:20:08] error[E0425]: cannot find value `invalid_option` in this scope
[01:20:08]     --> src/libstd/fs.rs:3109:13
[01:20:08] 3109 |             invalid_option = "invalid argument";
[01:20:08] 3109 |             invalid_option = "invalid argument";
[01:20:08]      |             ^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `invalid_options`
[01:20:25] error: aborting due to previous error
[01:20:25] 
[01:20:25] For more information about this error, try `rustc --explain E0425`.
[01:20:25] error: Could not compile `std`.
[01:20:25] error: Could not compile `std`.
[01:20:25] 
[01:20:25] To learn more, run the command again with --verbose.
[01:20:25] 
[01:20:25] 
[01:20:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:20:25] 
[01:20:25] 
[01:20:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:25] Build completed unsuccessfully in 1:16:24
---
travis_time:end:0b4ce126:start=1561598237929241537,finish=1561598237936699166,duration=7457629
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2554b62a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08c6f074
travis_time:start:08c6f074
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07c37248
$ dmesg | grep -i kill
