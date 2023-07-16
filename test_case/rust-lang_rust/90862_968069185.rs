plain
   Compiling unicode-width v0.1.8
error: could not compile `libc`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name build_script_build /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.106/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=76097fc39372545a -C extra-filename=-76097fc39372545a --out-dir /checkout/obj/build/bootstrap/debug/build/libc-76097fc39372545a -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 6, SIGABRT: process abort signal)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:55
make: *** [prepare] Error 1
---
   Compiling serde_json v1.0.59
   Compiling walkdir v2.3.1
   Compiling getopts v0.2.21
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
*** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo': corrupted double-linked list: 0x00007fb088005440 ***
======= Backtrace: =========
/lib/x86_64-linux-gnu/libc.so.6(+0x777f5)[0x7fb0dcba77f5]
/lib/x86_64-linux-gnu/libc.so.6(+0x7e6fd)[0x7fb0dcbae6fd]
/lib/x86_64-linux-gnu/libc.so.6(+0x81d0c)[0x7fb0dcbb1d0c]
/lib/x86_64-linux-gnu/libc.so.6(+0x832e0)[0x7fb0dcbb32e0]
/lib/x86_64-linux-gnu/libc.so.6(realloc+0x199)[0x7fb0dcbb48a9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0xab0b10)[0x55f20c1c6b10]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x174fd3)[0x55f20b88afd3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN3std3env7vars_os17h959933b0e0d8cc67E+0x1ed)[0x55f20c1d446d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0xad679d)[0x55f20c1ec79d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0xad872f)[0x55f20c1ee72f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN3std7process7Command5spawn17h011b8b8e94f230a2E+0x1c)[0x55f20c1e18bc]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(_ZN10cargo_util15process_builder14ProcessBuilder19exec_with_streaming17h09e92805dbed57d3E+0xf9)[0x55f20c026c19]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x252265)[0x55f20b968265]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x253ddc)[0x55f20b969ddc]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x3369c6)[0x55f20ba4c9c6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x59923e)[0x55f20bcaf23e]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x3366f3)[0x55f20ba4c6f3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0xad7e03)[0x55f20c1ede03]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x76ba)[0x7fb0dd40e6ba]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x6d)[0x7fb0dcc3751d]
======= Memory map: ========
55f20b716000-55f20c515000 r-xp 00000000 08:11 13950211                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
55f20c714000-55f20c7ba000 r--p 00dfe000 08:11 13950211                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
55f20c7ba000-55f20c7c2000 rw-p 00ea4000 08:11 13950211                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
55f20c7c2000-55f20c7c7000 rw-p 00000000 00:00 0 
55f20cac2000-55f20e005000 rw-p 00000000 00:00 0                          [heap]
7fb088000000-7fb088021000 rw-p 00000000 00:00 0 
7fb088021000-7fb08c000000 ---p 00000000 00:00 0 
7fb08c000000-7fb08c021000 rw-p 00000000 00:00 0 
7fb08c021000-7fb090000000 ---p 00000000 00:00 0 
7fb090000000-7fb090021000 rw-p 00000000 00:00 0 
7fb090021000-7fb094000000 ---p 00000000 00:00 0 
7fb094000000-7fb094021000 rw-p 00000000 00:00 0 
7fb094021000-7fb098000000 ---p 00000000 00:00 0 
7fb098000000-7fb098021000 rw-p 00000000 00:00 0 
7fb098021000-7fb09c000000 ---p 00000000 00:00 0 
7fb09c000000-7fb09c021000 rw-p 00000000 00:00 0 
7fb09c021000-7fb0a0000000 ---p 00000000 00:00 0 
7fb0a0000000-7fb0a0021000 rw-p 00000000 00:00 0 
7fb0a0021000-7fb0a4000000 ---p 00000000 00:00 0 
7fb0a4000000-7fb0a4021000 rw-p 00000000 00:00 0 
7fb0a4021000-7fb0a8000000 ---p 00000000 00:00 0 
7fb0a8000000-7fb0a8021000 rw-p 00000000 00:00 0 
7fb0a8021000-7fb0ac000000 ---p 00000000 00:00 0 
7fb0ac000000-7fb0ac021000 rw-p 00000000 00:00 0 
7fb0ac021000-7fb0b0000000 ---p 00000000 00:00 0 
7fb0b0000000-7fb0b0021000 rw-p 00000000 00:00 0 
7fb0b0021000-7fb0b4000000 ---p 00000000 00:00 0 
7fb0b4000000-7fb0b4021000 rw-p 00000000 00:00 0 
7fb0b4021000-7fb0b8000000 ---p 00000000 00:00 0 
7fb0b8000000-7fb0b8021000 rw-p 00000000 00:00 0 
7fb0b8021000-7fb0bc000000 ---p 00000000 00:00 0 
7fb0bc000000-7fb0bc021000 rw-p 00000000 00:00 0 
7fb0bc021000-7fb0c0000000 ---p 00000000 00:00 0 
7fb0c0000000-7fb0c0021000 rw-p 00000000 00:00 0 
7fb0c0021000-7fb0c4000000 ---p 00000000 00:00 0 
7fb0c8000000-7fb0c8021000 rw-p 00000000 00:00 0 
7fb0c8021000-7fb0cc000000 ---p 00000000 00:00 0 
7fb0cc000000-7fb0cc021000 rw-p 00000000 00:00 0 
7fb0cc021000-7fb0d0000000 ---p 00000000 00:00 0 
7fb0d0000000-7fb0d0021000 rw-p 00000000 00:00 0 
7fb0d0021000-7fb0d4000000 ---p 00000000 00:00 0 
7fb0d448f000-7fb0d4490000 ---p 00000000 00:00 0 
7fb0d4490000-7fb0d4690000 rw-p 00000000 00:00 0 
7fb0d4690000-7fb0d4691000 ---p 00000000 00:00 0 
7fb0d4691000-7fb0d4891000 rw-p 00000000 00:00 0 
7fb0d4891000-7fb0d4892000 ---p 00000000 00:00 0 
7fb0d4892000-7fb0d4a92000 rw-p 00000000 00:00 0 
7fb0d4a92000-7fb0d4a93000 ---p 00000000 00:00 0 
7fb0d4a93000-7fb0d4c93000 rw-p 00000000 00:00 0 
7fb0d4c93000-7fb0d4c94000 ---p 00000000 00:00 0 
7fb0d4c94000-7fb0d4e94000 rw-p 00000000 00:00 0 
7fb0d4e94000-7fb0d4e95000 ---p 00000000 00:00 0 
7fb0d4e95000-7fb0d5095000 rw-p 00000000 00:00 0 
7fb0d5095000-7fb0d5096000 ---p 00000000 00:00 0 
7fb0d5096000-7fb0d5098000 rw-p 00000000 00:00 0 
7fb0d5098000-7fb0d5099000 ---p 00000000 00:00 0 
7fb0d5099000-7fb0d509b000 rw-p 00000000 00:00 0 
7fb0d509b000-7fb0d509c000 ---p 00000000 00:00 0 
7fb0d509c000-7fb0d509e000 rw-p 00000000 00:00 0 
7fb0d509e000-7fb0d509f000 ---p 00000000 00:00 0 
7fb0d509f000-7fb0d529f000 rw-p 00000000 00:00 0 
7fb0d529f000-7fb0d52a0000 ---p 00000000 00:00 0 
7fb0d52a0000-7fb0d52a2000 rw-p 00000000 00:00 0 
7fb0d52a2000-7fb0d52a3000 ---p 00000000 00:00 0 
7fb0d52a3000-7fb0d54a3000 rw-p 00000000 00:00 0 
7fb0d54a3000-7fb0d54a4000 ---p 00000000 00:00 0 
7fb0d54a4000-7fb0d56a4000 rw-p 00000000 00:00 0 
7fb0d56a4000-7fb0d56a5000 ---p 00000000 00:00 0 
7fb0d56a5000-7fb0d58a5000 rw-p 00000000 00:00 0 
7fb0d58a5000-7fb0d58a6000 ---p 00000000 00:00 0 
7fb0d58a6000-7fb0d58a8000 rw-p 00000000 00:00 0 
7fb0d58a8000-7fb0d58a9000 ---p 00000000 00:00 0 
7fb0d58a9000-7fb0d5aa9000 rw-p 00000000 00:00 0 
7fb0d5aa9000-7fb0d5aaa000 ---p 00000000 00:00 0 
7fb0d5aaa000-7fb0d5aac000 rw-p 00000000 00:00 0 
7fb0d5aac000-7fb0d5aad000 ---p 00000000 00:00 0 
7fb0d5aad000-7fb0d5cad000 rw-p 00000000 00:00 0 
7fb0d5cb0000-7fb0d5cb1000 ---p 00000000 00:00 0 
7fb0d5cb1000-7fb0d5eb1000 rw-p 00000000 00:00 0 
7fb0d5eb1000-7fb0d5eb2000 ---p 00000000 00:00 0 
7fb0d5eb2000-7fb0d5eb4000 rw-p 00000000 00:00 0 
7fb0d5eb4000-7fb0d5eb5000 ---p 00000000 00:00 0 
7fb0d5eb5000-7fb0d60b5000 rw-p 00000000 00:00 0 
7fb0d60b5000-7fb0d60b6000 ---p 00000000 00:00 0 
7fb0d60b6000-7fb0d60b8000 rw-p 00000000 00:00 0 
7fb0d60b8000-7fb0d60b9000 ---p 00000000 00:00 0 
7fb0d60b9000-7fb0d62b9000 rw-p 00000000 00:00 0 
7fb0d62b9000-7fb0d62ba000 ---p 00000000 00:00 0 
7fb0d62ba000-7fb0d64ba000 rw-p 00000000 00:00 0 
7fb0d64ba000-7fb0d64bb000 ---p 00000000 00:00 0 
7fb0d64bb000-7fb0d66bb000 rw-p 00000000 00:00 0 
7fb0d66bb000-7fb0d66bc000 ---p 00000000 00:00 0 
7fb0d66bc000-7fb0d68bc000 rw-p 00000000 00:00 0 
7fb0d68bc000-7fb0d68bd000 ---p 00000000 00:00 0 
7fb0d68bd000-7fb0d6abd000 rw-p 00000000 00:00 0 
7fb0d6abd000-7fb0d6abe000 ---p 00000000 00:00 0 
7fb0d6abe000-7fb0d6cbe000 rw-p 00000000 00:00 0 
7fb0d6cbe000-7fb0d6cbf000 ---p 00000000 00:00 0 
7fb0d6cbf000-7fb0d6ebf000 rw-p 00000000 00:00 0 
7fb0d6ebf000-7fb0d6ec0000 ---p 00000000 00:00 0 
7fb0d6ec0000-7fb0d70c0000 rw-p 00000000 00:00 0 
7fb0d70c0000-7fb0d70c1000 ---p 00000000 00:00 0 
7fb0d70c1000-7fb0d7668000 rw-p 00000000 00:00 0 
7fb0d7668000-7fb0dc582000 r--s 00000000 08:11 13950221                   /cargo/registry/index/github.com-1ecc6299db9ec823/.git/objects/pack/pack-95afb929b26a4baf9757cd6d61e991441123bac8.pack
7fb0dc582000-7fb0dcb30000 r--s 00000000 08:11 13950222                   /cargo/registry/index/github.com-1ecc6299db9ec823/.git/objects/pack/pack-95afb929b26a4baf9757cd6d61e991441123bac8.idx
7fb0dcb30000-7fb0dccf0000 r-xp 00000000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7fb0dccf0000-7fb0dcef0000 ---p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7fb0dcef0000-7fb0dcef4000 r--p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7fb0dcef4000-7fb0dcef6000 rw-p 001c4000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7fb0dcef6000-7fb0dcefa000 rw-p 00000000 00:00 0 
7fb0dcefa000-7fb0dcefd000 r-xp 00000000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7fb0dcefd000-7fb0dd0fc000 ---p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7fb0dd0fc000-7fb0dd0fd000 r--p 00002000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7fb0dd0fd000-7fb0dd0fe000 rw-p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7fb0dd0fe000-7fb0dd206000 r-xp 00000000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7fb0dd206000-7fb0dd405000 ---p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7fb0dd405000-7fb0dd406000 r--p 00107000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7fb0dd406000-7fb0dd407000 rw-p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7fb0dd407000-7fb0dd41f000 r-xp 00000000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7fb0dd41f000-7fb0dd61e000 ---p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7fb0dd61e000-7fb0dd61f000 r--p 00017000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7fb0dd61f000-7fb0dd620000 rw-p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7fb0dd620000-7fb0dd624000 rw-p 00000000 00:00 0 
7fb0dd624000-7fb0dd62b000 r-xp 00000000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7fb0dd62b000-7fb0dd82a000 ---p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7fb0dd82a000-7fb0dd82b000 r--p 00006000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7fb0dd82b000-7fb0dd82c000 rw-p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7fb0dd82c000-7fb0dd842000 r-xp 00000000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7fb0dd842000-7fb0dda41000 ---p 00016000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7fb0dda41000-7fb0dda42000 rw-p 00015000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7fb0dda42000-7fb0dda68000 r-xp 00000000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7fb0dda68000-7fb0dda69000 rw-p 00000000 00:00 0 
7fb0dda69000-7fb0dda6a000 ---p 00000000 00:00 0 
7fb0dda6a000-7fb0dda6c000 rw-p 00000000 00:00 0 
7fb0dda6c000-7fb0dda6d000 ---p 00000000 00:00 0 
7fb0dda6d000-7fb0dda6f000 rw-p 00000000 00:00 0 
7fb0dda6f000-7fb0dda70000 ---p 00000000 00:00 0 
7fb0dda70000-7fb0dda72000 rw-p 00000000 00:00 0 
7fb0dda72000-7fb0dda73000 ---p 00000000 00:00 0 
7fb0dda73000-7fb0dda75000 rw-p 00000000 00:00 0 
7fb0dda75000-7fb0dda76000 ---p 00000000 00:00 0 
7fb0dda76000-7fb0dda78000 rw-p 00000000 00:00 0 
7fb0dda78000-7fb0dda79000 ---p 00000000 00:00 0 
7fb0dda79000-7fb0dda7b000 rw-p 00000000 00:00 0 
7fb0dda7b000-7fb0dda7c000 ---p 00000000 00:00 0 
7fb0dda7c000-7fb0ddc61000 rw-p 00000000 00:00 0 
7fb0ddc61000-7fb0ddc62000 ---p 00000000 00:00 0 
7fb0ddc62000-7fb0ddc64000 rw-p 00000000 00:00 0 
7fb0ddc64000-7fb0ddc65000 ---p 00000000 00:00 0 
7fb0ddc65000-7fb0ddc67000 rw-p 00000000 00:00 0 
7fb0ddc67000-7fb0ddc68000 r--p 00025000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7fb0ddc68000-7fb0ddc69000 rw-p 00026000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7fb0ddc69000-7fb0ddc6a000 rw-p 00000000 00:00 0 
7ffc326ae000-7ffc326cf000 rw-p 00000000 00:00 0                          [stack]
7ffc32753000-7ffc32756000 r--p 00000000 00:00 0                          [vvar]
7ffc32756000-7ffc32757000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 3/5:
---
   Compiling quote v1.0.7
