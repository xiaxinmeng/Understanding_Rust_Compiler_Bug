plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:2a9f5aa8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:04:44] 
######################################################################## 100.0%
[00:04:45] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:04:45]     Updating crates.io index
[00:05:05]     Updating git repository `https://github.com/Zoxc/mimallocator.git`
[00:05:08]   Downloaded getopts v0.2.19
[00:05:08]   Downloaded lazy_static v1.3.0
[00:05:08]   Downloaded filetime v0.2.4
[00:05:08]   Downloaded petgraph v0.4.13
---
[00:15:27]    Compiling rand_chacha v0.1.0
[00:15:28]    Compiling rand v0.6.1
[00:15:28]    Compiling parking_lot_core v0.4.0
[00:15:28]    Compiling quote v0.6.12
[00:15:29]    Compiling mimalloc-sys v0.1.1 (https://github.com/Zoxc/mimallocator.git#7be315f3)
[00:15:35]    Compiling measureme v0.3.0
[00:15:48]    Compiling flate2 v1.0.6
[00:15:52]    Compiling rls-span v0.5.1
[00:15:53]    Compiling serde_json v1.0.33
[00:15:53]    Compiling serde_json v1.0.33
[00:16:06]    Compiling rls-data v0.19.0
[00:16:11]    Compiling mimallocator v0.1.0 (https://github.com/Zoxc/mimallocator.git#7be315f3)
[00:16:13]    Compiling parking_lot v0.7.1
[00:16:14]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:16:19]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:16:31]    Compiling synstructure v0.10.2
---
[00:29:45]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:29:45]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:30:14]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:31:14]    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
[00:31:15] error: linking with `clang` failed: exit code: 1
[00:31:15]   |
[00:31:15]   = note: "clang" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-43873750fade7b55.rustc_binary.a8iqc3xj-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-43873750fade7b55.rustc_binary.a8iqc3xj-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-43873750fade7b55.rustc_binary.a8iqc3xj-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-43873750fade7b55.rustc_binary.a8iqc3xj-cgu.3.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-43873750fade7b55" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-7bf1fc81fc6cad69/out/build/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/mimalloc-sys-dca5aea327a63ddd/out/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-56f9b52425605653/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-1efef4ffbce85b3e/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-9e244f1d54f16f75" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_interface-695d60c0fb008578" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_lint-69bf5bbf675bfddc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_traits-f4ed0b02eebb9bc0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_resolve-2a50d5688bc03ade" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_privacy-00d7c74097c06957" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_typeck-268675138e417644" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_plugin-e26ea52538e6d5ff" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_passes-fc3ad3e82932cd11" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_codegen_ssa-31352b737ef959a0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_incremental-90b20d942da41fcd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_borrowck-43faf517926c339d" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_mir-8acd7111522e6b93" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_allocator-a55bf43315d2e036" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_save_analysis-83183815b51851ae" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_codegen_utils-506f6d88a0b4d63e" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_metadata-88f02aeae859cd85" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax_ext-766f1e2ce3e26b9d" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc-cac9efe5639ad1b4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lfmt_macros-0887ee7f8af32dad" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_fs_util-5f0122864e858157" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-ltest-8612e47523425cd2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lterm-42a3bfb8cb8e322f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax-11299e9e536ac5b6" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_target-7065623a095f14d2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_errors-4b055c20a7116a81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax_pos-251aa25f2cc64ab2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-larena-88bcda2d953d1aea" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_data_structures-f3015cb9147940a4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lgraphviz-5802bba1e3dd27f2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_cratesio_shim-7a3fa46122ece1d2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lserialize-93d296be51c19caa" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjemalloc_sys-c086f4d66da1bb7e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmimallocator-6a4c93f25f12b321.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmimalloc_sys-f89481adb10b382e.rlib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-15369ae74ee20b7f" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-361e1252e6f0814d.rlib" "-Wl,-Bdynamic" "-lpthread" "-lmimalloc-static" "-lutil" "-lutil" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../lib"
[00:31:15]   = note: /rustroot/bin/ld: cannot find -lmimalloc-static
[00:31:15]           clang-8: error: linker command failed with exit code 1 (use -v to see invocation)
[00:31:15] 
[00:31:15] error: aborting due to previous error
[00:31:15] 
[00:31:15] error: Could not compile `rustc-main`.
---
travis_time:end:11df0b49:start=1561282452092538860,finish=1561282452099138012,duration=6599152
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1db69350
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0083ac57
travis_time:start:0083ac57
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f4808d8
$ dmesg | grep -i kill
