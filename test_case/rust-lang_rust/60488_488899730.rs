plain
  Downloading https://files.pythonhosted.org/packages/e1/ae/baedc9cb175552e95f3395c43055a6a5e125ae4d48a1d7a924baca83e92e/rsa-3.4.2-py2.py3-none-any.whl (46kB)
Collecting colorama<=0.3.9,>=0.2.5 (from awscli)
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting botocore==1.12.141 (from awscli)
  Downloading https://files.pythonhosted.org/packages/af/72/bb5092d4f8a7b6c9a4508b784cdfed6d856e2a202383c345a66da71cc612/botocore-1.12.141-py2.py3-none-any.whl (5.4MB)
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: pyasn1>=0.1.3 in /usr/lib/python2.7/dist-packages (from rsa<=3.5.0,>=3.1.2->awscli) (0.1.9)
Collecting urllib3<1.25,>=1.20; python_version == "2.7" (from botocore==1.12.141->awscli)
  Downloading https://files.pythonhosted.org/packages/01/11/525b02e4acc0c747de8b6ccdab376331597c569c42ea66ab0a1dbd36eca2/urllib3-1.24.3-py2.py3-none-any.whl (118kB)
---
[00:03:58]  ---> 935f81178b9d
[00:03:58] Step 51/56 : ENV TARGETS=$TARGETS,thumbv7neon-unknown-linux-gnueabihf
[00:03:58]  ---> Using cache
[00:03:58]  ---> 160f6ee3d14d
[00:03:58] Step 52/56 : ENV CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc     CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc     CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc     CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc     CC_armebv7r_none_eabi=arm-none-eabi-gcc     CC_armv7a_none_eabi=arm-none-eabi-gcc     CC_armv7a_none_eabihf=arm-none-eabi-gcc     CC_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc     AR_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-ar     CXX_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
[00:03:58]  ---> 1d86202bd6b5
[00:03:58] Step 53/56 : ENV RUST_CONFIGURE_ARGS       --musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs
[00:03:58]  ---> Using cache
[00:03:58]  ---> 13f0c5ce979f
---
[01:01:01] travis_time:end:test_run-make:start=1556848545795678502,finish=1556848546214640798,duration=418962296

