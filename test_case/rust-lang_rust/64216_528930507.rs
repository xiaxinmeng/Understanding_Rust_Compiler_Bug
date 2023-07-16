plain
2019-09-06T16:50:41.4201293Z [RUSTC-TIMING] rustc_demangle test:false 1.954
2019-09-06T16:50:41.5435417Z [RUSTC-TIMING] hashbrown test:false 0.843
2019-09-06T16:50:59.8870827Z error: linking with `rust-lld` failed: exit code: 1
2019-09-06T16:50:59.8871937Z   |
2019-09-06T16:50:59.8876456Z   = note: "rust-lld" "-flavor" "gnu" "--build-id" "--eh-frame-hdr" "--hash-style=gnu" "-z" "rodynamic" "--dynamic-linker=ld.so.1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-907f9105db5863da.std.esy9z4rd-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libstd-907f9105db5863da.so" "--version-script=/tmp/rustcwgS3zf/list" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-907f9105db5863da.oxi41vvl1lcd13a.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-907f9105db5863da.3stmjbdkckw0iy3x.rcgu.o" "-O1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-1e4ba6d0b33e4f77/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/backtrace-sys-e37ae7a46570c3eb/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "-lzircon" "-lfdio" "-Bstatic" "--whole-archive" "/tmp/rustcwgS3zf/libpanic_unwind-ed29837ea9877e03.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/libbacktrace-31bd7be02e9bfb00.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/librustc_demangle-a2d332448a92ee43.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/libhashbrown-8267804f33516c9c.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/librustc_std_workspace_alloc-06d7176f4a65cc0e.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/libunwind-5e5859ac2741bcd5.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/libcfg_if-9a714e4ca47a9cfa.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/liblibc-2627bd081d486a4d.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/liballoc-56cc9264fb8a3eb7.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/librustc_std_workspace_core-e37f3f206b737f6d.rlib" "--no-whole-archive" "--whole-archive" "/tmp/rustcwgS3zf/libcore-55b9b454cffd3db5.rlib" "--no-whole-archive" "/tmp/rustcwgS3zf/libcompiler_builtins-e368f374c1108881.rlib" "-Bdynamic" "-lunwind" "-lc" "-lfdio" "-shared"
2019-09-06T16:50:59.8878082Z   = note: rust-lld: error: unable to find library -lzircon
2019-09-06T16:50:59.8879137Z           rust-lld: error: unable to find library -lfdio
2019-09-06T16:50:59.8879735Z           rust-lld: error: unable to find library -lunwind
2019-09-06T16:50:59.8880273Z           rust-lld: error: unable to find library -lc
2019-09-06T16:50:59.8880762Z           rust-lld: error: unable to find library -lfdio
2019-09-06T16:50:59.8881390Z 
2019-09-06T16:50:59.8910270Z error: aborting due to previous error
2019-09-06T16:50:59.8910586Z 
2019-09-06T16:50:59.9097941Z [RUSTC-TIMING] std test:false 18.362
---
2019-09-06T16:50:59.9230378Z == clock drift check ==
2019-09-06T16:50:59.9254559Z   local time: Fri Sep  6 16:50:59 UTC 2019
2019-09-06T16:50:59.9701326Z   network time: Fri, 06 Sep 2019 16:50:59 GMT
2019-09-06T16:50:59.9705742Z == end clock drift check ==
2019-09-06T16:51:02.6465089Z ##[error]Bash exited with code '1'.
2019-09-06T16:51:02.6507644Z ##[section]Starting: Upload CPU usage statistics
2019-09-06T16:51:02.6515619Z ==============================================================================
2019-09-06T16:51:02.6515723Z Task         : Bash
2019-09-06T16:51:02.6515787Z Description  : Run a Bash script on macOS, Linux, or Windows
