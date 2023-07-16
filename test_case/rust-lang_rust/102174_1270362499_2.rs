
compiling  sysv_i128_emulation::handwritten::rustc_calls_cc
running: "rustc" "--crate-type" "staticlib" "--out-dir" "target/temp/" "--target" "i686-unknown-linux-gnu" "-Cmetadata=sysv_i128_emulation_handwritten_rustc_caller" "handwritten_impls/rustc
/sysv_i128_emulation_handwritten_rustc_caller.rs"
running: "x86_64-linux-gnu-gcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-Wall" "-Wextra" "-o" "target/temp/handwritten_impls/cc/sysv_i128_emulation_handw
ritten_cc_callee.o" "-c" "handwritten_impls/cc/sysv_i128_emulation_handwritten_cc_callee.c"
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=handwritten_impls/cc/sysv_i128_emulation_handwritten_cc_callee.c:36:25: error: expected declaration specifiers or ‘...’ before ‘__int128’   36 | typedef void (*functy1)(__int128, __int128, float, __int128, uint8_t, __int128);      |                         ^~~~~~~~handwritten_impls/cc/sysv_i128_emulation_handwritten_cc_callee.c:36:35: error: expected declaration specifiers or ‘...’ before ‘__int128’   36 | typedef void (*functy1)(__int128, __int128, float, __int128, uint8_t, __int128);      |                                   ^~~~~~~~handwritten_impls/cc/sysv_i128_emulation_handwritten_cc_callee.c:36:52: error: expected declaration specifiers or ‘...’ before ‘__int128’   36 | typedef void (*functy1)(__int128, __int128, float, __int128, uint8_t, __int128);      |                                                    ^~~~~~~~handwritten_impls/cc/sysv_i128_emulation_handwritten_cc_callee.c:36:71: error: expected declaration specifiers or ‘...’ before ‘__int128’
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=
cargo:warning=   36 | typedef void (*functy1)(__int128, __int128, float, __int128, uint8_t, __int128);      |                                                                       ^~~~~~~~handwritten_impls/cc/sysv_i128_emulation_handwritten_cc_callee.c:39:27: error: expected declaration specifiers or ‘...’ before ‘__int128’   39 | void callee_native_layout(__int128* arg0, __int128* arg1, __int128* arg2) {      |                           ^~~~~~~~handwritten_impls/cc/sysv_i128_emulation_handwritten_cc_callee.c:39:43: error: expected declaration specifiers or ‘...’ before ‘__int128’   39 | void callee_native_layout(__int128* arg0, __int128* arg1, __int128* arg2) {      |                                           ^~~~~~~~handwritten_impls/cc/sysv_i128_emulation_handwritten_cc_callee.c:39:59: error: expected declaration specifiers or ‘...’ before ‘__int128’   39 | void callee_native_layout(__int128* arg0, __int128* arg1, __int128* arg2) {
[...]
