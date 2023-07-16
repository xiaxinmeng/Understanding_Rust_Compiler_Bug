plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:24b2aad8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
####################################                                      50.3%
######################################################################## 100.0%
[00:04:42] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:04:42]     Updating crates.io index
[00:04:56]     Updating git repository `https://github.com/tkaitchuck/aHash.git`
[00:04:58]   Downloaded cmake v0.1.33
[00:04:58]   Downloaded getopts v0.2.17
[00:04:58]   Downloaded cc v1.0.28
[00:04:58]   Downloaded serde_derive v1.0.81
---
[00:08:52]    Compiling proc-macro-hack v0.5.4
[00:08:52]    Compiling synstructure v0.10.1
[00:08:58]    Compiling const-random-macro v0.1.3
[00:09:06]    Compiling const-random v0.1.3
[00:09:06]    Compiling ahash v0.1.18 (https://github.com/tkaitchuck/aHash.git#91bcb84a)
[00:09:06]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:09:13]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:09:13]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:09:17]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
---
[00:22:40]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:22:40]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:23:21]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:24:36]    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
[00:24:37] error: linking with `clang` failed: exit code: 1
[00:24:37]   |
[00:24:37]   = note: "clang" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-bfe62e932bf70be7.rustc_binary.en7abl2s-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-bfe62e932bf70be7.rustc_binary.en7abl2s-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-bfe62e932bf70be7" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-fe46b6800c017b22/out/build/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-c83fbb1484f687df/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-9bf1df8bf6ec3444/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-675077696c32d904" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_interface-612ade6d9a44609d" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_lint-8edd76ffb0fe70f8" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_traits-1755bf9fe4581ed4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_resolve-bca8c33e84dbda5b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_privacy-fa1e4e4f650bc40e" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_plugin-6465bcecd0ffc669" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_passes-7c5b663c4e08adff" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_incremental-274dd1cc0fbba4af" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_borrowck-21b74fc9fbc42bb3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_allocator-2932929d56772277" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_save_analysis-dfff09402f855cd1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_codegen_utils-b6a54b9694b338cd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_metadata-95dc2cf8573b3a3a" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax_ext-f51519c4a8d4a378" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_mir-3b265a2f2b038c68" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_typeck-4989d5fa94719ec9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc-04924b9b3d8ee86d" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lfmt_macros-0b9fd772ce373859" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_fs_util-56cda24045c78362" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-ltest-8bbc568dbe611f62" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-llibtest-f3bbabcafd87c144" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lrustc_term-4b1431547060ae93" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax-0fc3323b5d348b78" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_target-b1b5336f5b508820" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_errors-ed09671b6178b62d" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax_pos-596155ab83f45a0d" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-larena-ca4509d44c4a2582" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_data_structures-ace0f5facabac100" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lgraphviz-177e469eced94c36" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_cratesio_shim-4c02d8c050c01dd7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lserialize-8ca5e532ee635e13" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjemalloc_sys-9e461a8d5c1e96fa.rlib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-87beca1a90eead9b" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-f6af5ba0167a1bba.rlib" "-Wl,-Bdynamic" "-lpthread" "-lutil" "-lutil" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../lib"
[00:24:37]   = note: /rustroot/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjemalloc_sys-9e461a8d5c1e96fa.rlib(ctl.pic.o): undefined reference to symbol '__rawmemchr@@GLIBC_2.2.5'
[00:24:37]           /lib64/libc.so.6: error adding symbols: DSO missing from command line
[00:24:37]           clang-8: error: linker command failed with exit code 1 (use -v to see invocation)
[00:24:37] 
[00:24:37] error: aborting due to previous error
[00:24:37] 
[00:24:37] error: Could not compile `rustc-main`.
---
travis_time:end:1328db69:start=1554063611326208332,finish=1554063611338572714,duration=12364382
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c7f3632
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c4e92c0
travis_time:start:1c4e92c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02688ee4
$ dmesg | grep -i kill
