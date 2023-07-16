plain
[00:52:55] test [run-make] run-make/thumb-none-cortex-m ... FAILED
[00:52:55] 
[00:52:55] failures:
[00:52:55] 
[00:52:55] ---- [run-make] run-make/thumb-none-cortex-m stdout ----
[00:52:55] error: make failed
[00:52:55] status: exit code: 2
[00:52:55] command: "make"
[00:52:55] stdout:
---
[00:52:55] TARGET=thumbv7em-none-eabihf
[00:52:55] S=/checkout
[00:52:55] _=/bin/sh
[00:52:55] TOOLSTATE_REPO_ACCESS_TOKEN=[secure]
[00:52:55] TRAVIS=true
[00:52:55] RUST_RELEASE_CHANNEL=nightly
[00:52:55] PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
[00:52:55] RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
[00:52:55] RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
[00:52:55] RUST_BUILD_STAGE=stage2-thumbv7em-none-eabihf
[00:52:55] BOOTSTRAP_PYTHON=/usr/bin/python2.7
[00:52:55] SCRIPT=python2.7 ../x.py test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imac-unknown-none-elf
[00:52:55] AR=ar
[00:52:55] TRAVIS_BRANCH=auto
[00:52:55] RUSTC_BOOTSTRAP=1
[00:52:55] HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:52:55] SCCACHE_BUCKET=rust-lang-ci-sccache2
[00:52:55] PWD=/checkout/obj
[00:52:55] CI_JOB_NAME=dist-various-1
[00:52:55] TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv7em-none-eabihf/lib
[00:52:55] LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
[00:52:55] PYTHON=/usr/bin/python2.7
[00:52:55] HOME=/home/user
[00:52:55] LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:52:55] HOSTNAME=6fc1655f0a11
[00:52:55] CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
[00:52:55] SHLVL=2
[00:52:55] CC= 
[00:52:55] BUILD_DIR=/checkout/obj/build
[00:52:55] BUILD_DIR=/checkout/obj/build
[00:52:55] RUST_TEST_TMPDIR=/checkout/obj/build/tmp
[00:52:55] LLVM_CXXFLAGS=
[00:52:55] STAGING_DIR=/tmp
[00:52:55] TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
[00:52:55] TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imac-unknown-none-elf
[00:52:55] RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-openssl-static --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp
[00:52:55] MAKEFLAGS=
[00:52:55] MFLAGS=
[00:52:55] BOOTSTRAP_PARENT_ID=976
[00:52:55] BOOTSTRAP_PARENT_ID=976
[00:52:55] __COMPAT_LAYER=RunAsInvoker
[00:52:55] CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
[00:52:55] BUILD=x86_64-unknown-linux-gnu
[00:52:55] CXX= 
[00:52:55] AWS_ACCESS_KEY_ID=AKIAJAMV3QAMMA6AXHFQ
[00:52:55] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:52:55] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:52:55] CARGO_HOME=/cargo
[00:52:55] RUSTC_LINKER=arm-none-eabi-gcc
[00:52:55] SCCACHE_REGION=us-west-1
[00:52:55] CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
[00:52:55] LLVM_COMPONENTS=
[00:52:55] TERM=xterm
[00:52:55] MAKELEVEL=1
[00:52:55] mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
[00:52:55] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && rm -rf cortex-m
[00:52:55] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && bash -x /checkout/src/test/run-make/thumb-none-cortex-m/../git_clone_sha1.sh cortex-m https://github.com/rust-embedded/cortex-m a448e9156e2cb1e556e5441fd65426952ef4b927 
[00:52:55] Makefile:29: recipe for target 'all' failed
[00:52:55] 
[00:52:55] ------------------------------------------
[00:52:55] stderr:
[00:52:55] stderr:
[00:52:55] ------------------------------------------
[00:52:55] + PROJECT_NAME=cortex-m
[00:52:55] + URL=https://github.com/rust-embedded/cortex-m
[00:52:55] + SHA1=a448e9156e2cb1e556e5441fd65426952ef4b927
[00:52:55] + git clone https://github.com/rust-embedded/cortex-m cortex-m
[00:52:55] Cloning into 'cortex-m'...
[00:52:55] fatal: unable to access 'https://github.com/rust-embedded/cortex-m/': Could not resolve host: github.com
[00:52:55] + err_exit
[00:52:55] + echo ERROR:
[00:52:55] + exit 1
[00:52:55] make: *** [all] Error 1
[00:52:55] ------------------------------------------
[00:52:55] 
[00:52:55] 
[00:52:55] thread '[run-make] run-make/thumb-none-cortex-m' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:52:55] 
[00:52:55] 
[00:52:55] failures:
[00:52:55]     [run-make] run-make/thumb-none-cortex-m
[00:52:55]     [run-make] run-make/thumb-none-cortex-m
[00:52:55] 
[00:52:55] test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:52:55] 
[00:52:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:52:55] 
[00:52:55] 
[00:52:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv7em-none-eabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv7em-none-eabihf" "--mode" "run-make" "--target" "thumbv7em-none-eabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/thumbv7em-none-eabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:55] 
[00:52:55] 
[00:52:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[00:52:55] Build completed unsuccessfully in 0:47:25
---
travis_time:end:1daf7560:start=1535092157015723461,finish=1535092157026913176,duration=11189715
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fd76655
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2622f690
travis_time:start:2622f690
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05ac2570
$ dmesg | grep -i kill
