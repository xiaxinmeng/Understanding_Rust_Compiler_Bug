plain
.................................................................................................... 9200/11468
.................................................................................................... 9300/11468
.................................................................................................... 9400/11468
..........................i......i.................................................................. 9500/11468
.................................................................iiiiiii..iiiiii.i.................. 9600/11468
.................................................................................................... 9800/11468
.................................................................................................... 9900/11468
.................................................................................................... 10000/11468
.................................................................................................... 10100/11468
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.069 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.415 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"library/std"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 2.598 seconds
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (/checkout/library/std)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-Wl,--eh-frame-hdr" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.13.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.14.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.15.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-44a0a549c4d20832/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand-f206f162d5fcbf13.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand_chacha-48cc45927a747e37.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libppv_lite86-91ea22edafcf05ed.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand_core-3eb5418979e33a4d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libgetrandom-52a3b20344a46072.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-ltest-5c4e3aa847ac7832" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-360a2d238031f7d1" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-a3426394929bdfab.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,-rpath,$ORIGIN/../lib"
  = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.1.rcgu.o: In function `std::f64::tests::test_round_to_even':
          std.af8qn5ha-cgu.1:(.text._ZN3std3f645tests18test_round_to_even17h7bb5de47ee68f75cE+0xe): undefined reference to `llvm.roundeven.f64'
          std.af8qn5ha-cgu.1:(.text._ZN3std3f645tests18test_round_to_even17h7bb5de47ee68f75cE+0x43): undefined reference to `llvm.roundeven.f64'
          std.af8qn5ha-cgu.1:(.text._ZN3std3f645tests18test_round_to_even17h7bb5de47ee68f75cE+0x78): undefined reference to `llvm.roundeven.f64'
          std.af8qn5ha-cgu.1:(.text._ZN3std3f645tests18test_round_to_even17h7bb5de47ee68f75cE+0xad): undefined reference to `llvm.roundeven.f64'
          std.af8qn5ha-cgu.1:(.text._ZN3std3f645tests18test_round_to_even17h7bb5de47ee68f75cE+0xe2): undefined reference to `llvm.roundeven.f64'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.1.rcgu.o:std.af8qn5ha-cgu.1:(.text._ZN3std3f645tests18test_round_to_even17h7bb5de47ee68f75cE+0x113): more undefined references to `llvm.roundeven.f64' follow
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.15.rcgu.o: In function `std::f32::tests::test_round_to_even':
          std.af8qn5ha-cgu.15:(.text._ZN3std3f325tests18test_round_to_even17h60b787ccaecb27e8E+0xe): undefined reference to `llvm.roundeven.f32'
          std.af8qn5ha-cgu.15:(.text._ZN3std3f325tests18test_round_to_even17h60b787ccaecb27e8E+0x42): undefined reference to `llvm.roundeven.f32'
          std.af8qn5ha-cgu.15:(.text._ZN3std3f325tests18test_round_to_even17h60b787ccaecb27e8E+0x76): undefined reference to `llvm.roundeven.f32'
          std.af8qn5ha-cgu.15:(.text._ZN3std3f325tests18test_round_to_even17h60b787ccaecb27e8E+0xaa): undefined reference to `llvm.roundeven.f32'
          std.af8qn5ha-cgu.15:(.text._ZN3std3f325tests18test_round_to_even17h60b787ccaecb27e8E+0xde): undefined reference to `llvm.roundeven.f32'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-c499aaf4f8b73525.std.af8qn5ha-cgu.15.rcgu.o:std.af8qn5ha-cgu.15:(.text._ZN3std3f325tests18test_round_to_even17h60b787ccaecb27e8E+0x10d): more undefined references to `llvm.roundeven.f32' follow
          collect2: error: ld returned 1 exit status

error: aborting due to previous error

error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:23
