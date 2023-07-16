plain
tidy check
[00:04:44] * 539 error codes
[00:04:44] * highest error code: E0911
[00:04:45] * 197 features
[00:04:45] *** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy': munmap_chunk(): invalid pointer: 0x0000555d328c38c0 ***
[00:04:45] ======= Backtrace: =========
[00:04:45] /lib/x86_64-linux-gnu/libc.so.6(+0x777e5)[0x7fea638f07e5]
[00:04:45] /lib/x86_64-linux-gnu/libc.so.6(cfree+0x1a8)[0x7fea638fd698]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x2b54c)[0x555d3286b54c]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x160dc)[0x555d328560dc]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x12f94)[0x555d32852f94]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x12923)[0x555d32852923]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x49383)[0x555d32889383]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x6bfda)[0x555d328abfda]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x49278)[0x555d32889278]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x54ee1)[0x555d32894ee1]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x13324)[0x555d32853324]
[00:04:45] /lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf0)[0x7fea63899830]
[00:04:45] /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy(+0x12159)[0x555d32852159]
[00:04:45] ======= Memory map: ========
[00:04:45] 555d32840000-555d328de000 r-xp 00000000 08:01 801431                     /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy
[00:04:45] 555d32ade000-555d32aec000 r--p 0009e000 08:01 801431                     /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy
[00:04:45] 555d32aec000-555d32aed000 rw-p 000ac000 08:01 801431                     /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy
[00:04:45] 555d33f64000-555d34083000 rw-p 00000000 00:00 0                          [heap]
[00:04:45] 7fea63879000-7fea63a39000 r-xp 00000000 08:01 1190520                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:04:45] 7fea63a39000-7fea63c39000 ---p 001c0000 08:01 1190520                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:04:45] 7fea63c39000-7fea63c3d000 r--p 001c0000 08:01 1190520                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:04:45] 7fea63c3d000-7fea63c3f000 rw-p 001c4000 08:01 1190520                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:04:45] 7fea63c3f000-7fea63c43000 rw-p 00000000 00:00 0 
[00:04:45] 7fea63c43000-7fea63c59000 r-xp 00000000 08:01 1190541                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:04:45] 7fea63c59000-7fea63e58000 ---p 00016000 08:01 1190541                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:04:45] 7fea63e58000-7fea63e59000 rw-p 00015000 08:01 1190541                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:04:45] 7fea63e59000-7fea63e71000 r-xp 00000000 08:01 1190588                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:04:45] 7fea63e71000-7fea64070000 ---p 00018000 08:01 1190588                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:04:45] 7fea64070000-7fea64071000 r--p 00017000 08:01 1190588                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:04:45] 7fea64071000-7fea64072000 rw-p 00018000 08:01 1190588                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:04:45] 7fea64072000-7fea64076000 rw-p 00000000 00:00 0 
[00:04:45] 7fea64076000-7fea6407d000 r-xp 00000000 08:01 1190594                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:04:45] 7fea6407d000-7fea6427c000 ---p 00007000 08:01 1190594                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:04:45] 7fea6427c000-7fea6427d000 r--p 00006000 08:01 1190594                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:04:45] 7fea6427d000-7fea6427e000 rw-p 00007000 08:01 1190594                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:04:45] 7fea6427e000-7fea64281000 r-xp 00000000 08:01 1190533                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:04:45] 7fea64281000-7fea64480000 ---p 00003000 08:01 1190533                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:04:45] 7fea64480000-7fea64481000 r--p 00002000 08:01 1190533                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:04:45] 7fea64481000-7fea64482000 rw-p 00003000 08:01 1190533                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:04:45] 7fea64482000-7fea644a8000 r-xp 00000000 08:01 1190500                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:04:45] 7fea6469d000-7fea646a1000 rw-p 00000000 00:00 0 
[00:04:45] 7fea646a3000-7fea646a7000 rw-p 00000000 00:00 0 
[00:04:45] 7fea646a7000-7fea646a8000 r--p 00025000 08:01 1190500                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:04:45] 7fea646a8000-7fea646a9000 rw-p 00026000 08:01 1190500                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:04:45] 7fea646a9000-7fea646aa000 rw-p 00000000 00:00 0 
[00:04:45] 7ffc74218000-7ffc74239000 rw-p 00000000 00:00 0                          [stack]
[00:04:45] 7ffc74348000-7ffc7434a000 r--p 00000000 00:00 0                          [vvar]
[00:04:45] 7ffc7434a000-7ffc7434c000 r-xp 00000000 00:00 0                          [vdso]
[00:04:45] ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
[00:04:45] 
[00:04:45] 
[00:04:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:45] expected success, got: signal: 6
[00:04:45] 
[00:04:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:45] Build completed unsuccessfully in 0:01:53
[00:04:45] Build completed unsuccessfully in 0:01:53
[00:04:45] Makefile:79: recipe for target 'tidy' failed
[00:04:45] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e19b484
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