[01:01:01] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "thumbv7em-none-eabihf", mode: "run-make", suite: "run-make", path: Some("src/test/run-make"), compare_mode: None } -- 0.434
[01:01:01] Build completed successfully in 0:55:15
[01:01:01] + python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,armv7a-none-eabi,armv7a-none-eabihf,thumbv7neon-unknown-linux-gnueabihf
[01:01:03] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[01:01:03] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[01:01:03] Dist docs (asmjs-unknown-emscripten)
[01:01:03]  skipping - docs disabled
---
[01:34:27]    Compiling compiler_builtins v0.1.10
[01:34:29] error: failed to run custom build command for `compiler_builtins v0.1.10`
[01:34:29] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-b32f22b58ab4da0d/build-script-build` (exit code: 101)
[01:34:29] --- stdout
[01:34:29] cargo:rerun-if-changed=build.rs
[01:34:29] cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.10/compiler-rt
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvdi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvsi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvdi3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvsi3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvti3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_div0.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_drsub.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_frsub.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/apple_versioning.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/bswapdi2.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/bswapsi2.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/clzdi2.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/clzsi2.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/clzti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/cmpdi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/cmpti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzdi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzsi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divdc3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/divmodsi4.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divsc3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divxc3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/extendhfsf2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ffsti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/fixdfsivfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/fixsfsivfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/fixunsdfsivfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/fixunssfsivfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/floatsidfvfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/floatsisfvfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/floatunssidfvfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/floatunssisfvfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/int_util.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/modsi3.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/muldc3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulsc3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvdi3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvsi3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvti3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulxc3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negdf2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/negdf2vfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negdi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negsf2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/negsf2vfp.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvdi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvsi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/paritydi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/paritysi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/parityti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountdi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountsi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/powixf2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/restore_vfp_d8_d15_regs.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/save_vfp_d8_d15_regs.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvdi3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvsi3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvti3.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch16.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch32.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch8.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switchu8.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/sync_synchronize.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncdfhf2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncdfsf2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncsfhf2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ucmpdi2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ucmpti2.c
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/udivmodsi4.S
[01:34:29] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/umodsi3.S
[01:34:29] TARGET = Some("armv7a-none-eabihf")
[01:34:29] HOST = Some("x86_64-unknown-linux-gnu")
[01:34:29] HOST = Some("x86_64-unknown-linux-gnu")
[01:34:29] CC_armv7a-none-eabihf = Some("sccache arm-none-eabi-gcc")
[01:34:29] CFLAGS_armv7a-none-eabihf = Some("-ffunction-sections -fdata-sections -fPIC")
[01:34:29] CRATE_CC_NO_DEFAULTS = None
[01:34:29] DEBUG = Some("false")
[01:34:29] CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7,vfp2,vfp3")
[01:34:29] CC_armv7a-none-eabihf = Some("sccache arm-none-eabi-gcc")
[01:34:29] CFLAGS_armv7a-none-eabihf = Some("-ffunction-sections -fdata-sections -fPIC")
[01:34:29] CRATE_CC_NO_DEFAULTS = None
[01:34:29] CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7,vfp2,vfp3")
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/absvdi2.o" "-c" "./compiler-rt/lib/builtins/absvdi2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/absvsi2.o" "-c" "./compiler-rt/lib/builtins/absvsi2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/absvti2.o" "-c" "./compiler-rt/lib/builtins/absvti2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/addvdi3.o" "-c" "./compiler-rt/lib/builtins/addvdi3.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/addvsi3.o" "-c" "./compiler-rt/lib/builtins/addvsi3.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/addvti3.o" "-c" "./compiler-rt/lib/builtins/addvti3.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/aeabi_cdcmp.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_cdcmp.S"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/aeabi_cfcmp.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_cfcmp.S"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/aeabi_div0.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_div0.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/aeabi_drsub.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_drsub.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/aeabi_frsub.o" "-c" "./compiler-rt/lib/builtins/arm/aeabi_frsub.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/apple_versioning.o" "-c" "./compiler-rt/lib/builtins/apple_versioning.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/bswapdi2.o" "-c" "./compiler-rt/lib/builtins/arm/bswapdi2.S"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/bswapsi2.o" "-c" "./compiler-rt/lib/builtins/arm/bswapsi2.S"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/clzdi2.o" "-c" "./compiler-rt/lib/builtins/arm/clzdi2.S"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/clzsi2.o" "-c" "./compiler-rt/lib/builtins/arm/clzsi2.S"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/clzti2.o" "-c" "./compiler-rt/lib/builtins/clzti2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/cmpdi2.o" "-c" "./compiler-rt/lib/builtins/cmpdi2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/cmpti2.o" "-c" "./compiler-rt/lib/builtins/cmpti2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/ctzdi2.o" "-c" "./compiler-rt/lib/builtins/ctzdi2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/ctzsi2.o" "-c" "./compiler-rt/lib/builtins/ctzsi2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/ctzti2.o" "-c" "./compiler-rt/lib/builtins/ctzti2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/divdc3.o" "-c" "./compiler-rt/lib/builtins/divdc3.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/divmodsi4.o" "-c" "./compiler-rt/lib/builtins/arm/divmodsi4.S"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/divsc3.o" "-c" "./compiler-rt/lib/builtins/divsc3.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/divxc3.o" "-c" "./compiler-rt/lib/builtins/divxc3.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/extendhfsf2.o" "-c" "./compiler-rt/lib/builtins/extendhfsf2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/ffsti2.o" "-c" "./compiler-rt/lib/builtins/ffsti2.c"
[01:34:29] exit code: 0
[01:34:29] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/fixdfsivfp.o" "-c" "./compiler-rt/lib/builtins/arm/fixdfsivfp.S"
[01:34:29] cargo:warning=./compiler-rt/lib/builtins/arm/fixdfsivfp.S: Assembler messages:
[01:34:29] cargo:warning=./compiler-rt/lib/builtins/arm/fixdfsivfp.S:26: Error: selected processor does not support `vmov d7,r0,r1' in ARM mode
[01:34:29] cargo:warning=./compiler-rt/lib/builtins/arm/fixdfsivfp.S:27: Error: selected processor does not support `vcvt.s32.f64 s15,d7' in ARM mode
[01:34:29] cargo:warning=./compiler-rt/lib/builtins/arm/fixdfsivfp.S:28: Error: selected processor does not support `vmov r0,s15' in ARM mode
[01:34:29] 
[01:34:29] --- stderr
[01:34:29] thread 'main' panicked at '
[01:34:29] 
[01:34:29] 
[01:34:29] Internal error occurred: Command "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-84cfe10922db2108/out/./compiler-rt/lib/builtins/arm/fixdfsivfp.o" "-c" "./compiler-rt/lib/builtins/arm/fixdfsivfp.S" with args "arm-none-eabi-gcc" did not execute successfully (status code exit code: 1).
[01:34:29] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
[01:34:29] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:29] 
[01:34:29] warning: build failed, waiting for other jobs to finish...
[01:34:29] warning: build failed, waiting for other jobs to finish...
[01:34:51] [RUSTC-TIMING] core test:false 24.359
[01:34:51] error: build failed
[01:34:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "armv7a-none-eabihf" "-j" "4" "--release" "--locked" "--color" "always" "-p" "alloc" "--manifest-path" "/checkout/src/liballoc/Cargo.toml" "--features" "compiler-builtins-mem" "--message-format" "json"
[01:34:51] expected success, got: exit code: 101
[01:34:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,armv7a-none-eabi,armv7a-none-eabihf,thumbv7neon-unknown-linux-gnueabihf
travis_time:end:1ac76f03:start=1556844885088012966,finish=1556850577582246270,duration=5692494233304
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0cc76e2f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08c0fca0:start=1556850579164507598,finish=1556850579172757150,duration=8249552
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18496952
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01476c00
travis_time:start:01476c00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:066912c4
$ dmesg | grep -i kill
