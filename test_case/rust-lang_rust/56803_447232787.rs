plain
[00:02:29]  ---> 7cfdcd6edda5
[00:02:29] Step 13/47 : COPY scripts/musl.sh /build
[00:02:29]  ---> Using cache
[00:02:29]  ---> 5df40fe94087
[00:02:29] Step 14/47 : RUN env     CC=arm-linux-gnueabi-gcc CFLAGS="-march=armv5te -marm -mfloat-abi=soft"     CXX=arm-linux-gnueabi-g++ CXXFLAGS="-march=armv5te -marm -mfloat-abi=soft"     bash musl.sh armv5te &&     env     CC=arm-linux-gnueabi-gcc CFLAGS="-march=armv6 -marm"     CXX=arm-linux-gnueabi-g++ CXXFLAGS="-march=armv6 -marm"     bash musl.sh arm &&     env     CC=arm-linux-gnueabihf-gcc CFLAGS="-march=armv6 -marm -mfpu=vfp"     CXX=arm-linux-gnueabihf-g++ CXXFLAGS="-march=armv6 -marm -mfpu=vfp"     bash musl.sh armhf &&     env     CC=arm-linux-gnueabihf-gcc CFLAGS="-march=armv7-a"     CXX=arm-linux-gnueabihf-g++ CXXFLAGS="-march=armv7-a"     bash musl.sh armv7 &&     env     CC=aarch64-linux-gnu-gcc     CXX=aarch64-linux-gnu-g++     bash musl.sh aarch64 &&     env     CC=mips-openwrt-linux-gcc     CXX=mips-openwrt-linux-g++     bash musl.sh mips &&     env     CC=mipsel-openwrt-linux-gcc     CXX=mipsel-openwrt-linux-g++     bash musl.sh mipsel &&     rm -rf /build/*
[00:02:29]  ---> 2823a72a8d13
[00:02:29] Step 15/47 : RUN echo "# a" >> /usr/local/mips-linux-musl/bin/mips-openwrt-linux-musl-wrapper.sh &&   echo "# b" >> /usr/local/mipsel-linux-musl/bin/mipsel-openwrt-linux-musl-wrapper.sh
[00:02:29]  ---> Using cache
[00:02:29]  ---> 44edc1ce1f68
---
[00:51:55] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[00:51:55] S=/checkout
[00:51:55] _=/bin/sh
[00:51:55] TOOLSTATE_REPO_ACCESS_TOKEN=[secure]
[00:51:55] TRAVIS=true
[00:51:55] RUST_RELEASE_CHANNEL=nightly
[00:51:55] PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
[00:51:55] RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
[00:51:55] RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
[00:51:55] RUST_BUILD_STAGE=stage2-thumbv6m-none-eabi
[00:51:55] BOOTSTRAP_PYTHON=/usr/bin/python2.7
[00:51:55] SCRIPT=python2.7 ../x.py test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:51:55] AR=ar
[00:51:55] TRAVIS_BRANCH=auto
[00:51:55] RUSTC_BOOTSTRAP=1
[00:51:55] HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:51:55] SCCACHE_BUCKET=rust-lang-ci-sccache2
[00:51:55] PWD=/checkout/obj
[00:51:55] CI_JOB_NAME=dist-various-1
[00:51:55] TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
[00:51:55] LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
[00:51:55] PYTHON=/usr/bin/python2.7
[00:51:55] HOME=/home/user
[00:51:55] LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:51:55] HOSTNAME=eada5893358a
[00:51:55] CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
[00:51:55] SHLVL=2
[00:51:55] CC= 
[00:51:55] BUILD_DIR=/checkout/obj/build
[00:51:55] BUILD_DIR=/checkout/obj/build
[00:51:55] RUST_TEST_TMPDIR=/checkout/obj/build/tmp
[00:51:55] LLVM_CXXFLAGS=
[00:51:55] RUSTFLAGS=--cap-lints=allow
[00:51:55] STAGING_DIR=/tmp
[00:51:55] TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
[00:51:55] TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:51:55] RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --set rust.codegen-units-std=1 --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp --set rust.remap-debuginfo --enable-missing-tools
[00:51:55] MAKEFLAGS=
[00:51:55] MFLAGS=
[00:51:55] BOOTSTRAP_PARENT_ID=1086
[00:51:55] BOOTSTRAP_PARENT_ID=1086
[00:51:55] __COMPAT_LAYER=RunAsInvoker
[00:51:55] CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
[00:51:55] BUILD=x86_64-unknown-linux-gnu
[00:51:55] CXX= 
[00:51:55] AWS_ACCESS_KEY_ID=[secure]
[00:51:55] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:51:55] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:51:55] CARGO_HOME=/cargo
[00:51:55] RUSTC_LINKER=arm-none-eabi-gcc
[00:51:55] SCCACHE_REGION=us-west-1
[00:51:55] CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
[00:51:55] LLVM_COMPONENTS=
[00:51:55] TERM=xterm
[00:51:55] MAKELEVEL=1
[00:51:55] mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
[00:51:55] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && rm -rf cortex-m
[00:51:55] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && bash -x /checkout/src/test/run-make/thumb-none-cortex-m/../git_clone_sha1.sh cortex-m https://github.com/rust-embedded/cortex-m a448e9156e2cb1e556e5441fd65426952ef4b927 
[00:51:55] Makefile:31: recipe for target 'all' failed
[00:51:55] 
[00:51:55] ------------------------------------------
[00:51:55] stderr:
[00:51:55] stderr:
[00:51:55] ------------------------------------------
[00:51:55] + PROJECT_NAME=cortex-m
[00:51:55] + URL=https://github.com/rust-embedded/cortex-m
[00:51:55] + SHA1=a448e9156e2cb1e556e5441fd65426952ef4b927
[00:51:55] + git clone https://github.com/rust-embedded/cortex-m cortex-m
[00:51:55] Cloning into 'cortex-m'...
[00:51:55] fatal: unable to access 'https://github.com/rust-embedded/cortex-m/': Could not resolve host: github.com
[00:51:55] + err_exit
[00:51:55] + echo ERROR:
[00:51:55] + exit 1
[00:51:55] make: *** [all] Error 1
[00:51:55] ------------------------------------------
[00:51:55] 
[00:51:55] thread '[run-make] run-make/thumb-none-cortex-m' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:51:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:55] status: exit code: 2
[00:51:55] command: "make"
[00:51:55] stdout:
[00:51:55] ------------------------------------------
[00:51:55] bash script.sh
[00:51:55] AWS_ACCESS_KEY_ID=[secure]
[00:51:55] AWS_SECRET_ACCESS_KEY=[secure]
[00:51:55] BOOTSTRAP_PARENT_ID=1086
[00:51:55] BOOTSTRAP_PYTHON=/usr/bin/python2.7
[00:51:55] BUILD_DIR=/checkout/obj/build
[00:51:55] CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
[00:51:55] CARGO_HOME=/cargo
[00:51:55] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[00:51:55] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[00:51:55] CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
[00:51:55] CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
[00:51:55] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:51:55] CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
[00:51:55] CI_JOB_NAME=dist-various-1
[00:51:55] DEPLOY=1
[00:51:55] HERE=/checkout/src/test/run-make/thumb-none-qemu
[00:51:55] HOME=/home/user
[00:51:55] HOSTNAME=eada5893358a
[00:51:55] HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:51:55] LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:51:55] LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
[00:51:55] LLVM_COMPONENTS=
[00:51:55] LLVM_CXXFLAGS=
[00:51:55] MAKEFLAGS=
[00:51:55] MAKELEVEL=1
[00:51:55] MFLAGS=
[00:51:55] PWD=/checkout/src/test/run-make/thumb-none-qemu
[00:51:55] PYTHON=/usr/bin/python2.7
[00:51:55] PYTHON=/usr/bin/python2.7
[00:51:55] RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
[00:51:55] RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
[00:51:55] RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --set rust.codegen-units-std=1 --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp --set rust.remap-debuginfo --enable-missing-tools
[00:51:55] RUST_RELEASE_CHANNEL=nightly
[00:51:55] S=/checkout
[00:51:55] SCCACHE_BUCKET=rust-lang-ci-sccache2
[00:51:55] SCCACHE_REGION=us-west-1
[00:51:55] SCRIPT=python2.7 ../x.py test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:51:55] SHLVL=3
[00:51:55] SRC=/checkout
[00:51:55] STAGING_DIR=/tmp
[00:51:55] TARGET=thumbv6m-none-eabi
[00:51:55] TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:51:55] TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
[00:51:55] TERM=xterm
[00:51:55] TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:51:55] TRAVIS=true
[00:51:55] TRAVIS_BRANCH=auto
[00:51:55] TRAVIS_BRANCH=auto
[00:51:55] WORK_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:51:55] _=/usr/bin/env
[00:51:55] __COMPAT_LAYER=RunAsInvoker
[00:51:55] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
[00:51:55] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
[00:51:55] Makefile:27: recipe for target 'all' failed
[00:51:55] ------------------------------------------
[00:51:55] stderr:
[00:51:55] ------------------------------------------
[00:51:55] ------------------------------------------
[00:51:55] + CRATE=example
[00:51:55] + env
[00:51:55] + env
[00:51:55] + mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:51:55] + pushd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:51:55] + rm -rf example
[00:51:55] + cp -a /checkout/src/test/run-make/thumb-none-qemu/example .
[00:51:55] + pushd example
[00:51:55] + grep 'x = 42'
[00:51:55] + env RUSTC_BOOTSTRAP=1 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi
[00:51:55] warning: spurious network error (2 tries remaining): curl error: Could not resolve host: github.com
[00:51:55] ; class=Net (12)
[00:51:55] warning: spurious network error (1 tries remaining): curl error: Could not resolve host: github.com
[00:51:55] ; class=Net (12)
[00:51:55] ; class=Net (12)
[00:51:55] error: failed to load source for a dependency on `cortex-m`
[00:51:55] Caused by:
[00:51:55]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[00:51:55] 
[00:51:55] Caused by:
[00:51:55] Caused by:
[00:51:55]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[00:51:55] 
[00:51:55] Caused by:
[00:51:55]   curl error: Could not resolve host: github.com
[00:51:55] ; class=Net (12)
[00:51:55] make: *** [all] Error 1
[00:51:55] ------------------------------------------
[00:51:55] 
[00:51:55] thread '[run-make] run-make/thumb-none-qemu' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:51:55] 
---
[00:51:55] test result: FAILED. 9 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[00:51:55] 
[00:51:55] 
[00:51:55] 
[00:51:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:55] 
[00:51:55] 
[00:51:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[00:51:55] Build completed unsuccessfully in 0:47:57
---
travis_time:end:03d44630:start=1544770292388399686,finish=1544770292396368145,duration=7968459
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c2c429a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00115551
travis_time:start:00115551
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0daad773
$ dmesg | grep -i kill
