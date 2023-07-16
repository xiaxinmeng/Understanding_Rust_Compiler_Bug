plain
tidy check
[00:05:28] * 539 error codes
[00:05:28] * highest error code: E0911
[00:05:28] * 197 features
[00:05:29] *** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy': munmap_chunk(): invalid pointer: 0x000055d995d7a4a0 ***
[00:05:29] ======= Backtrace: =========
[00:05:29] /lib/x86_64-linux-gnu/libc.so.6(+0x777e5)[0x7fa3d31257e5]
[00:05:29] /lib/x86_64-linux-gnu/libc.so.6(cfree+0x1a8)[0x7fa3d3132698]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x22726)[0x55d995d1a726]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x292eb)[0x55d995d212eb]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x12b64)[0x55d995d0ab64]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x124f3)[0x55d995d0a4f3]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x480d3)[0x55d995d400d3]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x6ab6a)[0x55d995d62b6a]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x47fc8)[0x55d995d3ffc8]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x53c21)[0x55d995d4bc21]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x12ef4)[0x55d995d0aef4]
[00:05:29] /lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf0)[0x7fa3d30ce830]
[00:05:29] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x11d29)[0x55d995d09d29]
[00:05:29] ======= Memory map: ========
[00:05:29] 55d995cf8000-55d995d95000 r-xp 00000000 08:01 801001                     /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy
[00:05:29] 55d995f94000-55d995fa2000 r--p 0009c000 08:01 801001                     /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy
[00:05:29] 55d995fa2000-55d995fa3000 rw-p 000aa000 08:01 801001                     /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy
[00:05:29] 55d9973a5000-55d9974c4000 rw-p 00000000 00:00 0                          [heap]
[00:05:29] 7fa3d30ae000-7fa3d326e000 r-xp 00000000 08:01 1191392                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:05:29] 7fa3d326e000-7fa3d346e000 ---p 001c0000 08:01 1191392                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:05:29] 7fa3d346e000-7fa3d3472000 r--p 001c0000 08:01 1191392                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:05:29] 7fa3d3472000-7fa3d3474000 rw-p 001c4000 08:01 1191392                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:05:29] 7fa3d3474000-7fa3d3478000 rw-p 00000000 00:00 0 
[00:05:29] 7fa3d3478000-7fa3d348e000 r-xp 00000000 08:01 1191413                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:05:29] 7fa3d348e000-7fa3d368d000 ---p 00016000 08:01 1191413                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:05:29] 7fa3d368d000-7fa3d368e000 rw-p 00015000 08:01 1191413                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:05:29] 7fa3d368e000-7fa3d36a6000 r-xp 00000000 08:01 1191460                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:05:29] 7fa3d36a6000-7fa3d38a5000 ---p 00018000 08:01 1191460                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:05:29] 7fa3d38a5000-7fa3d38a6000 r--p 00017000 08:01 1191460                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:05:29] 7fa3d38a6000-7fa3d38a7000 rw-p 00018000 08:01 1191460                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:05:29] 7fa3d38a7000-7fa3d38ab000 rw-p 00000000 00:00 0 
[00:05:29] 7fa3d38ab000-7fa3d38b2000 r-xp 00000000 08:01 1191466                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:05:29] 7fa3d38b2000-7fa3d3ab1000 ---p 00007000 08:01 1191466                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:05:29] 7fa3d3ab1000-7fa3d3ab2000 r--p 00006000 08:01 1191466                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:05:29] 7fa3d3ab2000-7fa3d3ab3000 rw-p 00007000 08:01 1191466                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:05:29] 7fa3d3ab3000-7fa3d3ab6000 r-xp 00000000 08:01 1191405                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:05:29] 7fa3d3ab6000-7fa3d3cb5000 ---p 00003000 08:01 1191405                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:05:29] 7fa3d3cb5000-7fa3d3cb6000 r--p 00002000 08:01 1191405                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:05:29] 7fa3d3cb6000-7fa3d3cb7000 rw-p 00003000 08:01 1191405                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:05:29] 7fa3d3cb7000-7fa3d3cdd000 r-xp 00000000 08:01 1191372                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:05:29] 7fa3d3ed2000-7fa3d3ed6000 rw-p 00000000 00:00 0 
[00:05:29] 7fa3d3ed8000-7fa3d3edc000 rw-p 00000000 00:00 0 
[00:05:29] 7fa3d3edc000-7fa3d3edd000 r--p 00025000 08:01 1191372                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:05:29] 7fa3d3edd000-7fa3d3ede000 rw-p 00026000 08:01 1191372                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:05:29] 7fa3d3ede000-7fa3d3edf000 rw-p 00000000 00:00 0 
[00:05:29] 7ffe7f741000-7ffe7f762000 rw-p 00000000 00:00 0                          [stack]
[00:05:29] 7ffe7f77f000-7ffe7f781000 r--p 00000000 00:00 0                          [vvar]
[00:05:29] 7ffe7f781000-7ffe7f783000 r-xp 00000000 00:00 0                          [vdso]
[00:05:29] ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
[00:05:29] 
[00:05:29] 
[00:05:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:29] expected success, got: signal: 6
[00:05:29] 
[00:05:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:29] Build completed unsuccessfully in 0:02:27
[00:05:29] Build completed unsuccessfully in 0:02:27
[00:05:29] Makefile:79: recipe for target 'tidy' failed
[00:05:29] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06774cb0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
