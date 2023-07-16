plain
   Compiling toml v0.5.7
error: could not compile `bootstrap`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name bootstrap --edition=2021 src/bootstrap/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=1 -C metadata=b49b759706b6c45d -C extra-filename=-b49b759706b6c45d --out-dir /checkout/obj/build/bootstrap/debug/deps -C incremental=/checkout/obj/build/bootstrap/debug/incremental -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern build_helper=/checkout/obj/build/bootstrap/debug/deps/libbuild_helper-dded4ef7a66d7fd2.rmeta --extern cc=/checkout/obj/build/bootstrap/debug/deps/libcc-9022b5d7a536baba.rmeta --extern cmake=/checkout/obj/build/bootstrap/debug/deps/libcmake-a1cd7916c7902e9e.rmeta --extern filetime=/checkout/obj/build/bootstrap/debug/deps/libfiletime-21f7d6d27914ba34.rmeta --extern getopts=/checkout/obj/build/bootstrap/debug/deps/libgetopts-3bc5fa5912184ba0.rmeta --extern ignore=/checkout/obj/build/bootstrap/debug/deps/libignore-6eab7523daa887b2.rmeta --extern lazy_static=/checkout/obj/build/bootstrap/debug/deps/liblazy_static-1f5d3923e02f6398.rmeta --extern libc=/checkout/obj/build/bootstrap/debug/deps/liblibc-214e4f81fa8cd553.rmeta --extern merge=/checkout/obj/build/bootstrap/debug/deps/libmerge-ba43cbd72628adcc.rmeta --extern num_cpus=/checkout/obj/build/bootstrap/debug/deps/libnum_cpus-18e3b0eb22fae47b.rmeta --extern once_cell=/checkout/obj/build/bootstrap/debug/deps/libonce_cell-5bf50caf48c031d7.rmeta --extern opener=/checkout/obj/build/bootstrap/debug/deps/libopener-204d18544f7127ea.rmeta --extern serde=/checkout/obj/build/bootstrap/debug/deps/libserde-e7ed636813468773.rmeta --extern serde_json=/checkout/obj/build/bootstrap/debug/deps/libserde_json-b69a92aa1055d696.rmeta --extern time=/checkout/obj/build/bootstrap/debug/deps/libtime-1b8611857d71d3f6.rmeta --extern toml=/checkout/obj/build/bootstrap/debug/deps/libtoml-477f7006fd660090.rmeta -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
Build completed unsuccessfully in 0:01:12
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 2/5:
Command failed. Attempt 2/5:
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
    Finished dev [unoptimized] target(s) in 13.02s
*** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo': free(): invalid pointer: 0x0000564582a34540 ***
======= Backtrace: =========
/lib/x86_64-linux-gnu/libc.so.6(+0x777f5)[0x7f3b684577f5]
/lib/x86_64-linux-gnu/libc.so.6(+0x8038a)[0x7f3b6846038a]
/lib/x86_64-linux-gnu/libc.so.6(cfree+0x4c)[0x7f3b6846458c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b8391)[0x56458075f391]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x99e8d9)[0x5645807458d9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x9b668d)[0x56458075d68d]
/lib/x86_64-linux-gnu/libc.so.6(+0x3a008)[0x7f3b6841a008]
/lib/x86_64-linux-gnu/libc.so.6(+0x3a055)[0x7f3b6841a055]
/lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf7)[0x7f3b68400847]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo(+0x176f41)[0x56457ff1df41]
======= Memory map: ========
56457fda7000-564580ba6000 r-xp 00000000 08:11 12658100                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
564580da5000-564580e4b000 r--p 00dfe000 08:11 12658100                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
564580e4b000-564580e53000 rw-p 00ea4000 08:11 12658100                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
564580e53000-564580e58000 rw-p 00000000 00:00 0 
564582a1a000-564582d06000 rw-p 00000000 00:00 0                          [heap]
7f3b64000000-7f3b64021000 rw-p 00000000 00:00 0 
7f3b64021000-7f3b68000000 ---p 00000000 00:00 0 
7f3b683e0000-7f3b685a0000 r-xp 00000000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f3b685a0000-7f3b687a0000 ---p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f3b687a0000-7f3b687a4000 r--p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f3b687a4000-7f3b687a6000 rw-p 001c4000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7f3b687a6000-7f3b687aa000 rw-p 00000000 00:00 0 
7f3b687aa000-7f3b687ad000 r-xp 00000000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f3b687ad000-7f3b689ac000 ---p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f3b689ac000-7f3b689ad000 r--p 00002000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f3b689ad000-7f3b689ae000 rw-p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7f3b689ae000-7f3b68ab6000 r-xp 00000000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f3b68ab6000-7f3b68cb5000 ---p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f3b68cb5000-7f3b68cb6000 r--p 00107000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f3b68cb6000-7f3b68cb7000 rw-p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7f3b68cb7000-7f3b68ccf000 r-xp 00000000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f3b68ccf000-7f3b68ece000 ---p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f3b68ece000-7f3b68ecf000 r--p 00017000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f3b68ecf000-7f3b68ed0000 rw-p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7f3b68ed0000-7f3b68ed4000 rw-p 00000000 00:00 0 
7f3b68ed4000-7f3b68edb000 r-xp 00000000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f3b68edb000-7f3b690da000 ---p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f3b690da000-7f3b690db000 r--p 00006000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f3b690db000-7f3b690dc000 rw-p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7f3b690dc000-7f3b690f2000 r-xp 00000000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f3b690f2000-7f3b692f1000 ---p 00016000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f3b692f1000-7f3b692f2000 rw-p 00015000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7f3b692f2000-7f3b69318000 r-xp 00000000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f3b6950c000-7f3b69512000 rw-p 00000000 00:00 0 
7f3b69516000-7f3b69517000 rw-p 00000000 00:00 0 
7f3b69517000-7f3b69518000 r--p 00025000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f3b69518000-7f3b69519000 rw-p 00026000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7f3b69519000-7f3b6951a000 rw-p 00000000 00:00 0 
7ffe73739000-7ffe7375a000 rw-p 00000000 00:00 0                          [stack]
7ffe73774000-7ffe73777000 r--p 00000000 00:00 0                          [vvar]
7ffe73777000-7ffe73778000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--no-deps" "--manifest-path" "/checkout/Cargo.toml"
expected success, got: signal: 6 (core dumped)', src/bootstrap/metadata.rs:39:18
Build completed unsuccessfully in 0:00:13
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 3/5:
---
   Compiling snap v1.0.1
[RUSTC-TIMING] itoa test:false 0.163
   Compiling ansi_term v0.12.1
[RUSTC-TIMING] build_script_build test:false 0.239
rustc exited with signal: 11 (core dumped)
error: could not compile `ryu`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/ryu-1.0.5/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off -C metadata=4aa0bde94fc6dc63 -C extra-filename=-4aa0bde94fc6dc63 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/ryu-4aa0bde94fc6dc63 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] tinyvec test:false 0.316
[RUSTC-TIMING] datafrog test:false 0.359
[RUSTC-TIMING] build_script_build test:false 0.483
[RUSTC-TIMING] build_script_build test:false 0.464
