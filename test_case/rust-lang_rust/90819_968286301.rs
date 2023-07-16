plain
downloading https://static.rust-lang.org/dist/2021-10-23/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-10-23/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
*** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo': malloc(): memory corruption (fast): 0x0000556f29621000 ***
======= Backtrace: =========
/lib/x86_64-linux-gnu/libc.so.6(+0x777f5)[0x7f24c70777f5]
/lib/x86_64-linux-gnu/libc.so.6(+0x82679)[0x7f24c7082679]
/lib/x86_64-linux-gnu/libc.so.6(__libc_malloc+0x54)[0x7f24c70841d4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b8601)[0x556f28f4d601]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x99ecdf)[0x556f28f33cdf]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0xa1c78c)[0x556f28fb178c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x931bf6)[0x556f28ec6bf6]
/lib/x86_64-linux-gnu/libpthread.so.0(+0xea99)[0x7f24c78e5a99]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9feff6)[0x556f28f93ff6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(OPENSSL_init_ssl+0xcc)[0x556f28ec69ec]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN3std4sync4once4Once10call_inner17h268a40483d9482e6E+0x2ce)[0x556f2870a73e]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN11openssl_sys4init17h9c980fbb04839f6bE+0x4b)[0x556f28eb61ab]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x818639)[0x556f28dad639]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN3std4sync4once4Once10call_inner17h268a40483d9482e6E+0x2ce)[0x556f2870a73e]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN11libgit2_sys4init17h16c4b0b1b323743dE+0x40)[0x556f28d2cc60]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN4git26config6Config12open_default17hb8d0cb2dbd74dd1cE+0x1f)[0x556f28d2144f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x23b5a2)[0x556f287d05a2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN5cargo3ops8registry27needs_custom_http_transport17haa600f78fff05729E+0x16)[0x556f287cf626]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x1ea6de)[0x556f2877f6de]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x1a66cd)[0x556f2873b6cd]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x1e8294)[0x556f2877d294]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x1beaa3)[0x556f28753aa3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x1dffc9)[0x556f28774fc9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN3std2rt19lang_start_internal17h9f6868747dc491d4E+0x431)[0x556f29062731]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x1ea752)[0x556f2877f752]
/lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf0)[0x7f24c7020840]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x176f41)[0x556f2870bf41]
======= Memory map: ========
556f28595000-556f29394000 r-xp 00000000 08:01 14194982                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
556f29593000-556f29639000 r--p 00dfe000 08:01 14194982                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
556f29639000-556f29641000 rw-p 00ea4000 08:01 14194982                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
556f29641000-556f29646000 rw-p 00000000 00:00 0 
556f2ad58000-556f2add3000 rw-p 00000000 00:00 0                          [heap]
7f24c0000000-7f24c0021000 rw-p 00000000 00:00 0 
7f24c0021000-7f24c4000000 ---p 00000000 00:00 0 
7f24c7000000-7f24c71c0000 r-xp 00000000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f24c71c0000-7f24c73c0000 ---p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f24c73c0000-7f24c73c4000 r--p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f24c73c4000-7f24c73c6000 rw-p 001c4000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f24c73c6000-7f24c73ca000 rw-p 00000000 00:00 0 
7f24c73ca000-7f24c73cd000 r-xp 00000000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f24c73cd000-7f24c75cc000 ---p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f24c75cc000-7f24c75cd000 r--p 00002000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f24c75cd000-7f24c75ce000 rw-p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f24c75ce000-7f24c76d6000 r-xp 00000000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f24c76d6000-7f24c78d5000 ---p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f24c78d5000-7f24c78d6000 r--p 00107000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f24c78d6000-7f24c78d7000 rw-p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f24c78d7000-7f24c78ef000 r-xp 00000000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f24c78ef000-7f24c7aee000 ---p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f24c7aee000-7f24c7aef000 r--p 00017000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f24c7aef000-7f24c7af0000 rw-p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f24c7af0000-7f24c7af4000 rw-p 00000000 00:00 0 
7f24c7af4000-7f24c7afb000 r-xp 00000000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f24c7afb000-7f24c7cfa000 ---p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f24c7cfa000-7f24c7cfb000 r--p 00006000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f24c7cfb000-7f24c7cfc000 rw-p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f24c7cfc000-7f24c7d12000 r-xp 00000000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f24c7d12000-7f24c7f11000 ---p 00016000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f24c7f11000-7f24c7f12000 rw-p 00015000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f24c7f12000-7f24c7f38000 r-xp 00000000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f24c812b000-7f24c8131000 rw-p 00000000 00:00 0 
7f24c8133000-7f24c8134000 rw-p 00000000 00:00 0 
7f24c8134000-7f24c8135000 ---p 00000000 00:00 0 
7f24c8135000-7f24c8137000 rw-p 00000000 00:00 0 
7f24c8137000-7f24c8138000 r--p 00025000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f24c8138000-7f24c8139000 rw-p 00026000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f24c8139000-7f24c813a000 rw-p 00000000 00:00 0 
7ffc18bb0000-7ffc18bd1000 rw-p 00000000 00:00 0                          [stack]
7ffc18be9000-7ffc18bec000 r--p 00000000 00:00 0                          [vvar]
7ffc18bec000-7ffc18bed000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
Build completed unsuccessfully in 0:00:21
