plain
[01:12:57] test [run-make] run-make/thumb-none-cortex-m ... FAILED
[01:12:57] 
[01:12:57] failures:
[01:12:57] 
[01:12:57] ---- [run-make] run-make/thumb-none-cortex-m stdout ----
[01:12:57] error: make failed
[01:12:57] status: exit code: 2
[01:12:57] command: "make"
[01:12:57] stdout:
---
[01:12:57] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[01:12:57] S=/checkout
[01:12:57] _=/bin/sh
[01:12:57] TOOLSTATE_REPO_ACCESS_TOKEN=[secure]
[01:12:57] TRAVIS=true
[01:12:57] RUST_RELEASE_CHANNEL=nightly
[01:12:57] PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
[01:12:57] RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
[01:12:57] RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
[01:12:57] RUST_BUILD_STAGE=stage2-thumbv6m-none-eabi
[01:12:57] BOOTSTRAP_PYTHON=/usr/bin/python2.7
[01:12:57] SCRIPT=python2.7 ../x.py test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[01:12:57] AR=ar
[01:12:57] TRAVIS_BRANCH=auto
[01:12:57] RUSTC_BOOTSTRAP=1
[01:12:57] HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[01:12:57] SCCACHE_BUCKET=rust-lang-ci-sccache2
[01:12:57] PWD=/checkout/obj
[01:12:57] CI_JOB_NAME=dist-various-1
[01:12:57] TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
[01:12:57] LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
[01:12:57] PYTHON=/usr/bin/python2.7
[01:12:57] HOME=/home/user
[01:12:57] LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[01:12:57] HOSTNAME=4a4d354c30b5
[01:12:57] CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
[01:12:57] SHLVL=2
[01:12:57] CC= 
[01:12:57] BUILD_DIR=/checkout/obj/build
[01:12:57] BUILD_DIR=/checkout/obj/build
[01:12:57] RUST_TEST_TMPDIR=/checkout/obj/build/tmp
[01:12:57] LLVM_CXXFLAGS=
[01:12:57] STAGING_DIR=/tmp
[01:12:57] TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
[01:12:57] TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[01:12:57] RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-openssl-static --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp
[01:12:57] MAKEFLAGS=
[01:12:57] MFLAGS=
[01:12:57] BOOTSTRAP_PARENT_ID=1045
[01:12:57] BOOTSTRAP_PARENT_ID=1045
[01:12:57] __COMPAT_LAYER=RunAsInvoker
[01:12:57] CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
[01:12:57] BUILD=x86_64-unknown-linux-gnu
[01:12:57] CXX= 
[01:12:57] AWS_ACCESS_KEY_ID=AKIAJAMV3QAMMA6AXHFQ
[01:12:57] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[01:12:57] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[01:12:57] CARGO_HOME=/cargo
[01:12:57] RUSTC_LINKER=arm-none-eabi-gcc
[01:12:57] SCCACHE_REGION=us-west-1
[01:12:57] CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
[01:12:57] LLVM_COMPONENTS=
[01:12:57] TERM=xterm
[01:12:57] MAKELEVEL=1
[01:12:57] mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
[01:12:57] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && rm -rf cortex-m
[01:12:57] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && bash -x /checkout/src/test/run-make/thumb-none-cortex-m/../git_clone_sha1.sh cortex-m https://github.com/rust-embedded/cortex-m a448e9156e2cb1e556e5441fd65426952ef4b927 
[01:12:57] Makefile:29: recipe for target 'all' failed
[01:12:57] 
[01:12:57] ------------------------------------------
[01:12:57] stderr:
[01:12:57] stderr:
[01:12:57] ------------------------------------------
[01:12:57] + PROJECT_NAME=cortex-m
[01:12:57] + URL=https://github.com/rust-embedded/cortex-m
[01:12:57] + SHA1=a448e9156e2cb1e556e5441fd65426952ef4b927
[01:12:57] + git clone https://github.com/rust-embedded/cortex-m cortex-m
[01:12:57] Cloning into 'cortex-m'...
[01:12:57] fatal: unable to access 'https://github.com/rust-embedded/cortex-m/': Could not resolve host: github.com
[01:12:57] + err_exit
[01:12:57] + echo ERROR:
[01:12:57] + exit 1
[01:12:57] make: *** [all] Error 1
[01:12:57] ------------------------------------------
[01:12:57] 
[01:12:57] 
[01:12:57] thread '[run-make] run-make/thumb-none-cortex-m' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:12:57] 
[01:12:57] 
[01:12:57] failures:
[01:12:57]     [run-make] run-make/thumb-none-cortex-m
[01:12:57]     [run-make] run-make/thumb-none-cortex-m
[01:12:57] 
[01:12:57] test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:57] 
[01:12:57] 
[01:12:57] 
[01:12:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:57] 
[01:12:57] 
[01:12:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[01:12:57] Build completed unsuccessfully in 1:07:22
---
travis_time:end:034366d1:start=1536132537932162864,finish=1536132537940220114,duration=8057250
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05566d3c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3229b903
travis_time:start:3229b903
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23a89d50
$ dmesg | grep -i kill
