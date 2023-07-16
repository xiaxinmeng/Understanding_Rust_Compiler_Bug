plain
[01:25:26] [RUSTC-TIMING] cargo test:false 138.406
[01:26:44] [RUSTC-TIMING] rls test:false 77.824
[01:26:50] [RUSTC-TIMING] rls test:false 6.123
[01:26:50]     Finished release [optimized] target(s) in 4m 43s
[01:26:50] duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:26:50] 
[01:26:50] the following dependencies are duplicated although they have the same features enabled:
[01:26:50]   crossbeam-channel 0.2.6 (registry+https://github.com/rust-lang/crates.io-index)
[01:26:50]     `rls` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libcrossbeam_channel-847b9bde19455337.rlib")
[01:26:50]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libcrossbeam_channel-694f8ae15c491d73.rlib")
[01:26:50]   ignore 0.4.4 (registry+https://github.com/rust-lang/crates.io-index)
[01:26:50]     `rls` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libignore-9e2734c668c93115.rlib")
[01:26:50]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libignore-2ea005206020e491.rlib")
[01:26:50] the following dependencies have different features:
[01:26:50]   parking_lot_core 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:26:50]     `rls` additionally enabled features {"nightly"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libparking_lot_core-ebc045d9e817e68b.rlib"
[01:26:50]     `cargo` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libparking_lot_core-122e91f3326c9552.rlib"
[01:26:50]   parking_lot 0.6.4 (registry+https://github.com/rust-lang/crates.io-index)
[01:26:50] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:169:13
[01:26:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:26:50]     `rls` additionally enabled features {"parking_lot_core", "nightly"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libparking_lot-ed53e4fb9ce4db63.rlib"
[01:26:50]     `cargo` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libparking_lot-2694dc4c18db5d32.rlib"
[01:26:50] 
[01:26:50] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set

[01:26:50] travis_time:end:stage2-rls:start=1549330486924265527,finish=1549330770245436465,duration=283321170938

[01:26:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
---
travis_time:end:02cb9718:start=1549330772519413969,finish=1549330772527543940,duration=8129971
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02cc0c8e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e5e1233
travis_time:start:0e5e1233
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1232bd0f
$ dmesg | grep -i kill
