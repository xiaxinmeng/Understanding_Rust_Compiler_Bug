plain
[00:01:54]  ---> d43e4eb982ae
[00:01:54] Step 36/48 : ENV TARGETS $TARGETS,thumbv7em-none-eabihf
[00:01:54]  ---> Using cache
[00:01:54]  ---> c5665d55c4ba
[00:01:54] Step 37/48 : ENV TARGETS $TARGETS,thumbv8m.main-none-eabi
[00:01:54]  ---> 9cf0a3be9d78
[00:01:54] Step 38/48 : ENV TARGETS $TARGETS,riscv32imc-unknown-none-elf
[00:01:54]  ---> Using cache
[00:01:54]  ---> 0aae52ff5f54
---
[00:52:43] travis_time:end:test_run-make:start=1545355965256314686,finish=1545355969921927304,duration=4665612618

[00:52:43] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "thumbv7em-none-eabihf", mode: "run-make", suite: "run-make", path: Some("src/test/run-make"), compare_mode: None } -- 4.679
[00:52:43] Build completed successfully in 0:49:12
[00:52:43] + python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.main-none-eabi,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:52:45] Dist docs (asmjs-unknown-emscripten)
[00:52:45]  skipping - docs disabled
[00:52:45] Dist docs (wasm32-unknown-emscripten)
[00:52:45]  skipping - docs disabled
---
[00:52:45] Dist docs (thumbv7em-none-eabi)
[00:52:45]  skipping - docs disabled
[00:52:45] Dist docs (thumbv7em-none-eabihf)
[00:52:45]  skipping - docs disabled
[00:52:45] Dist docs (thumbv8m.main-none-eabi)
[00:52:45] Dist docs (riscv32imc-unknown-none-elf)
[00:52:45]  skipping - docs disabled
[00:52:45] Dist docs (riscv32imac-unknown-none-elf)
[00:52:45]  skipping - docs disabled
---
[00:52:45] Dist compiler docs (thumbv7em-none-eabi)
[00:52:45]  skipping - compiler docs disabled
[00:52:45] Dist compiler docs (thumbv7em-none-eabihf)
[00:52:45]  skipping - compiler docs disabled
[00:52:45] Dist compiler docs (thumbv8m.main-none-eabi)
[00:52:45] Dist compiler docs (riscv32imc-unknown-none-elf)
[00:52:45]  skipping - compiler docs disabled
[00:52:45] Dist compiler docs (riscv32imac-unknown-none-elf)
[00:52:45]  skipping - compiler docs disabled
---
[01:15:59] travis_time:end:stage2-std:start=1545357365530995546,finish=1545357365840445847,duration=309450301

