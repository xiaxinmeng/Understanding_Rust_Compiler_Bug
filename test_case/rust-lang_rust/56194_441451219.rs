plain
[00:48:42] status: exit code: 2
[00:48:42] command: "make"
[00:48:42] stdout:
[00:48:42] ------------------------------------------
[00:48:42] bash script.sh
[00:48:42] AWS_ACCESS_KEY_ID=[secure]
[00:48:42] AWS_SECRET_ACCESS_KEY=[secure]
[00:48:42] BOOTSTRAP_PARENT_ID=1059
[00:48:42] BOOTSTRAP_PYTHON=/usr/bin/python2.7
[00:48:42] BUILD_DIR=/checkout/obj/build
[00:48:42] CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
[00:48:42] CARGO_HOME=/cargo
[00:48:42] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[00:48:42] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[00:48:42] CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
[00:48:42] CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
[00:48:42] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:48:42] CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
[00:48:42] CI_JOB_NAME=dist-various-1
[00:48:42] DEPLOY=1
[00:48:42] HERE=/checkout/src/test/run-make/thumb-none-qemu
[00:48:42] HOME=/home/user
[00:48:42] HOSTNAME=9c635bf737ce
[00:48:42] HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:48:42] LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:48:42] LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
[00:48:42] LLVM_COMPONENTS=
[00:48:42] LLVM_CXXFLAGS=
[00:48:42] MAKEFLAGS=
[00:48:42] MAKELEVEL=1
[00:48:42] MFLAGS=
[00:48:42] PWD=/checkout/src/test/run-make/thumb-none-qemu
[00:48:42] PYTHON=/usr/bin/python2.7
[00:48:42] PYTHON=/usr/bin/python2.7
[00:48:42] RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
[00:48:42] RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
[00:48:42] RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --set rust.codegen-units-std=1 --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp --set rust.remap-debuginfo --enable-missing-tools
[00:48:42] RUST_RELEASE_CHANNEL=nightly
[00:48:42] S=/checkout
[00:48:42] SCCACHE_BUCKET=rust-lang-ci-sccache2
[00:48:42] SCCACHE_REGION=us-west-1
[00:48:42] SCRIPT=python2.7 ../x.py test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:48:42] SHLVL=3
[00:48:42] SRC=/checkout
[00:48:42] STAGING_DIR=/tmp
[00:48:42] TARGET=thumbv6m-none-eabi
[00:48:42] TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:48:42] TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
[00:48:42] TERM=xterm
[00:48:42] TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:48:42] TRAVIS=true
[00:48:42] TRAVIS_BRANCH=auto
[00:48:42] TRAVIS_BRANCH=auto
[00:48:42] WORK_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:48:42] _=/usr/bin/env
[00:48:42] __COMPAT_LAYER=RunAsInvoker
[00:48:42] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
[00:48:42] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
[00:48:42] Makefile:27: recipe for target 'all' failed
[00:48:42] ------------------------------------------
[00:48:42] stderr:
[00:48:42] ------------------------------------------
[00:48:42] ------------------------------------------
[00:48:42] + CRATE=example
[00:48:42] + env
[00:48:42] + sort
[00:48:42] + mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:48:42] + pushd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
[00:48:42] + rm -rf example
[00:48:42] + cp -a /checkout/src/test/run-make/thumb-none-qemu/example .
[00:48:42] + pushd example
[00:48:42] + env RUSTC_BOOTSTRAP=1 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi
[00:48:42] + grep 'x = 42'
[00:48:42] error: current package believes it's in a workspace when it's not:
[00:48:42] current:   /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/Cargo.toml
[00:48:42] 
[00:48:42] 
[00:48:42] this may be fixable by adding `obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example` to the `workspace.members` array of the manifest located at: /checkout/Cargo.toml
[00:48:42] make: *** [all] Error 1
[00:48:42] ------------------------------------------
[00:48:42] 
[00:48:42] thread '[run-make] run-make/thumb-none-qemu' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:42] CC_armebv7r_none_eabi=arm-none-eabi-gcc
[00:48:42] S=/checkout
[00:48:42] _=/bin/sh
[00:48:42] TOOLSTATE_REPO_ACCESS_TOKEN=[secure]
[00:48:42] TRAVIS=true
[00:48:42] RUST_RELEASE_CHANNEL=nightly
[00:48:42] PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
[00:48:42] RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
[00:48:42] RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
[00:48:42] RUST_BUILD_STAGE=stage2-thumbv6m-none-eabi
[00:48:42] BOOTSTRAP_PYTHON=/usr/bin/python2.7
[00:48:42] SCRIPT=python2.7 ../x.py test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:48:42] AR=ar
[00:48:42] TRAVIS_BRANCH=auto
[00:48:42] RUSTC_BOOTSTRAP=1
[00:48:42] HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:48:42] SCCACHE_BUCKET=rust-lang-ci-sccache2
[00:48:42] PWD=/checkout/obj
[00:48:42] CI_JOB_NAME=dist-various-1
[00:48:42] TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
[00:48:42] LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
[00:48:42] PYTHON=/usr/bin/python2.7
[00:48:42] HOME=/home/user
[00:48:42] LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
[00:48:42] HOSTNAME=9c635bf737ce
[00:48:42] CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
[00:48:42] SHLVL=2
[00:48:42] CC= 
[00:48:42] BUILD_DIR=/checkout/obj/build
[00:48:42] BUILD_DIR=/checkout/obj/build
[00:48:42] RUST_TEST_TMPDIR=/checkout/obj/build/tmp
[00:48:42] LLVM_CXXFLAGS=
[00:48:42] RUSTFLAGS=--cap-lints=allow
[00:48:42] STAGING_DIR=/tmp
[00:48:42] TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
[00:48:42] TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[00:48:42] RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7=/musl-armv7       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --enable-emscripten       --disable-docs --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --set rust.codegen-units-std=1 --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp --set rust.remap-debuginfo --enable-missing-tools
[00:48:42] MAKEFLAGS=
[00:48:42] MFLAGS=
[00:48:42] BOOTSTRAP_PARENT_ID=1059
[00:48:42] BOOTSTRAP_PARENT_ID=1059
[00:48:42] __COMPAT_LAYER=RunAsInvoker
[00:48:42] CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
[00:48:42] BUILD=x86_64-unknown-linux-gnu
[00:48:42] CXX= 
[00:48:42] AWS_ACCESS_KEY_ID=[secure]
[00:48:42] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:48:42] CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
[00:48:42] CARGO_HOME=/cargo
[00:48:42] RUSTC_LINKER=arm-none-eabi-gcc
[00:48:42] SCCACHE_REGION=us-west-1
[00:48:42] CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
[00:48:42] LLVM_COMPONENTS=
[00:48:42] TERM=xterm
[00:48:42] MAKELEVEL=1
[00:48:42] mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
[00:48:42] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && rm -rf cortex-m
[00:48:42] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && bash -x /checkout/src/test/run-make/thumb-none-cortex-m/../git_clone_sha1.sh cortex-m https://github.com/rust-embedded/cortex-m a448e9156e2cb1e556e5441fd65426952ef4b927 
[00:48:42] HEAD is now at a448e91 v0.5.0
[00:48:42] # HACK(eddyb) sets `RUSTC_BOOTSTRAP=1` so Cargo can accept nightly features.
[00:48:42] # These come from the top-level Rust workspace, that this crate is not a
[00:48:42] # member of, but Cargo tries to load the workspace `Cargo.toml` anyway.
[00:48:42] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && cd cortex-m && env RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --target thumbv6m-none-eabi -v
[00:48:42] Makefile:31: recipe for target 'all' failed
[00:48:42] ------------------------------------------
[00:48:42] stderr:
[00:48:42] ------------------------------------------
[00:48:42] ------------------------------------------
[00:48:42] + PROJECT_NAME=cortex-m
[00:48:42] + URL=https://github.com/rust-embedded/cortex-m
[00:48:42] + SHA1=a448e9156e2cb1e556e5441fd65426952ef4b927
[00:48:42] + git clone https://github.com/rust-embedded/cortex-m cortex-m
[00:48:42] Cloning into 'cortex-m'...
[00:48:42] + cd cortex-m
[00:48:42] + git reset --hard a448e9156e2cb1e556e5441fd65426952ef4b927
[00:48:42] error: current package believes it's in a workspace when it's not:
[00:48:42] current:   /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m/cortex-m/Cargo.toml
[00:48:42] 
[00:48:42] 
[00:48:42] this may be fixable by adding `obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m/cortex-m` to the `workspace.members` array of the manifest located at: /checkout/Cargo.toml
[00:48:42] make: *** [all] Error 101
[00:48:42] ------------------------------------------
[00:48:42] 
[00:48:42] thread '[run-make] run-make/thumb-none-cortex-m' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:42] 
---
[00:48:42] test result: FAILED. 7 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[00:48:42] 
[00:48:42] 
[00:48:42] 
[00:48:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:42] 
[00:48:42] 
[00:48:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[00:48:42] Build completed unsuccessfully in 0:44:52
---
travis_time:end:0fd7e341:start=1543161640823235787,finish=1543161640831362640,duration=8126853
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3e63b03a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b66cc6
travis_time:start:03b66cc6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:070d4302
$ dmesg | grep -i kill
