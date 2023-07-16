plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1f50f8d0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:07:31]    Compiling rand_isaac v0.1.1
[00:07:33]    Compiling rand_xorshift v0.1.0
[00:07:33]    Compiling owning_ref v0.3.3
[00:07:33]    Compiling semver v0.9.0
[00:07:34]    Compiling rustc-hash v1.0.1 (https://github.com/Zoxc/rustc-hash.git?branch=tweaks#5619974e)
[00:07:34]    Compiling itertools v0.8.0
[00:07:34]    Compiling chalk-macros v0.1.0
[00:07:35]    Compiling backtrace-sys v0.1.27
[00:07:35]    Compiling miniz-sys v0.1.11
---
[00:51:08]    Compiling owning_ref v0.3.3
[00:51:09]    Compiling rand_xorshift v0.1.0
[00:51:09]    Compiling rand_isaac v0.1.1
[00:51:09]    Compiling rand_hc v0.1.0
[00:51:10]    Compiling rustc-hash v1.0.1 (https://github.com/Zoxc/rustc-hash.git?branch=tweaks#5619974e)
[00:51:10]    Compiling semver v0.9.0
[00:51:10]    Compiling chalk-macros v0.1.0
[00:51:13]    Compiling itertools v0.8.0
[00:51:13]    Compiling humantime v1.2.0
---
[01:22:31]     Checking rand_isaac v0.1.1
[01:22:31]    Compiling rustc_target v0.0.0 (/checkout/src/librustc_target)
[01:22:31]     Checking chalk-macros v0.1.0
[01:22:31]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:22:31]     Checking rustc-hash v1.0.1 (https://github.com/Zoxc/rustc-hash.git?branch=tweaks#5619974e)
[01:22:31]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:22:31]    Compiling rustc_metadata v0.0.0 (/checkout/src/librustc_metadata)
[01:22:31]    Compiling rustc_incremental v0.0.0 (/checkout/src/librustc_incremental)
[01:22:31]     Checking humantime v1.2.0
---
[01:40:08]   Downloaded packed_simd v0.3.1
[01:40:21] error: failed to sync
[01:40:21] 
[01:40:21] Caused by:
[01:40:21]   found duplicate version of package `rustc-hash v1.0.1` vendored from two sources:
[01:40:21] 
[01:40:21]  source 1: https://github.com/Zoxc/rustc-hash.git?branch=tweaks#5619974e
[01:40:21]  source 2: registry `https://github.com/rust-lang/crates.io-index`
[01:40:21] 
[01:40:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[01:40:21] expected success, got: exit code: 101
[01:40:21] 
---
travis_time:end:0f4239c0:start=1554084449723232106,finish=1554084449740213028,duration=16980922
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:100b9c34
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d75e894
travis_time:start:0d75e894
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b67ad21
$ dmesg | grep -i kill
