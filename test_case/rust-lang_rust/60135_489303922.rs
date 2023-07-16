plain
[00:03:45]  ---> 935f81178b9d
[00:03:45] Step 51/56 : ENV TARGETS=$TARGETS,thumbv7neon-unknown-linux-gnueabihf
[00:03:45]  ---> Using cache
[00:03:45]  ---> 160f6ee3d14d
[00:03:45] Step 52/56 : ENV CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc     CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc     CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc     CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc     CC_armebv7r_none_eabi=arm-none-eabi-gcc     CC_armv7a_none_eabi=arm-none-eabi-gcc     CC_armv7a_none_eabihf=arm-none-eabi-gcc     CC_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc     AR_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-ar     CXX_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
[00:03:45]  ---> 1d86202bd6b5
[00:03:45] Step 53/56 : ENV RUST_CONFIGURE_ARGS       --musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs
[00:03:45]  ---> Using cache
[00:03:45]  ---> 13f0c5ce979f
---
[01:02:51] travis_time:end:test_run-make:start=1556953432303417126,finish=1556953432722829229,duration=419412103

[01:02:51] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "thumbv7em-none-eabihf", mode: "run-make", suite: "run-make", path: Some("src/test/run-make"), compare_mode: None } -- 0.435
[01:02:51] Build completed successfully in 0:57:20
[01:02:51] + python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,armv7a-none-eabi,armv7a-none-eabihf,thumbv7neon-unknown-linux-gnueabihf
[01:02:54] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[01:02:54] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[01:02:54] Dist docs (asmjs-unknown-emscripten)
[01:02:54]  skipping - docs disabled
---
[01:37:25]    Compiling compiler_builtins v0.1.10
[01:37:28] error: failed to run custom build command for `compiler_builtins v0.1.10`
[01:37:28] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-42fce5e38c1933b0/build-script-build` (exit code: 101)
[01:37:28] --- stdout
[01:37:28] cargo:rerun-if-changed=build.rs
[01:37:28] cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.10/compiler-rt
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvdi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvsi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvdi3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvsi3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvti3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_div0.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_drsub.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_frsub.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/apple_versioning.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/bswapdi2.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/bswapsi2.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/clzdi2.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/clzsi2.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/clzti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/cmpdi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/cmpti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzdi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzsi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divdc3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/divmodsi4.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divsc3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divxc3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/extendhfsf2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ffsti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/fixdfsivfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/fixsfsivfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/fixunsdfsivfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/fixunssfsivfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/floatsidfvfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/floatsisfvfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/floatunssidfvfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/floatunssisfvfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/int_util.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/modsi3.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/muldc3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulsc3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvdi3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvsi3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvti3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulxc3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negdf2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/negdf2vfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negdi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negsf2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/negsf2vfp.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvdi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvsi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/paritydi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/paritysi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/parityti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountdi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountsi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/powixf2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/restore_vfp_d8_d15_regs.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/save_vfp_d8_d15_regs.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvdi3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvsi3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvti3.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch16.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch32.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch8.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switchu8.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/sync_synchronize.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncdfhf2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncdfsf2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncsfhf2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ucmpdi2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ucmpti2.c
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/udivmodsi4.S
[01:37:28] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/umodsi3.S
[01:37:28] TARGET = Some("armv7a-none-eabihf")
[01:37:28] HOST = Some("x86_64-unknown-linux-gnu")
[01:37:28] HOST = Some("x86_64-unknown-linux-gnu")
[01:37:28] CC_armv7a-none-eabihf = Some("sccache arm-none-eabi-gcc")
[01:37:28] CFLAGS_armv7a-none-eabihf = Some("-ffunction-sections -fdata-sections -fPIC")
[01:37:28] CRATE_CC_NO_DEFAULTS = None
[01:37:28] DEBUG = Some("false")
[01:37:28] CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7,vfp2,vfp3")
[01:37:28] CC_armv7a-none-eabihf = Some("sccache arm-none-eabi-gcc")
[01:37:28] CFLAGS_armv7a-none-eabihf = Some("-ffunction-sections -fdata-sections -fPIC")
[01:37:28] CRATE_CC_NO_DEFAULTS = None
[01:37:28] CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7,vfp2,vfp3")
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/absvdi2.o" "-c" "./compiler-rt/lib/builtins/absvdi2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/absvsi2.o" "-c" "./compiler-rt/lib/builtins/absvsi2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/absvti2.o" "-c" "./compiler-rt/lib/builtins/absvti2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/addvdi3.o" "-c" "./compiler-rt/lib/builtins/addvdi3.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/addvsi3.o" "-c" "./compiler-rt/lib/builtins/addvsi3.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/addvti3.o" "-c" "./compiler-rt/lib/builtins/addvti3.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/aeabi_cdcmp.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_cdcmp.S"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/aeabi_cfcmp.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_cfcmp.S"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/aeabi_div0.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_div0.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/aeabi_drsub.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_drsub.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/aeabi_frsub.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_frsub.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/apple_versioning.o" "-c" "./compiler-rt/lib/builtins/apple_versioning.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/bswapdi2.o" "-c" "./compiler-rt/lib/builtins/arm/bswapdi2.S"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/bswapsi2.o" "-c" "./compiler-rt/lib/builtins/arm/bswapsi2.S"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/clzdi2.o" "-c" "./compiler-rt/lib/builtins/arm/clzdi2.S"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/clzsi2.o" "-c" "./compiler-rt/lib/builtins/arm/clzsi2.S"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/clzti2.o" "-c" "./compiler-rt/lib/builtins/clzti2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/cmpdi2.o" "-c" "./compiler-rt/lib/builtins/cmpdi2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/cmpti2.o" "-c" "./compiler-rt/lib/builtins/cmpti2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/ctzdi2.o" "-c" "./compiler-rt/lib/builtins/ctzdi2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/ctzsi2.o" "-c" "./compiler-rt/lib/builtins/ctzsi2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/ctzti2.o" "-c" "./compiler-rt/lib/builtins/ctzti2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/divdc3.o" "-c" "./compiler-rt/lib/builtins/divdc3.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/divmodsi4.o" "-c" "./compiler-rt/lib/builtins/arm/divmodsi4.S"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/divsc3.o" "-c" "./compiler-rt/lib/builtins/divsc3.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/divxc3.o" "-c" "./compiler-rt/lib/builtins/divxc3.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/extendhfsf2.o" "-c" "./compiler-rt/lib/builtins/extendhfsf2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/ffsti2.o" "-c" "./compiler-rt/lib/builtins/ffsti2.c"
[01:37:28] exit code: 0
[01:37:28] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/fixdfsivfp.o" "-c" "./compiler-rt/lib/builtins/arm/fixdfsivfp.S"
[01:37:28] cargo:warning=./compiler-rt/lib/builtins/arm/fixdfsivfp.S: Assembler messages:
[01:37:28] cargo:warning=./compiler-rt/lib/builtins/arm/fixdfsivfp.S:26: Error: selected processor does not support `vmov d7,r0,r1' in ARM mode
[01:37:28] cargo:warning=./compiler-rt/lib/builtins/arm/fixdfsivfp.S:27: Error: selected processor does not support `vcvt.s32.f64 s15,d7' in ARM mode
[01:37:28] cargo:warning=./compiler-rt/lib/builtins/arm/fixdfsivfp.S:28: Error: selected processor does not support `vmov r0,s15' in ARM mode
[01:37:28] 
[01:37:28] --- stderr
[01:37:28] thread 'main' panicked at '
[01:37:28] 
[01:37:28] 
[01:37:28] Internal error occurred: Command "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-5c9fe6f284ee09ad/out/./compiler-rt/lib/builtins/arm/fixdfsivfp.o" "-c" "./compiler-rt/lib/builtins/arm/fixdfsivfp.S" with args "arm-none-eabi-gcc" did not execute successfully (status code exit code: 1).
[01:37:28] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
[01:37:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:37:28] 
[01:37:28] warning: build failed, waiting for other jobs to finish...
[01:37:28] warning: build failed, waiting for other jobs to finish...
[01:37:51] [RUSTC-TIMING] core test:false 25.826
[01:37:51] error: build failed
[01:37:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "armv7a-none-eabihf" "-j" "4" "--release" "--locked" "--color" "always" "-p" "alloc" "--manifest-path" "/checkout/src/liballoc/Cargo.toml" "--features" "compiler-builtins-mem" "--message-format" "json"
[01:37:51] expected success, got: exit code: 101
[01:37:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,armv7a-none-eabi,armv7a-none-eabihf,thumbv7neon-unknown-linux-gnueabihf
travis_time:end:15d31a7e:start=1556949660882056497,finish=1556955533213821036,duration=5872331764539
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0516ee2a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03ae4e78:start=1556955534950826024,finish=1556955534964083449,duration=13257425
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1dc17deb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b83e608
travis_time:start:1b83e608
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05aa2272
$ dmesg | grep -i kill
