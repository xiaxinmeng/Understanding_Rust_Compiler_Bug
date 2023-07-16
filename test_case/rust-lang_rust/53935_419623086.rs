plain
[01:45:32] test process::tests::test_add_to_env ... ok
[01:45:32] test process::tests::test_command_implements_send ... ok
[01:45:32] test process::tests::test_finish_once ... ok
[01:45:32] test process::tests::test_finish_twice ... ok
[01:45:32] thread '<unnamed>' panicked at 'output doesn't contain `RUN_TEST_NEW_ENV2=456`
[01:45:32] AR=ar
[01:45:32] AR_x86_64-unknown-linux-gnu=ar
[01:45:32] AWS_ACCESS_KEY_ID=AKIAJAMV3QAMMA6AXHFQ
[01:45:32] AWS_SECRET_ACCESS_KEY=[secure]
[01:45:32] BOOTSTRAP_PARENT_ID=9692
[01:45:32] BOOTSTRAP_PYTHON=/usr/bin/python2.7
[01:45:32] BUILD_DIR=/checkout/obj/build/tmp/distcheck/build
[01:45:32] CARGO=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
[01:45:32] CARGO_HOME=/cargo
[01:45:32] CARGO_MANIFEST_DIR=/checkout/obj/build/tmp/distcheck/src/libstd
[01:45:32] CARGO_MANIFEST_DIR=/checkout/obj/build/tmp/distcheck/src/libstd
[01:45:32] CARGO_PKG_AUTHORS=The Rust Project Developers
[01:45:32] CARGO_PKG_DESCRIPTION=The Rust Standard Library
[01:45:32] CARGO_PKG_HOMEPAGE=
[01:45:32] CARGO_PKG_NAME=std
[01:45:32] CARGO_PKG_VERSION=0.0.0
[01:45:32] CARGO_PKG_VERSION_MAJOR=0
[01:45:32] CARGO_PKG_VERSION_MINOR=0
[01:45:32] CARGO_PKG_VERSION_PATCH=0
[01:45:32] CARGO_PKG_VERSION_PRE=
[01:45:32] CARGO_TARGET_DIR=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage1-std
[01:45:32] CC=sccache cc
[01:45:32] CC_x86_64-unknown-linux-gnu=sccache cc
[01:45:32] CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu
[01:45:32] CFG_RELEASE_CHANNEL=dev
[01:45:32] CFLAGS=-ffunction-sections -fdata-sections -fPIC -m64
[01:45:32] CFLAGS_x86_64-unknown-linux-gnu=-ffunction-sections -fdata-sections -fPICINES=false
[01:45:32] RUSTC_DEBUG_ASSERTIONS=true
[01:45:32] RUSTC_DENY_WARNINGS=1
[01:45:32] RUSTC_ERROR_METADATA_DST=/checkout/obj/build/tmp/distcheck/build/tmp/extended-error-metadata
[01:45:32] RUSTC_FORCE_UNSTABLE=1
[01:45:32] RUSTC_LIBDIR=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage1/lib
[01:45:32] RUSTC_METADATA_SUFFIX=rustc
[01:45:32] RUSTC_PRINT_STEP_TIMINGS=1
[01:45:32] RUSTC_REAL=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
[01:45:32] RUSTC_RPATH=true
[01:45:32] RUSTC_SNAPSHOT=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
[01:45:32] RUSTC_SNAPSHOT_LIBDIR=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/lib
[01:45:32] RUSTC_STAGE=1
[01:45:32] RUSTC_SYSROOT=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage1
[01:45:32] RUSTC_VERBOSE=0
[01:45:32] RUSTDOC=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/rustdoc
[01:45:32] RUSTDOC_CRATE_VERSION=1.30.0-dev
[01:45:32] RUSTDOC_REAL=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc
[01:45:32] RUSTDOC_REAL=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc
[01:45:32] RUST_CONFIGURE_ARGS=--build=x86_64-unknown-linux-gnu --set rust.ignore-git=false --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-openssl-static --enable-debug-assertions --enable-llvm-assertions
[01:45:32] RUST_RELEASE_CHANNEL=nightly
[01:45:32] SCCACHE_BUCKET=rust-lang-ci-sccache2
[01:45:32] SCCACHE_REGION=us-west-1
[01:45:32] SCRIPT=python2.7 ../x.py test distcheck
[01:45:32] SHLVL=2
[01:45:32] SHLVL=2
[01:45:32] SRC=/checkout/obj/build/tmp/distcheck
[01:45:32] SSL_CERT_DIR=/usr/lib/ssl/certs
[01:45:32] SSL_CERT_FILE=/usr/lib/ssl/certs/ca-certificates.crt
[01:45:32] TERM=xterm
[01:45:32] TEST_MIRI=false
[01:45:32] TRAVIS=true
[01:45:32] TRAVIS_BRANCH=auto
[01:45:32] TRAVIS_BRANCH=auto
[01:45:32] WINAPI_NO_BUNDLED_LIBRARIES=1
[01:45:32] _=/bin/sh
[01:45:32] __CARGO_DEFAULT_LIB_METADATA=dev
[01:45:32] ', libstd/process.rs:1828:13
[01:45:32] test process::tests::test_capture_env_at_spawn ... ok
[01:45:32] test process::tests::test_interior_nul_in_args_is_error ... ok
[01:45:32] test process::tests::test_interior_nul_in_arg_is_error ... ok
[01:45:32] test process::tests::test_interior_nul_in_current_dir_is_error ... ok
---
[01:45:42] 
[01:45:42] error: test failed, to rerun pass '--lib'
[01:45:42] 
[01:45:42] 
[01:45:42] command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--frozen" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/obj/build/tmp/distcheck/src/libstd/Cargo.toml" "-p" "std" "--"
[01:45:42] 
[01:45:42] 
[01:45:42] failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test
[01:45:42] Build completed unsuccessfully in 1:30:36
[01:45:42] Build completed unsuccessfully in 1:30:36
[01:45:43] make: *** [check] Error 1
[01:45:43] Makefile:58: recipe for target 'check' failed
[01:45:43] 
[01:45:43] 
[01:45:43] command did not execute successfully: "make" "check"
[01:45:43] 
[01:45:43] 
[01:45:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[01:45:43] Build completed unsuccessfully in 1:42:23
---
travis_time:end:0df1bfd7:start=1536394899256407954,finish=1536394899265364875,duration=8956921
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ed4e417
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e051970
travis_time:start:0e051970
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c5062c8
$ dmesg | grep -i kill
