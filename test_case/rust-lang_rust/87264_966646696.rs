plain
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
[RUSTC-TIMING] cfg_if test:false 0.035
[RUSTC-TIMING] adler test:false 0.295
*** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc': corrupted double-linked list: 0x00007efdc8010ef0 ***
======= Backtrace: =========
/lib/x86_64-linux-gnu/libc.so.6(+0x777f5)[0x7efde70e67f5]
/lib/x86_64-linux-gnu/libc.so.6(+0x8298b)[0x7efde70f198b]
/lib/x86_64-linux-gnu/libc.so.6(__libc_malloc+0x54)[0x7efde70f31d4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x41794b2)[0x7efdeb9434b2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x3eb8ed1)[0x7efdeb682ed1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x3ec211e)[0x7efdeb68c11e]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x3eba648)[0x7efdeb684648]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x3e7e90b)[0x7efdeb64890b]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x3e84cc3)[0x7efdeb64ecc3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x328255f)[0x7efdeaa4c55f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x16ce2ba)[0x7efde8e982ba]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x328453d)[0x7efdeaa4e53d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x2f89b36)[0x7efdea753b36]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x13f8741)[0x7efde8bc2741]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x1260aee)[0x7efde8a2aaee]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x1294cd2)[0x7efde8a5ecd2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x128e8c8)[0x7efde8a588c8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x130f7f6)[0x7efde8ad97f6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-97dbda54023f838b.so(+0x1354f11)[0x7efde8b1ef11]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-a2d927ecb5dbac51.so(rust_metadata_std_f874550ecfaf23b8+0xbace3)[0x7efde74f3ce3]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x76ba)[0x7efde6c556ba]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x6d)[0x7efde717651d]
======= Memory map: ========
55979ef5c000-55979ef5d000 r-xp 00000000 08:11 14457884                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
55979f15c000-55979f15d000 r--p 00000000 08:11 14457884                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
55979f15d000-55979f15e000 rw-p 00001000 08:11 14457884                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
5597a0131000-5597a01e9000 rw-p 00000000 00:00 0                          [heap]
7efda8000000-7efda8159000 rw-p 00000000 00:00 0 
7efda8159000-7efdac000000 ---p 00000000 00:00 0 
7efdac000000-7efdac021000 rw-p 00000000 00:00 0 
7efdac021000-7efdb0000000 ---p 00000000 00:00 0 
7efdb0000000-7efdb0196000 rw-p 00000000 00:00 0 
7efdb0196000-7efdb4000000 ---p 00000000 00:00 0 
7efdb4000000-7efdb411e000 rw-p 00000000 00:00 0 
7efdb411e000-7efdb8000000 ---p 00000000 00:00 0 
7efdb8000000-7efdb8169000 rw-p 00000000 00:00 0 
7efdb8169000-7efdbc000000 ---p 00000000 00:00 0 
7efdbc000000-7efdbc13f000 rw-p 00000000 00:00 0 
7efdbc13f000-7efdc0000000 ---p 00000000 00:00 0 
7efdc0000000-7efdc018a000 rw-p 00000000 00:00 0 
7efdc018a000-7efdc4000000 ---p 00000000 00:00 0 
7efdc4000000-7efdc4161000 rw-p 00000000 00:00 0 
7efdc4161000-7efdc8000000 ---p 00000000 00:00 0 
7efdc8000000-7efdc85ed000 rw-p 00000000 00:00 0 
7efdc85ed000-7efdcc000000 ---p 00000000 00:00 0 
7efdcc000000-7efdcc1a2000 rw-p 00000000 00:00 0 
7efdcc1a2000-7efdd0000000 ---p 00000000 00:00 0 
7efdd0000000-7efdd0098000 rw-p 00000000 00:00 0 
7efdd0098000-7efdd4000000 ---p 00000000 00:00 0 
7efdd4000000-7efdd41b3000 rw-p 00000000 00:00 0 
7efdd41b3000-7efdd8000000 ---p 00000000 00:00 0 
7efdd8000000-7efdd8021000 rw-p 00000000 00:00 0 
7efdd8021000-7efddc000000 ---p 00000000 00:00 0 
7efddcb64000-7efddcb65000 ---p 00000000 00:00 0 
7efddcb65000-7efddcd65000 rw-p 00000000 00:00 0 
7efddcd65000-7efddcd66000 ---p 00000000 00:00 0 
7efddcd66000-7efddcf66000 rw-p 00000000 00:00 0 
7efddcf66000-7efddcf67000 ---p 00000000 00:00 0 
7efddcf67000-7efddd167000 rw-p 00000000 00:00 0 
7efddd167000-7efddd168000 ---p 00000000 00:00 0 
7efddd168000-7efddd368000 rw-p 00000000 00:00 0 
7efde0000000-7efde1cfe000 rw-p 00000000 00:00 0 
7efde1cfe000-7efde4000000 ---p 00000000 00:00 0 
7efde40ea000-7efde40eb000 ---p 00000000 00:00 0 
7efde40eb000-7efde42eb000 rw-p 00000000 00:00 0 
7efde42eb000-7efde42ec000 ---p 00000000 00:00 0 
7efde42ec000-7efde44ec000 rw-p 00000000 00:00 0 
7efde44ec000-7efde44ed000 ---p 00000000 00:00 0 
7efde44ed000-7efde46ed000 rw-p 00000000 00:00 0 
7efde46ed000-7efde46ee000 ---p 00000000 00:00 0 
7efde46ee000-7efde48ee000 rw-p 00000000 00:00 0 
7efde48ee000-7efde48ef000 ---p 00000000 00:00 0 
7efde48ef000-7efde4aef000 rw-p 00000000 00:00 0 
7efde4aef000-7efde4af0000 ---p 00000000 00:00 0 
7efde4af0000-7efde4cf0000 rw-p 00000000 00:00 0 
7efde4cf0000-7efde4cf1000 ---p 00000000 00:00 0 
7efde4cf1000-7efde4ef1000 rw-p 00000000 00:00 0 
7efde4ef1000-7efde4ef2000 ---p 00000000 00:00 0 
7efde4ef2000-7efde50f2000 rw-p 00000000 00:00 0 
7efde50f2000-7efde50f3000 ---p 00000000 00:00 0 
7efde50f3000-7efde53f4000 rw-p 00000000 00:00 0 
7efde578a000-7efde578b000 ---p 00000000 00:00 0 
7efde578b000-7efde5f8b000 rw-p 00000000 00:00 0 
7efde5f8b000-7efde5f92000 r-xp 00000000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7efde5f92000-7efde6191000 ---p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7efde6191000-7efde6192000 r--p 00006000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7efde6192000-7efde6193000 rw-p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7efde6193000-7efde61a9000 r-xp 00000000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7efde61a9000-7efde63a8000 ---p 00016000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7efde63a8000-7efde63a9000 rw-p 00015000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7efde63a9000-7efde651b000 r-xp 00000000 00:35 4129810                    /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.21
7efde651b000-7efde671b000 ---p 00172000 00:35 4129810                    /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.21
7efde671b000-7efde6725000 r--p 00172000 00:35 4129810                    /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.21
7efde6725000-7efde6727000 rw-p 0017c000 00:35 4129810                    /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.21
7efde6727000-7efde672b000 rw-p 00000000 00:00 0 
7efde672b000-7efde6744000 r-xp 00000000 00:35 4129008                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7efde6744000-7efde6943000 ---p 00019000 00:35 4129008                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7efde6943000-7efde6944000 r--p 00018000 00:35 4129008                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7efde6944000-7efde6945000 rw-p 00019000 00:35 4129008                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7efde6945000-7efde6a4d000 r-xp 00000000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7efde6a4d000-7efde6c4c000 ---p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7efde6c4c000-7efde6c4d000 r--p 00107000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7efde6c4d000-7efde6c4e000 rw-p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7efde6c4e000-7efde6c66000 r-xp 00000000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7efde6c66000-7efde6e65000 ---p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7efde6e65000-7efde6e66000 r--p 00017000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7efde6e66000-7efde6e67000 rw-p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7efde6e67000-7efde6e6b000 rw-p 00000000 00:00 0 
7efde6e6b000-7efde6e6e000 r-xp 00000000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7efde6e6e000-7efde706d000 ---p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7efde706d000-7efde706e000 r--p 00002000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7efde706e000-7efde706f000 rw-p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7efde706f000-7efde722f000 r-xp 00000000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7efde722f000-7efde742f000 ---p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7efde742f000-7efde7433000 r--p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7efde7433000-7efde7435000 rw-p 001c4000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7efde7435000-7efde7439000 rw-p 00000000 00:00 0 
7efde7439000-7efde75b9000 r-xp 00000000 08:11 14196198                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-a2d927ecb5dbac51.so
7efde75b9000-7efde77b8000 ---p 00180000 08:11 14196198                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-a2d927ecb5dbac51.so
7efde77b8000-7efde77c9000 r--p 0017f000 08:11 14196198                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-a2d927ecb5dbac51.so
7efde77c9000-7efde77ca000 rw-p 00190000 08:11 14196198                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-a2d927ecb5dbac51.so
7efde77ca000-7efdf212c000 r-xp 00000000 08:11 14457894                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-97dbda54023f838b.so
7efdf212c000-7efdf232b000 ---p 0a962000 08:11 14457894                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-97dbda54023f838b.so
7efdf232b000-7efdf2b05000 r--p 0a961000 08:11 14457894                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-97dbda54023f838b.so
7efdf2b05000-7efdf2b30000 rw-p 0b13b000 08:11 14457894                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-97dbda54023f838b.so
7efdf2b30000-7efdf2ba4000 rw-p 00000000 00:00 0 
7efdf2ba4000-7efdf2bca000 r-xp 00000000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7efdf2c7c000-7efdf2c7d000 ---p 00000000 00:00 0 
7efdf2c7d000-7efdf2c7f000 rw-p 00000000 00:00 0 
7efdf2c7f000-7efdf2c80000 ---p 00000000 00:00 0 
7efdf2c80000-7efdf2c82000 rw-p 00000000 00:00 0 
7efdf2d66000-7efdf2d67000 ---p 00000000 00:00 0 
7efdf2d67000-7efdf2d69000 rw-p 00000000 00:00 0 
7efdf2d69000-7efdf2d6a000 ---p 00000000 00:00 0 
7efdf2d6a000-7efdf2d6c000 rw-p 00000000 00:00 0 
7efdf2d6c000-7efdf2d6d000 ---p 00000000 00:00 0 
7efdf2d6d000-7efdf2d6f000 rw-p 00000000 00:00 0 
7efdf2d6f000-7efdf2d70000 ---p 00000000 00:00 0 
7efdf2d70000-7efdf2d72000 rw-p 00000000 00:00 0 
7efdf2d72000-7efdf2d73000 ---p 00000000 00:00 0 
7efdf2d73000-7efdf2d75000 rw-p 00000000 00:00 0 
7efdf2d75000-7efdf2d76000 ---p 00000000 00:00 0 
7efdf2d76000-7efdf2d78000 rw-p 00000000 00:00 0 
7efdf2d7b000-7efdf2d7c000 ---p 00000000 00:00 0 
7efdf2d7c000-7efdf2d7e000 rw-p 00000000 00:00 0 
7efdf2d7e000-7efdf2d7f000 ---p 00000000 00:00 0 
7efdf2d7f000-7efdf2d81000 rw-p 00000000 00:00 0 
7efdf2d81000-7efdf2d82000 ---p 00000000 00:00 0 
7efdf2d82000-7efdf2db9000 rw-p 00000000 00:00 0 
7efdf2db9000-7efdf2dc2000 rw-p 00000000 00:00 0 
7efdf2dc2000-7efdf2dc3000 ---p 00000000 00:00 0 
7efdf2dc3000-7efdf2dc5000 rw-p 00000000 00:00 0 
7efdf2dc5000-7efdf2dc6000 ---p 00000000 00:00 0 
7efdf2dc6000-7efdf2dc9000 rw-p 00000000 00:00 0 
7efdf2dc9000-7efdf2dca000 r--p 00025000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7efdf2dca000-7efdf2dcb000 rw-p 00026000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7efdf2dcb000-7efdf2dcc000 rw-p 00000000 00:00 0 
7ffef0bcc000-7ffef0bee000 rw-p 00000000 00:00 0                          [stack]
7ffef0bf3000-7ffef0bf6000 r--p 00000000 00:00 0                          [vvar]
7ffef0bf6000-7ffef0bf7000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
[RUSTC-TIMING] compiler_builtins test:false 1.288
[RUSTC-TIMING] compiler_builtins test:false 1.288
rustc exited with signal: 6 (core dumped)
error: could not compile `compiler_builtins`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.49/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=0 --cfg 'feature="c"' --cfg 'feature="cc"' --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=e2ece3328e63625c -C extra-filename=-e2ece3328e63625c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-2deeba5fb4de62b0.rmeta --cap-lints allow -Zsymbol-mangling-version=legacy -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-b394ac4ad7d471eb/out --cfg 'feature="unstable"' --cfg '__absvdi2="optimized-c"' --cfg '__absvsi2="optimized-c"' --cfg '__absvti2="optimized-c"' --cfg '__addvdi3="optimized-c"' --cfg '__addvsi3="optimized-c"' --cfg '__addvti3="optimized-c"' --cfg '__clzdi2="optimized-c"' --cfg '__clzsi2="optimized-c"' --cfg '__clzti2="optimized-c"' --cfg '__cmpdi2="optimized-c"' --cfg '__cmpti2="optimized-c"' --cfg '__ctzdi2="optimized-c"' --cfg '__ctzsi2="optimized-c"' --cfg '__ctzti2="optimized-c"' --cfg '__divdc3="optimized-c"' --cfg '__divsc3="optimized-c"' --cfg '__divxc3="optimized-c"' --cfg '__extendhfsf2="optimized-c"' --cfg '__ffsti2="optimized-c"' --cfg '__floatdisf="optimized-c"' --cfg '__floatdixf="optimized-c"' --cfg '__floatundidf="optimized-c"' --cfg '__floatundisf="optimized-c"' --cfg '__floatundixf="optimized-c"' --cfg '__int_util="optimized-c"' --cfg '__muldc3="optimized-c"' --cfg '__mulsc3="optimized-c"' --cfg '__mulvdi3="optimized-c"' --cfg '__mulvsi3="optimized-c"' --cfg '__mulvti3="optimized-c"' --cfg '__mulxc3="optimized-c"' --cfg '__negdf2="optimized-c"' --cfg '__negdi2="optimized-c"' --cfg '__negsf2="optimized-c"' --cfg '__negti2="optimized-c"' --cfg '__negvdi2="optimized-c"' --cfg '__negvsi2="optimized-c"' --cfg '__negvti2="optimized-c"' --cfg '__paritydi2="optimized-c"' --cfg '__paritysi2="optimized-c"' --cfg '__parityti2="optimized-c"' --cfg '__popcountdi2="optimized-c"' --cfg '__popcountsi2="optimized-c"' --cfg '__popcountti2="optimized-c"' --cfg '__powixf2="optimized-c"' --cfg '__subvdi3="optimized-c"' --cfg '__subvsi3="optimized-c"' --cfg '__subvti3="optimized-c"' --cfg '__truncdfhf2="optimized-c"' --cfg '__truncdfsf2="optimized-c"' --cfg '__truncsfhf2="optimized-c"' --cfg '__ucmpdi2="optimized-c"' --cfg '__ucmpti2="optimized-c"' --cfg 'apple_versioning="optimized-c"' -l static=compiler-rt` (exit status: 254)
[RUSTC-TIMING] libc test:false 1.291
[RUSTC-TIMING] memchr test:false 2.708
[RUSTC-TIMING] rustc_demangle test:false 2.971
[RUSTC-TIMING] alloc test:false 4.485
[RUSTC-TIMING] alloc test:false 4.485
[RUSTC-TIMING] core test:false 32.091
error: build failed
Build completed unsuccessfully in 0:07:24
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
