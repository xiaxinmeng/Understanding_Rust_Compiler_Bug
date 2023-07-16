plain
[00:01:57]  ---> d43e4eb982ae
[00:01:57] Step 36/48 : ENV TARGETS $TARGETS,thumbv7em-none-eabihf
[00:01:57]  ---> Using cache
[00:01:57]  ---> c5665d55c4ba
[00:01:57] Step 37/48 : ENV TARGETS $TARGETS,thumbv8m.main-none-eabi
[00:01:57]  ---> 9cf0a3be9d78
[00:01:57] Step 38/48 : ENV TARGETS $TARGETS,riscv32imc-unknown-none-elf
[00:01:57]  ---> Using cache
[00:01:57]  ---> 0aae52ff5f54
---
[00:54:11] travis_time:end:test_run-make:start=1545387711049235464,finish=1545387715653164547,duration=4603929083

[00:54:11] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "thumbv7em-none-eabihf", mode: "run-make", suite: "run-make", path: Some("src/test/run-make"), compare_mode: None } -- 4.618
[00:54:11] Build completed successfully in 0:50:26
[00:54:11] + python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.main-none-eabi,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:54:12] Dist docs (asmjs-unknown-emscripten)
[00:54:12]  skipping - docs disabled
[00:54:12] Dist docs (wasm32-unknown-emscripten)
[00:54:12]  skipping - docs disabled
---
[00:54:12] Dist docs (thumbv7em-none-eabi)
[00:54:12]  skipping - docs disabled
[00:54:12] Dist docs (thumbv7em-none-eabihf)
[00:54:12]  skipping - docs disabled
[00:54:12] Dist docs (thumbv8m.main-none-eabi)
[00:54:12] Dist docs (riscv32imc-unknown-none-elf)
[00:54:12]  skipping - docs disabled
[00:54:12] Dist docs (riscv32imac-unknown-none-elf)
[00:54:12]  skipping - docs disabled
---
[00:54:12] Dist compiler docs (thumbv7em-none-eabi)
[00:54:12]  skipping - compiler docs disabled
[00:54:12] Dist compiler docs (thumbv7em-none-eabihf)
[00:54:12]  skipping - compiler docs disabled
[00:54:12] Dist compiler docs (thumbv8m.main-none-eabi)
[00:54:12] Dist compiler docs (riscv32imc-unknown-none-elf)
[00:54:12]  skipping - compiler docs disabled
[00:54:12] Dist compiler docs (riscv32imac-unknown-none-elf)
[00:54:12]  skipping - compiler docs disabled
---
[01:17:30] travis_time:end:stage2-std:start=1545389114528828546,finish=1545389114847170018,duration=318341472

[01:17:30] [TIMING] Std { target: "thumbv7em-none-eabihf", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.318
[01:17:40] [TIMING] Std { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "thumbv7em-none-eabihf" } -- 10.223
[01:17:40] Dist std stage2 (x86_64-unknown-linux-gnu -> thumbv8m.main-none-eabi)
travis_time:start:stage2-std
travis_time:start:stage2-std
Building stage2 std artifacts (x86_64-unknown-linux-gnu -> thumbv8m.main-none-eabi)
[01:17:41]    Compiling compiler_builtins v0.1.2
[01:17:41] error: failed to run custom build command for `compiler_builtins v0.1.2`
[01:17:41] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-4496991c33f8f3f2/build-script-build` (exit code: 101)
[01:17:41] --- stdout
[01:17:41] --- stdout
[01:17:41] cargo:rerun-if-changed=build.rs
[01:17:41] cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.2/compiler-rt
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvdi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvsi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvdi3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvsi3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvti3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_div0.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_drsub.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_frsub.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/apple_versioning.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/bswapdi2.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/bswapsi2.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/clzdi2.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/clzsi2.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/clzti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/cmpdi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/cmpti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzdi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzsi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divdc3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/divmodsi4.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divsc3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divxc3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/extendhfsf2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ffsti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/int_util.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/modsi3.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/muldc3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulsc3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvdi3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvsi3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvti3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulxc3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negdf2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negdi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negsf2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvdi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvsi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/paritydi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/paritysi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/parityti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountdi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountsi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/powixf2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvdi3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvsi3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvti3.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch16.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch32.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch8.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switchu8.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/sync_synchronize.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncdfhf2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncdfsf2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncsfhf2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ucmpdi2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ucmpti2.c
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/udivmodsi4.S
[01:17:41] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/umodsi3.S
[01:17:41] TARGET = Some("thumbv8m.main-none-eabi")
[01:17:41] HOST = Some("x86_64-unknown-linux-gnu")
[01:17:41] HOST = Some("x86_64-unknown-linux-gnu")
[01:17:41] CC_thumbv8m.main-none-eabi = Some("sccache arm-none-eabi-gcc")
[01:17:41] CFLAGS_thumbv8m.main-none-eabi = Some("-ffunction-sections -fdata-sections -fPIC -mthumb -march=armv8-m.main")
[01:17:41] DEBUG = Some("false")
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
[01:17:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:17:41] 
[01:17:41] warning: build failed, waiting for other jobs to finish...
[01:17:41] warning: build failed, waiting for other jobs to finish...
[01:18:06] [RUSTC-TIMING] core test:false 25.262
[01:18:06] error: build failed
[01:18:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "thumbv8m.main-none-eabi" "-j" "4" "--release" "--locked" "--color" "always" "-p" "alloc" "--manifest-path" "/checkout/src/liballoc/Cargo.toml" "--features" "compiler-builtins-mem" "--message-format" "json"
[01:18:06] expected success, got: exit code: 101
[01:18:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.main-none-eabi,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
travis_time:end:248ca32e:start=1545384464314752971,finish=1545389150962180104,duration=4686647427133
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:216b1d38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2b76bd04:start=1545389151783227848,finish=1545389151792663450,duration=9435602
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:296f28eb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0181ad60
travis_time:start:0181ad60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1bbae1c9
$ dmesg | grep -i kill