[01:15:59] [TIMING] Std { target: "thumbv7em-none-eabihf", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 0.309
[01:16:08] [TIMING] Std { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "thumbv7em-none-eabihf" } -- 9.424
[01:16:08] Dist std stage2 (x86_64-unknown-linux-gnu -> thumbv8m.main-none-eabi)
travis_time:start:stage2-std
travis_time:start:stage2-std
Building stage2 std artifacts (x86_64-unknown-linux-gnu -> thumbv8m.main-none-eabi)
[01:16:09]    Compiling compiler_builtins v0.1.2
[01:16:09] error: failed to run custom build command for `compiler_builtins v0.1.2`
[01:16:09] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-ce477c427387b2b2/build-script-build` (exit code: 101)
[01:16:09] --- stdout
[01:16:09] --- stdout
[01:16:09] cargo:rerun-if-changed=build.rs
[01:16:09] cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.2/compiler-rt
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvdi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvsi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/absvti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvdi3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvsi3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/addvti3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_div0.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_drsub.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/aeabi_frsub.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/apple_versioning.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/bswapdi2.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/bswapsi2.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/clzdi2.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/clzsi2.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/clzti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/cmpdi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/cmpti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzdi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzsi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ctzti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divdc3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/divmodsi4.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divsc3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/divxc3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/extendhfsf2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ffsti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/int_util.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/modsi3.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/muldc3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulsc3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvdi3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvsi3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulvti3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/mulxc3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negdf2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negdi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negsf2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvdi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvsi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/negvti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/paritydi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/paritysi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/parityti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountdi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountsi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/popcountti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/powixf2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvdi3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvsi3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/subvti3.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch16.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch32.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switch8.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/switchu8.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/sync_synchronize.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncdfhf2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncdfsf2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/truncsfhf2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ucmpdi2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/ucmpti2.c
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/udivmodsi4.S
[01:16:09] cargo:rerun-if-changed=./compiler-rt/lib/builtins/arm/umodsi3.S
[01:16:09] TARGET = Some("thumbv8m.main-none-eabi")
[01:16:09] HOST = Some("x86_64-unknown-linux-gnu")
[01:16:09] HOST = Some("x86_64-unknown-linux-gnu")
[01:16:09] CC_thumbv8m.main-none-eabi = Some("sccache arm-none-eabi-gcc")
[01:16:09] CFLAGS_thumbv8m.main-none-eabi = Some("-ffunction-sections -fdata-sections -fPIC -mthumb -march=armv8-m.main")
[01:16:09] DEBUG = Some("false")
[01:16:09] CC_thumbv8m.main-none-eabi = Some("sccache arm-none-eabi-gcc")
[01:16:09] CFLAGS_thumbv8m.main-none-eabi = Some("-ffunction-sections -fdata-sections -fPIC -mthumb -march=armv8-m.main")
[01:16:09] running: "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-mthumb" "-march=armv8-m.main" "-mthumb" "-march=armv8-m.main" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv8m.main-none-eabi/release/build/compiler_builtins-bffa5f5dd716b8ab/out/./compiler-rt/lib/builtins/absvdi2.o" "-c" "./compiler-rt/lib/builtins/absvdi2.c"
[01:16:09] cargo:warning=arm-none-eabi-gcc: error: unrecognized argument in option '-march=armv8-m.main'
[01:16:09] cargo:warning=arm-none-eabi-gcc: note: valid arguments to '-march=' are: armv2 armv2a armv3 armv3m armv4 armv4t armv5 armv5e armv5t armv5te armv6 armv6-m armv6j armv6k armv6s-m armv6t2 armv6z armv6zk armv7 armv7-a armv7-m armv7-r armv7e-m armv7ve armv8-a armv8-a+crc iwmmxt iwmmxt2 native
[01:16:09] cargo:warning=arm-none-eabi-gcc: error: unrecognized argument in option '-march=armv8-m.main'
[01:16:09] cargo:warning=arm-none-eabi-gcc: note: valid arguments to '-march=' are: armv2 armv2a armv3 armv3m armv4 armv4t armv5 armv5e armv5t armv5te armv6 armv6-m armv6j armv6k armv6s-m armv6t2 armv6z armv6zk armv7 armv7-a armv7-m armv7-r armv7e-m armv7ve armv8-a armv8-a+crc iwmmxt iwmmxt2 native
[01:16:09] 
[01:16:09] --- stderr
[01:16:09] thread 'main' panicked at '
[01:16:09] 
[01:16:09] 
[01:16:09] Internal error occurred: Command "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-mthumb" "-march=armv8-m.main" "-mthumb" "-march=armv8-m.main" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv8m.main-none-eabi/release/build/compiler_builtins-bffa5f5dd716b8ab/out/./compiler-rt/lib/builtins/absvdi2.o" "-c" "./compiler-rt/lib/builtins/absvdi2.c" with args "arm-none-eabi-gcc" did not execute successfully (status code exit code: 1).
[01:16:09] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.27/src/lib.rs:2313:5
[01:16:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:16:09] 
[01:16:09] warning: build failed, waiting for other jobs to finish...
[01:16:09] warning: build failed, waiting for other jobs to finish...
[01:16:33] [RUSTC-TIMING] core test:false 24.902
[01:16:33] error: build failed
[01:16:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "thumbv8m.main-none-eabi" "-j" "4" "--release" "--locked" "--color" "always" "-p" "alloc" "--manifest-path" "/checkout/src/liballoc/Cargo.toml" "--features" "compiler-builtins-mem" "--message-format" "json"
[01:16:33] expected success, got: exit code: 101
[01:16:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.main-none-eabi,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
travis_time:end:009c73f2:start=1545352806579809212,finish=1545357400812326218,duration=4594232517006
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:164b8d4c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:16b5e64d:start=1545357401630210357,finish=1545357401638259705,duration=8049348
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0136e91b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:046bc900
travis_time:start:046bc900
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0986a1b0
$ dmesg | grep -i kill
