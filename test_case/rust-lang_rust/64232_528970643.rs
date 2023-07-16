plain
2019-09-06T18:45:59.4371009Z [RUSTC-TIMING] panic_unwind test:false 0.309
2019-09-06T18:46:00.0672055Z [RUSTC-TIMING] hashbrown test:false 0.883
2019-09-06T18:46:19.2419829Z error: linking with `rust-lld` failed: exit code: 1
2019-09-06T18:46:19.2421398Z   |
2019-09-06T18:46:19.2425794Z   = note: "rust-lld" "-flavor" "gnu" "--build-id" "--eh-frame-hdr" "--hash-style=gnu" "-z" "rodynamic" "--dynamic-linker=ld.so.1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c89ef629c0b62996.std.c9q5bil2-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libstd-c89ef629c0b62996.so" "--version-script=/tmp/rustcAVp1La/list" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c89ef629c0b62996.270qm7qio3xzldr8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c89ef629c0b62996.27wdggknlnjwaroe.rcgu.o" "-O1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-1f861c53393e2e2f/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/backtrace-sys-c7019a52a4415df9/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "-lzircon" "-lfdio" "-Bstatic" "--whole-archive" "/tmp/rustcAVp1La/libpanic_unwind-538d2fc6626ebfd1.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/libbacktrace-42ab14b8b06e3711.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/librustc_demangle-041cfa99b4e95774.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/libhashbrown-27a467445f478bff.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/librustc_std_workspace_alloc-199dfb46e8783261.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/libunwind-9030432a334c5f97.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/libcfg_if-96b69eaade2b17bb.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/liblibc-c8f92346994b59a6.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/liballoc-ea2174868aad93fd.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/librustc_std_workspace_core-394fd8fd98a56a29.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcAVp1La/libcore-1b119d5b11fe2c3b.rlib" "--no-whole-archive" "/tmp/rustcAVp1La/libcompiler_builtins-b6826176b7b29207.rlib" "-Bdynamic" "-lunwind" "-lc" "-lfdio" "-shared"
2019-09-06T18:46:19.2428609Z   = note: rust-lld: error: unable to find library -lzircon
2019-09-06T18:46:19.2429256Z           rust-lld: error: unable to find library -lfdio
2019-09-06T18:46:19.2429876Z           rust-lld: error: unable to find library -lunwind
2019-09-06T18:46:19.2430794Z           rust-lld: error: unable to find library -lc
2019-09-06T18:46:19.2431436Z           rust-lld: error: unable to find library -lfdio
2019-09-06T18:46:19.2435101Z 
2019-09-06T18:46:19.2452106Z error: aborting due to previous error
2019-09-06T18:46:19.2455337Z 
2019-09-06T18:46:19.2603138Z [RUSTC-TIMING] std test:false 19.189
---
2019-09-06T18:46:19.2742592Z == clock drift check ==
2019-09-06T18:46:19.2762051Z   local time: Fri Sep  6 18:46:19 UTC 2019
2019-09-06T18:46:19.3628942Z   network time: Fri, 06 Sep 2019 18:46:19 GMT
2019-09-06T18:46:19.3632936Z == end clock drift check ==
2019-09-06T18:46:21.5744203Z ##[error]Bash exited with code '1'.
2019-09-06T18:46:21.5782315Z ##[section]Starting: Upload CPU usage statistics
2019-09-06T18:46:21.5788577Z ==============================================================================
2019-09-06T18:46:21.5788713Z Task         : Bash
2019-09-06T18:46:21.5788784Z Description  : Run a Bash script on macOS, Linux, or Windows
