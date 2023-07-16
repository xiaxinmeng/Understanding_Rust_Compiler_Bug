plain
DirectMap4k:      264128 kB
DirectMap2M:     4978688 kB
DirectMap1G:    55574528 kB
    Finished dev [unoptimized] target(s) in 0.21s
*** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo': double free or corruption (!prev): 0x0000562926d5b530 ***
======= Backtrace: =========
/lib/x86_64-linux-gnu/libc.so.6(+0x777f5)[0x7f7f29eb37f5]
/lib/x86_64-linux-gnu/libc.so.6(+0x8038a)[0x7f7f29ebc38a]
/lib/x86_64-linux-gnu/libc.so.6(cfree+0x4c)[0x7f7f29ec058c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b8391)[0x562925044391]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9c5af4)[0x562925051af4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b139f)[0x56292503d39f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b6683)[0x562925042683]
/lib/x86_64-linux-gnu/libc.so.6(+0x3a008)[0x7f7f29e76008]
/lib/x86_64-linux-gnu/libc.so.6(+0x3a055)[0x7f7f29e76055]
/lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf7)[0x7f7f29e5c847]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x176f41)[0x562924802f41]
======= Memory map: ========
56292468c000-56292548b000 r-xp 00000000 08:01 13950927                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
56292568a000-562925730000 r--p 00dfe000 08:01 13950927                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
562925730000-562925738000 rw-p 00ea4000 08:01 13950927                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
562925738000-56292573d000 rw-p 00000000 00:00 0 
562926d58000-562927044000 rw-p 00000000 00:00 0                          [heap]
7f7f24000000-7f7f24021000 rw-p 00000000 00:00 0 
7f7f24021000-7f7f28000000 ---p 00000000 00:00 0 
7f7f29e3c000-7f7f29ffc000 r-xp 00000000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f7f29ffc000-7f7f2a1fc000 ---p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f7f2a1fc000-7f7f2a200000 r--p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f7f2a200000-7f7f2a202000 rw-p 001c4000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f7f2a202000-7f7f2a206000 rw-p 00000000 00:00 0 
7f7f2a206000-7f7f2a209000 r-xp 00000000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f7f2a209000-7f7f2a408000 ---p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f7f2a408000-7f7f2a409000 r--p 00002000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f7f2a409000-7f7f2a40a000 rw-p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f7f2a40a000-7f7f2a512000 r-xp 00000000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f7f2a512000-7f7f2a711000 ---p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f7f2a711000-7f7f2a712000 r--p 00107000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f7f2a712000-7f7f2a713000 rw-p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f7f2a713000-7f7f2a72b000 r-xp 00000000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f7f2a72b000-7f7f2a92a000 ---p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f7f2a92a000-7f7f2a92b000 r--p 00017000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f7f2a92b000-7f7f2a92c000 rw-p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f7f2a92c000-7f7f2a930000 rw-p 00000000 00:00 0 
7f7f2a930000-7f7f2a937000 r-xp 00000000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f7f2a937000-7f7f2ab36000 ---p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f7f2ab36000-7f7f2ab37000 r--p 00006000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f7f2ab37000-7f7f2ab38000 rw-p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f7f2ab38000-7f7f2ab4e000 r-xp 00000000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f7f2ab4e000-7f7f2ad4d000 ---p 00016000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f7f2ad4d000-7f7f2ad4e000 rw-p 00015000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f7f2ad4e000-7f7f2ad74000 r-xp 00000000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f7f2af68000-7f7f2af6e000 rw-p 00000000 00:00 0 
7f7f2af72000-7f7f2af73000 rw-p 00000000 00:00 0 
7f7f2af73000-7f7f2af74000 r--p 00025000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f7f2af74000-7f7f2af75000 rw-p 00026000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f7f2af75000-7f7f2af76000 rw-p 00000000 00:00 0 
7ffdfe12f000-7ffdfe150000 rw-p 00000000 00:00 0                          [stack]
7ffdfe1bd000-7ffdfe1c0000 r--p 00000000 00:00 0                          [vvar]
7ffdfe1c0000-7ffdfe1c1000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--no-deps" "--manifest-path" "/checkout/Cargo.toml"
expected success, got: signal: 6 (core dumped)', src/bootstrap/metadata.rs:39:18
Build completed unsuccessfully in 0:00:00
