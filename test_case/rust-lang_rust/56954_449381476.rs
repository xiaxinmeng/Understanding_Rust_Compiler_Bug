
[01:17:41] CC_thumbv8m.main-none-eabi = Some("sccache arm-none-eabi-gcc")
[01:17:41] CFLAGS_thumbv8m.main-none-eabi = Some("-ffunction-sections -fdata-sections -fPIC -mthumb -march=armv8-m.main")
[01:17:41] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-mthumb" "-march=armv8-m.main" "-mthumb" "-march=armv8-m.main" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv8m.main-none-eabi/release/build/compiler_builtins-76ec5569094432bb/out/./compiler-rt/lib/builtins/absvdi2.o" "-c" "./compiler-rt/lib/builtins/absvdi2.c"
[01:17:41] cargo:warning=arm-none-eabi-gcc: error: unrecognized argument in option '-march=armv8-m.main'
[01:17:41] cargo:warning=arm-none-eabi-gcc: note: valid arguments to '-march=' are: armv2 armv2a armv3 armv3m armv4 armv4t armv5 armv5e armv5t armv5te armv6 armv6-m armv6j armv6k armv6s-m armv6t2 armv6z armv6zk armv7 armv7-a armv7-m armv7-r armv7e-m armv7ve armv8-a armv8-a+crc iwmmxt iwmmxt2 native
[01:17:41] cargo:warning=arm-none-eabi-gcc: error: unrecognized argument in option '-march=armv8-m.main'
[01:17:41] cargo:warning=arm-none-eabi-gcc: note: valid arguments to '-march=' are: armv2 armv2a armv3 armv3m armv4 armv4t armv5 armv5e armv5t armv5te armv6 armv6-m armv6j armv6k armv6s-m armv6t2 armv6z armv6zk armv7 armv7-a armv7-m armv7-r armv7e-m armv7ve armv8-a armv8-a+crc iwmmxt iwmmxt2 native
[01:17:41] 
[01:17:41] --- stderr
[01:17:41] thread 'main' panicked at '
[01:17:41] 
[01:17:41] 
[01:17:41] Internal error occurred: Command "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-mthumb" "-march=armv8-m.main" "-mthumb" "-march=armv8-m.main" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv8m.main-none-eabi/release/build/compiler_builtins-76ec5569094432bb/out/./compiler-rt/lib/builtins/absvdi2.o" "-c" "./compiler-rt/lib/builtins/absvdi2.c" with args "arm-none-eabi-gcc" did not execute successfully (status code exit code: 1).
[01:17:41] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.27/src/lib.rs:2313:5
