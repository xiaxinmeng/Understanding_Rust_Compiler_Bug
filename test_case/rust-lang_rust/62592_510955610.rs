
cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S: Assembler messages:
cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S:21: Error: bad instruction `push{r4, r5,r6,lr}'
*snip*
Internal error occurred: Command "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/sync_fetch_and_add_8.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S" with args "armv7-unknown-linux-gnueabihf-gcc" did not execute successfully (status code exit code: 1).
