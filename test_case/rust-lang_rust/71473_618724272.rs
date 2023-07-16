
[INFO] [stderr] Caused by:
[INFO] [stderr]   process didn't exit successfully: `/opt/rustwide/target/debug/build/rav1e-3a89a641e64125d3/build-script-build` (exit code: 101)
[INFO] [stderr] --- stdout
[INFO] [stderr] cargo:rustc-cfg=nasm_x86_64
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/ipred.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/ipred.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/me.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/me.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/itx_ssse3.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/itx_ssse3.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/ipred_ssse3.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/ipred_ssse3.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/satd.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/satd.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/mc.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/cdef.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/cdef.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/itx.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/itx.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/sad_sse2.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/sad_sse2.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/tables.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/tables.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc_ssse3.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/mc_ssse3.o"
[INFO] [stderr] running: "nasm" "-felf64" "-gdwarf" "-I/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/" "-Isrc/" "/opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/sad_avx.asm" "-o" "/opt/rustwide/target/debug/build/rav1e-05e9929c2949d6bf/out/src/x86/sad_avx.o"
[INFO] [stderr] 
[INFO] [stderr] --- stderr
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:5506: error: parser: instruction expected
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:2767: ... from macro `PREP_8TAP' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:5506: error: symbol `vpdpbusd' redefined
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:2768: ... from macro `PREP_8TAP' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:5506: error: parser: instruction expected
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:2768: ... from macro `PREP_8TAP' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:5506: error: symbol `vpdpbusd' redefined
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:2812: ... from macro `PREP_8TAP' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:5506: error: parser: instruction expected
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:2812: ... from macro `PREP_8TAP' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:5506: error: symbol `vpdpbusd' redefined
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:2813: ... from macro `PREP_8TAP' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:5506: error: parser: instruction expected
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:2813: ... from macro `PREP_8TAP' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:5506: error: symbol `vpdpbusd' redefined
[...]
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:4417: ... from macro `W_MASK' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:6009: error: symbol `vpshldw' redefined
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:4417: ... from macro `W_MASK' defined here
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:6009: error: parser: instruction expected
[INFO] [stderr] /opt/rustwide/cargo-home/git/checkouts/rav1e-14e16d25dd5cd9c0/91658bd/src/x86/mc.asm:4417: ... from macro `W_MASK' defined here
[INFO] [stderr] thread '<unnamed>' panicked at 'nonzero exit status: exit code: 1', /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/nasm-rs-0.1.7/src/lib.rs:343:9
