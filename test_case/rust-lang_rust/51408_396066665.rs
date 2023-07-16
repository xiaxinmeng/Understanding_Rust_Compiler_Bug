plain
########################################################                  79.1%
######################################################################## 100.0%
[00:01:15] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:17]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:37]     Updating git repository `https://github.com/est31/addr2line`
[00:01:49]     Updating git repository `https://github.com/est31/gimli`
[00:01:56]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:01:58]     Updating git repository `https://github.com/gimli-rs/cpp_demangle`
[00:01:59]     Updating git repository `https://github.com/kevinaboos/stable_deref_trait`
[00:02:00]  Downloading cmake v0.1.30
[00:02:00]  Downloading num_cpus v1.8.0
[00:02:00]  Downloading getopts v0.2.17
[00:02:00]  Downloading lazy_static v0.2.11
---
[00:03:15]  Downloading core-foundation-sys v0.6.0
[00:03:15]  Downloading backtrace-sys v0.1.22
[00:03:15]  Downloading rustc-demangle v0.1.7
[00:03:15]  Downloading is-match v0.1.0
[00:03:15]  Downloading goblin v0.0.15
[00:03:16]  Downloading scroll v0.9.0
[00:03:16]  Downloading uuid v0.6.5
[00:03:17]  Downloading byteorder v1.2.2
[00:03:17]  Downloading chalk-engine v0.6.0
[00:03:18]  Downloading difference v2.0.0
[00:03:18]  Downloading openssl v0.10.6
---
[00:03:56]    Compiling plain v0.2.3
[00:03:56]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:56]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:03:57]    Compiling byteorder v1.2.2
[00:03:58]    Compiling stable_deref_trait v1.0.0 (https://github.com/kevinaboos/stable_deref_trait?rev=8aa58b57a20b0dc215084f84de81e0f7e7dea76e#8aa58b57)
[00:03:58]    Compiling smallvec v0.4.4
[00:03:58]    Compiling smallvec v0.6.0
[00:03:59]    Compiling lazycell v1.0.0
[00:03:59]    Compiling cmake v0.1.30
---
[00:04:01]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:01]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:01]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:04:02]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:04:03]    Compiling intervaltree v0.2.3
[00:04:04]    Compiling cpp_demangle v0.2.9 (https://github.com/gimli-rs/cpp_demangle?rev=6d5320d2f5c119923647b6d7d2a7d563717a4ddf#6d5320d2)
[00:04:04]    Compiling gimli v0.16.0 (https://github.com/est31/gimli?rev=304caacf8b73f117813b1a0d436ce24403459bf2#304caacf)
[00:04:06]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:07]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:04:09]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:04:09]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:09]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:10]    Compiling quote v0.6.3
[00:04:11]    Compiling syn v0.14.2
[00:04:31]    Compiling scroll_derive v0.9.4
[00:04:34]    Compiling scroll v0.9.0
[00:04:34]    Compiling goblin v0.0.15
[00:04:49]    Compiling object v0.9.0
[00:04:51]    Compiling addr2line v0.6.0 (https://github.com/est31/addr2line?branch=no_std_prototype#19fca3a5)
[00:05:14] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:05:14] travis_fold:end:stage0-std

[00:05:14] travis_time:end:stage0-std:start=1528650788525222851,finish=1528650894934990582,duration=106409767731
---
[00:20:57]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:20:57]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:20:57]    Compiling cc v1.0.15
[00:20:58] error: build failed
[00:20:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:20:58] expected success, got: exit code: 101
[00:20:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1196:9
[00:20:58] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:20:58] travis_time:end:stage0-rustc_codegen_llvm:start=1528651837295640117,finish=1528651838733527239,duration=1437887122

[00:20:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:58] Build completed unsuccessfully in 0:14:48
[00:20:58] make: *** [all] Error 1
[00:20:58] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03b67012
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
