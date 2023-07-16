
CFLAGS_aarch64_unknown_none_softfloat="-mstrict-align -march=armv8-a+nofp+nosimd" CC_aarch64_unknown_none_softfloat=$PWD/gcc/bin/aarch64-elf-gcc RUST_COMPILER_RT_ROOT=/opt/repos/rust/src/llvm-project/compiler-rt xargo build --target aarch64-unknown-none-softfloat --features c -vv

// Some omitted

[compiler_builtins 0.1.24] running: "/opt/compiler-builtins/gcc/bin/aarch64-elf-gcc" "/opt/compiler-builtins/gcc/bin/aarch64-elf-gcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-mstrict-align" "-march=armv8-a+nofp+nosimd" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/opt/compiler-builtins/target/aarch64-unknown-none-softfloat/debug/build/compiler_builtins-2375755c04f9af94/out/comparetf2.o" "-c" "/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c"
[compiler_builtins 0.1.24] cargo:warning=In file included from /opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:40:0:
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/fp_lib.h: In function 'toRep':
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/fp_lib.h:218:23: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning= static __inline rep_t toRep(fp_t x) {
[compiler_builtins 0.1.24] cargo:warning=                       ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/fp_lib.h:222:5: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=   } rep = {.f = x};
[compiler_builtins 0.1.24] cargo:warning=     ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c: In function '__letf2':
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:45:32: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning= COMPILER_RT_ABI enum LE_RESULT __letf2(fp_t a, fp_t b) {
[compiler_builtins 0.1.24] cargo:warning=                                ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:45:32: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:47:28: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=   const srep_t aInt = toRep(a);
[compiler_builtins 0.1.24] cargo:warning=                            ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:48:28: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=   const srep_t bInt = toRep(b);
[compiler_builtins 0.1.24] cargo:warning=                            ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c: In function '__getf2':
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:98:32: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning= COMPILER_RT_ABI enum GE_RESULT __getf2(fp_t a, fp_t b) {
[compiler_builtins 0.1.24] cargo:warning=                                ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:98:32: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:100:28: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=   const srep_t aInt = toRep(a);
[compiler_builtins 0.1.24] cargo:warning=                            ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:101:28: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=   const srep_t bInt = toRep(b);
[compiler_builtins 0.1.24] cargo:warning=                            ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c: In function '__unordtf2':
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:128:21: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning= COMPILER_RT_ABI int __unordtf2(fp_t a, fp_t b) {
[compiler_builtins 0.1.24] cargo:warning=                     ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:128:21: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:129:27: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=   const rep_t aAbs = toRep(a) & absMask;
[compiler_builtins 0.1.24] cargo:warning=                           ^
[compiler_builtins 0.1.24] cargo:warning=/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c:130:27: sorry, unimplemented: '-mgeneral-regs-only' and floating point code
[compiler_builtins 0.1.24] cargo:warning=   const rep_t bAbs = toRep(b) & absMask;
[compiler_builtins 0.1.24] cargo:warning=                           ^
[compiler_builtins 0.1.24] exit code: 1
[compiler_builtins 0.1.24] 
[compiler_builtins 0.1.24] 
[compiler_builtins 0.1.24] error occurred: Command "/opt/compiler-builtins/gcc/bin/aarch64-elf-gcc" "/opt/compiler-builtins/gcc/bin/aarch64-elf-gcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-mstrict-align" "-march=armv8-a+nofp+nosimd" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/opt/compiler-builtins/target/aarch64-unknown-none-softfloat/debug/build/compiler_builtins-2375755c04f9af94/out/comparetf2.o" "-c" "/opt/rust/src/llvm-project/compiler-rt/lib/builtins/comparetf2.c" with args "aarch64-elf-gcc" did not execute successfully (status code exit code: 1).
[compiler_builtins 0.1.24] 
[compiler_builtins 0.1.24] 
error: failed to run custom build command for `compiler_builtins v0.1.24 (/opt/compiler-builtins)`

