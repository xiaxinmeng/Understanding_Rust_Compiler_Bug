plain
[00:23:00]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:23:03]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:24:03]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:24:13]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:24:42] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/mod.rs:4897:41
[00:24:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
-unknown-linux-gnu/release/deps/librustc_rayon-e1907f3777b8aca0.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-819cb430c31f8238.so --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-ad6626ca6f9c7adf.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c8132f366b701b34.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c8132f366b701b34.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-d441f38020b30a7e.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-da75a1391e0819dc.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-7e250215e5863400.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-053154eea32faea2.rlib --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-c83988feac67e21f.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-e0b46302cb6adfc1.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-lin/x86_64-unknown-linux-gnu/lib
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
68788 ./src/llvm/lib
65424 ./src/llvm-emscripten/test/CodeGen
62124 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
---
travis_time:end:01ee438a:start=1530224657264233245,finish=1530224657274840818,duration=10607573
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1977bd2a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16e549dc
$ dmesg | grep -i kill