error: could not compile `getopts`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name getopts /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.21/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=3bc5fa5912184ba0 -C extra-filename=-3bc5fa5912184ba0 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern unicode_width=/checkout/obj/build/bootstrap/debug/deps/libunicode_width-58a25f3a30d31e30.rmeta --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:01
make: *** [prepare] Error 1
---
   Compiling proc-macro-error v1.0.4
error: could not compile `globset`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name globset /cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.5/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=0925db85937ab6c7 -C extra-filename=-0925db85937ab6c7 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern aho_corasick=/checkout/obj/build/bootstrap/debug/deps/libaho_corasick-6acdc16a2f6d6338.rmeta --extern bstr=/checkout/obj/build/bootstrap/debug/deps/libbstr-92e9831e788eaf80.rmeta --extern fnv=/checkout/obj/build/bootstrap/debug/deps/libfnv-c84116ea3423057b.rmeta --extern log=/checkout/obj/build/bootstrap/debug/deps/liblog-ed7a0d1137dcadc8.rmeta --extern regex=/checkout/obj/build/bootstrap/debug/deps/libregex-b0c184cb911fb168.rmeta --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:03
make: *** [prepare] Error 1
---
DirectMap2M:     4995072 kB
DirectMap1G:    55574528 kB
+ python3 ../x.py --stage 2 test --host= --target i586-unknown-linux-gnu,i686-unknown-linux-musl
    Finished dev [unoptimized] target(s) in 0.21s
*** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo': corrupted double-linked list: 0x0000556105595540 ***
======= Backtrace: =========
/lib/x86_64-linux-gnu/libc.so.6(+0x777f5)[0x7ff4dd9337f5]
/lib/x86_64-linux-gnu/libc.so.6(+0x7e7d3)[0x7ff4dd93a7d3]
/lib/x86_64-linux-gnu/libc.so.6(+0x80688)[0x7ff4dd93c688]
/lib/x86_64-linux-gnu/libc.so.6(cfree+0x4c)[0x7ff4dd94058c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b8391)[0x556104349391]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9c5af4)[0x556104356af4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b139f)[0x55610434239f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b6683)[0x556104347683]
/lib/x86_64-linux-gnu/libc.so.6(+0x3a008)[0x7ff4dd8f6008]
/lib/x86_64-linux-gnu/libc.so.6(+0x3a055)[0x7ff4dd8f6055]
/lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf7)[0x7ff4dd8dc847]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x176f41)[0x556103b07f41]
======= Memory map: ========
556103991000-556104790000 r-xp 00000000 08:11 13950211                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
55610498f000-556104a35000 r--p 00dfe000 08:11 13950211                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
556104a35000-556104a3d000 rw-p 00ea4000 08:11 13950211                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
556104a3d000-556104a42000 rw-p 00000000 00:00 0 
556105592000-55610587e000 rw-p 00000000 00:00 0                          [heap]
7ff4d8000000-7ff4d8021000 rw-p 00000000 00:00 0 
7ff4d8021000-7ff4dc000000 ---p 00000000 00:00 0 
7ff4dd8bc000-7ff4dda7c000 r-xp 00000000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7ff4dda7c000-7ff4ddc7c000 ---p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7ff4ddc7c000-7ff4ddc80000 r--p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7ff4ddc80000-7ff4ddc82000 rw-p 001c4000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7ff4ddc82000-7ff4ddc86000 rw-p 00000000 00:00 0 
7ff4ddc86000-7ff4ddc89000 r-xp 00000000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7ff4ddc89000-7ff4dde88000 ---p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7ff4dde88000-7ff4dde89000 r--p 00002000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7ff4dde89000-7ff4dde8a000 rw-p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7ff4dde8a000-7ff4ddf92000 r-xp 00000000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7ff4ddf92000-7ff4de191000 ---p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7ff4de191000-7ff4de192000 r--p 00107000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7ff4de192000-7ff4de193000 rw-p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7ff4de193000-7ff4de1ab000 r-xp 00000000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7ff4de1ab000-7ff4de3aa000 ---p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7ff4de3aa000-7ff4de3ab000 r--p 00017000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7ff4de3ab000-7ff4de3ac000 rw-p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7ff4de3ac000-7ff4de3b0000 rw-p 00000000 00:00 0 
7ff4de3b0000-7ff4de3b7000 r-xp 00000000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7ff4de3b7000-7ff4de5b6000 ---p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7ff4de5b6000-7ff4de5b7000 r--p 00006000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7ff4de5b7000-7ff4de5b8000 rw-p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7ff4de5b8000-7ff4de5ce000 r-xp 00000000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7ff4de5ce000-7ff4de7cd000 ---p 00016000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7ff4de7cd000-7ff4de7ce000 rw-p 00015000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7ff4de7ce000-7ff4de7f4000 r-xp 00000000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7ff4de9e7000-7ff4de9ed000 rw-p 00000000 00:00 0 
7ff4de9f2000-7ff4de9f3000 rw-p 00000000 00:00 0 
7ff4de9f3000-7ff4de9f4000 r--p 00025000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7ff4de9f4000-7ff4de9f5000 rw-p 00026000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7ff4de9f5000-7ff4de9f6000 rw-p 00000000 00:00 0 
7ffcd57b6000-7ffcd57d7000 rw-p 00000000 00:00 0                          [stack]
7ffcd57d9000-7ffcd57dc000 r--p 00000000 00:00 0                          [vvar]
7ffcd57dc000-7ffcd57dd000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--no-deps" "--manifest-path" "/checkout/Cargo.toml"
expected success, got: signal: 6 (core dumped)', src/bootstrap/metadata.rs:39:18
Build completed unsuccessfully in 0:00:00
