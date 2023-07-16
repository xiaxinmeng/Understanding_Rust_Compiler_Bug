plain
2019-09-05T07:19:37.8904587Z [RUSTC-TIMING] panic_unwind test:false 0.260
2019-09-05T07:19:38.5787455Z [RUSTC-TIMING] hashbrown test:false 0.866
2019-09-05T07:19:57.6687338Z error: linking with `rust-lld` failed: exit code: 1
2019-09-05T07:19:57.6688606Z   |
2019-09-05T07:19:57.6692913Z   = note: "rust-lld" "-flavor" "gnu" "--build-id" "--eh-frame-hdr" "--hash-style=gnu" "-z" "rodynamic" "--dynamic-linker=ld.so.1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-18fbc03c077c11be.std.7pkhc7iw-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libstd-18fbc03c077c11be.so" "--version-script=/tmp/rustcVsHW5X/list" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-18fbc03c077c11be.3d2zvctenki9tp9b.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-18fbc03c077c11be.4xzxw5c8ok9pskoe.rcgu.o" "-O1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-7d3e5b1abddf868d/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/backtrace-sys-b8583d4026cb390c/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "-lzircon" "-lfdio" "-Bstatic" "--whole-archive" "/tmp/rustcVsHW5X/libpanic_unwind-57598bc416d1fec2.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/libbacktrace-5e4e44c3709a6ee9.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/librustc_demangle-7ba4493fedd92f7d.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/libhashbrown-9417bfef281cd46c.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/librustc_std_workspace_alloc-e6f7b9eede6e8679.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/libunwind-d883bd7771e20bde.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/libcfg_if-79b998edafa97e8e.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/liblibc-ff7fe28e2f588926.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/liballoc-947a4762c0c0a056.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/librustc_std_workspace_core-c1e8f3dd410d714d.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcVsHW5X/libcore-84d107d4cc594f9f.rlib" "--no-whole-archive" "/tmp/rustcVsHW5X/libcompiler_builtins-7d898751eaf69b01.rlib" "-Bdynamic" "-lunwind" "-lc" "-lfdio" "-shared"
2019-09-05T07:19:57.6695206Z   = note: rust-lld: error: unable to find library -lzircon
2019-09-05T07:19:57.6696226Z           rust-lld: error: unable to find library -lfdio
2019-09-05T07:19:57.6696887Z           rust-lld: error: unable to find library -lunwind
2019-09-05T07:19:57.6697450Z           rust-lld: error: unable to find library -lc
2019-09-05T07:19:57.6697978Z           rust-lld: error: unable to find library -lfdio
2019-09-05T07:19:57.6702201Z 
2019-09-05T07:19:57.6734714Z error: aborting due to previous error
2019-09-05T07:19:57.6738504Z 
2019-09-05T07:19:57.6949150Z [RUSTC-TIMING] std test:false 19.112
---
2019-09-05T07:19:57.7111991Z == clock drift check ==
2019-09-05T07:19:57.7133941Z   local time: Thu Sep  5 07:19:57 UTC 2019
2019-09-05T07:19:57.8687466Z   network time: Thu, 05 Sep 2019 07:19:57 GMT
2019-09-05T07:19:57.8691799Z == end clock drift check ==
2019-09-05T07:20:00.4132704Z ##[error]Bash exited with code '1'.
2019-09-05T07:20:00.4177499Z ##[section]Starting: Upload CPU usage statistics
2019-09-05T07:20:00.4183266Z ==============================================================================
2019-09-05T07:20:00.4183404Z Task         : Bash
2019-09-05T07:20:00.4183509Z Description  : Run a Bash script on macOS, Linux, or Windows
