plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:069992f5
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[02:08:11]    Compiling chrono v0.4.6
[02:08:14]    Compiling vergen v3.0.4
[02:08:17]    Compiling miri v0.1.0 (/checkout/src/tools/miri)
[02:09:02]     Finished release [optimized] target(s) in 54.31s
[02:09:02] duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[02:09:02] 
[02:09:02] the following dependencies are duplicated although they have the same features enabled:
[02:09:02]   lock_api 0.1.3 (registry+https://github.com/rust-lang/crates.io-index)
[02:09:02]     `miri` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblock_api-d1cfcbed72736cd0.rlib")
[02:09:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblock_api-7317565db4aa825a.rlib")
[02:09:02]   parking_lot 0.6.4 (registry+https://github.com/rust-lang/crates.io-index)
[02:09:02]     `miri` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-296134448e3a9962.rlib")
[02:09:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-cbd5205c8879a30d.rlib")
[02:09:02]   rustc-workspace-hack 1.0.0 (path+file:///checkout/src/tools/rustc-workspace-hack)
[02:09:02]     `miri` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_workspace_hack-574c161b5c78b8cb.rlib")
[02:09:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_workspace_hack-831a5e7043354d91.rlib")
[02:09:02] the following dependencies have different features:
[02:09:02]   scopeguard 0.3.3 (registry+https://github.com/rust-lang/crates.io-index)
[02:09:02]     `miri` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libscopeguard-253cb60950318616.rlib"
[02:09:02]     `cargo` additionally enabled features {"use_std", "default"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libscopeguard-c042589f14e73ac1.rlib"
[02:09:02]   byteorder 1.2.7 (registry+https://github.com/rust-lang/crates.io-index)
[02:09:02]     `miri` additionally enabled features {"i128"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libbyteorder-d19c3cbbf5a428d6.rlib"
[02:09:02]     `rls` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libbyteorder-f409f2b66e5f3bba.rlib"
[02:09:02]   curl-sys 0.4.15 (registry+https://github.com/rust-lang/crates.io-index)
[02:09:02]     `miri` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcurl_sys-2d3c8813bcfc2ee1.rlib"
[02:09:02]     `cargo` additionally enabled features {"http2", "libnghttp2-sys"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcurl_sys-160bf63e87ae4364.rlib"
[02:09:02] 
[02:09:02] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
[02:09:02] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:170:13
[02:09:02] travis_fold:end:stage2-miri

[02:09:02] travis_time:end:stage2-miri:start=1546124120605405290,finish=1546124174966069338,duration=54360664048

---
travis_time:end:046bb5bc:start=1546124178964366386,finish=1546124178978174017,duration=13807631
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16b34293
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04681b7a
travis_time:start:04681b7a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:005f2624
$ dmesg | grep -i kill
