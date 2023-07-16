plain
[00:53:33] test [run-make] run-make/wasm-export-all-symbols ... ok
[00:53:33] test [run-make] run-make/wasm-panic-small ... ok
[00:53:33] test [run-make] run-make/wasm-symbols-not-exported ... ok
[00:53:33] test [run-make] run-make/wasm-symbols-not-imported ... ok
[00:53:37] test [run-make] run-make/thumb-none-qemu ... FAILED
[00:53:39] 
[00:53:39] failures:
[00:53:39] 
[00:53:39] 
[00:53:39] ---- [run-make] run-make/thumb-none-qemu stdout ----
[00:53:39] error: make failed
[00:53:39] status: exit code: 2
[00:53:39] command: "make"
[00:53:39] stdout:
[00:53:39] stdout:
[00:53:39] ------------------------------------------
[00:53:39] bash script.sh
[00:53:39] AWS_ACCESS_KEY_ID=AKIAJAMV3QAMMA6AXHFQ
[00:53:39] AWS_SECRET_ACCESS_KEY=[secure]
[00:53:39] BOOTSTRAP_PARENT_ID=1052
[00:53:39] BOOTSTRAP_PYTHON=/usr/bin/python2.7
[00:53:39] BUILD_DIR=/checkout/obj/build
[00:53:39] CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
[00:53:39] CARGO_HOME=/cargo
[00:53:39] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[00:53:39] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[00:53:39] CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
[00:53:39] CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
[00:53:39] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:53:39] CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
[00:53:39] CI_JOB_NAME=dist-various-1
[00:53:39] DEPLOY=1
[00:53:39] HERE=/checkout/src/test/run-make/thumb-none-qemu
[00:53:39] HOME=/home/user
[00:53:39] HOSTNAME=4782378fbc1c
[00:53:39] HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:53:39] LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:53:39] LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
[00:53:39] LLVM_COMPONENTS=
[00:53:39] LLVM_CXXFLAGS=
[00:53:39] MAKEFLAGS=
[00:53:39] MAKELEVEL=1
[00:53:39] MFLAGS=
[00:53:39] PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
[00:53:39] PWD=/checkout/src/test/run-make/thumb-none-qemu
[00:53:39] PYTHON=/usr/bin/python2.7
[00:53:39] RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
[00:53:39] RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
[00:53:39] RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp --set rust.remap-debuginfo --enable-missing-tools
[00:53:39] RUST_RELEASE_CHANNEL=nightly
[00:53:39] S=/checkout
[00:53:39] SCCACHE_BUCKET=rust-lang-ci-sccache2
[00:53:39] SCCACHE_REGION=us-west-1
[00:53:39] SCRIPT=python2.7 ../x.py test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:53:39] SHLVL=3
[00:53:39] SRC=/checkout
[00:53:39] STAGING_DIR=/tmp
[00:53:39] TARGET=thumbv6m-none-eabi
[00:53:39] TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:53:39] TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
[00:53:39] TERM=xterm
[00:53:39] TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:53:39] TRAVIS=true
[00:53:39] TRAVIS_BRANCH=auto
[00:53:39] TRAVIS_BRANCH=auto
[00:53:39] WORK_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:53:39] _=/usr/bin/env
[00:53:39] __COMPAT_LAYER=RunAsInvoker
[00:53:39] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
[00:53:39] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
[00:53:39] Makefile:27: recipe for target 'all' failed
[00:53:39] ------------------------------------------
[00:53:39] stderr:
[00:53:39] ------------------------------------------
[00:53:39] ------------------------------------------
[00:53:39] + CRATE=example
[00:53:39] + env
[00:53:39] + sort
[00:53:39] + mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:53:39] + pushd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:53:39] + rm -rf example
[00:53:39] + cp -a /checkout/src/test/run-make/thumb-none-qemu/example .
[00:53:39] + pushd example
[00:53:39] + env 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi
[00:53:39] + grep 'x = 42'
[00:53:39]     Blocking waiting for file lock on the registry index
[00:53:39]  Downloading panic-halt v0.2.0
[00:53:39]  Downloading panic-halt v0.2.0
[00:53:39]  Downloading cortex-m-semihosting v0.3.1
[00:53:39]  Downloading cortex-m-rt v0.5.4
[00:53:39]  Downloading cortex-m v0.5.7
[00:53:39]  Downloading r0 v0.2.2
[00:53:39]  Downloading volatile-register v0.2.0
[00:53:39]     Blocking waiting for file lock on bare-metal-0.2.3.crate
[00:53:39]     Blocking waiting for file lock on vcell-0.1.0.crate
[00:53:39]    Compiling cortex-m v0.5.7
[00:53:39]    Compiling vcell v0.1.0
[00:53:39]    Compiling cortex-m-semihosting v0.3.1
[00:53:39]    Compiling cortex-m-rt v0.5.4
[00:53:39]    Compiling aligned v0.2.0
[00:53:39]    Compiling bare-metal v0.2.3
[00:53:39]    Compiling r0 v0.2.2
[00:53:39]    Compiling panic-halt v0.2.0
[00:53:39]    Compiling volatile-register v0.2.0
[00:53:39]    Compiling example v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example)
[00:53:39]     Finished dev [unoptimized + debuginfo] target(s) in 4.66s
[00:53:39]      Running `target/thumbv6m-none-eabi/debug/example`
[00:53:39] target/thumbv6m-none-eabi/debug/example: 1: target/thumbv6m-none-eabi/debug/example: Syntax error: word unexpected (expecting ")")
[00:53:39] make: *** [all] Error 1
[00:53:39] ------------------------------------------
[00:53:39] 
[00:53:39] 
[00:53:39] thread '[run-make] run-make/thumb-none-qemu' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:53:39] 
[00:53:39] 
[00:53:39] failures:
[00:53:39]     [run-make] run-make/thumb-none-qemu
[00:53:39]     [run-make] run-make/thumb-none-qemu
[00:53:39] 
[00:53:39] test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:53:39] 
[00:53:39] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:53:39] 
[00:53:39] 
[00:53:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:39] 
[00:53:39] 
[00:53:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[00:53:39] Build completed unsuccessfully in 0:47:19
---
travis_time:end:0c51746a:start=1540550164670387193,finish=1540550164676845634,duration=6458441
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e521b6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29b9764c
travis_time:start:29b9764c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2290ed5f
$ dmesg | grep -i kill
