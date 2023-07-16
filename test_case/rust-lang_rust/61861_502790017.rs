plain
[01:36:25] [RUSTC-TIMING] rustfmt test:false 19.146
[01:36:27] [RUSTC-TIMING] git_rustfmt test:false 21.185
[01:36:27] [RUSTC-TIMING] cargo_fmt test:false 21.328
[01:36:27]     Finished release [optimized] target(s) in 7m 28s
[01:36:27] duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:36:27] the following dependencies are duplicated although they have the same features enabled:
[01:36:27] the following dependencies are duplicated although they have the same features enabled:
[01:36:27]   failure 0.1.5 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libfailure-554472df3e9547e6.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libfailure-533763ff6e57cf06.rlib")
[01:36:27]   serde 1.0.82 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde-d77291c60fb4e754.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde-e4d891adb48592aa.rlib")
[01:36:27]   semver 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libsemver-48aeead1db768744.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libsemver-974bcb68eaa0c118.rlib")
[01:36:27]   toml 0.5.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libtoml-7d48f24f29532ed4.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libtoml-eb061cb413aeb1f5.rlib")
[01:36:27]   serde_json 1.0.33 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde_json-39850ae3e5a17aec.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde_json-636f4ff29f5a27f7.rlib")
[01:36:27]   rand_pcg 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_pcg-48f8ae2dcbbd24d3.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_pcg-887f624d78e70919.rlib")
[01:36:27]   rand_chacha 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_chacha-cd8b3666b74b08c8.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_chacha-214c1b2723229780.rlib")
[01:36:27]   rand 0.6.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand-54b03bb7042dfd23.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand-d38def963f352073.rlib")
[01:36:27]   parking_lot_core 0.4.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot_core-b8635598c4800eda.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot_core-b12c11145b2cb86d.rlib")
[01:36:27]   parking_lot 0.7.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot-2d2e063134f9543e.rlib")
[01:36:27] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:172:13
[01:36:27] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot-8ce7891b46a6748b.rlib")
[01:36:27]   openssl-sys 0.9.43 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl_sys-c4dcb184b453a698.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl_sys-16a3aea6897e769e.rlib")
[01:36:27]   openssl 0.10.16 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl-60bb96a86ca5ecf2.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl-bac3e53163d8f089.rlib")
[01:36:27]   curl-sys 0.4.18 (registry+https://github.com/rust-lang/crates.io-index)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libcurl_sys-076a21fd7a3769fc.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libcurl_sys-e67b1187198e9485.rlib")
[01:36:27]   rustc-workspace-hack 1.0.0 (path+file:///checkout/src/tools/rustc-workspace-hack)
[01:36:27]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librustc_workspace_hack-737534b0114f188b.rlib")
[01:36:27]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librustc_workspace_hack-cf95dde0820924d3.rlib")
[01:36:27] the following dependencies have different features:
[01:36:27] 
[01:36:27] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set

[01:36:27] travis_time:end:stage1-rustfmt:start=1560794119546788478,finish=1560794567913898142,duration=448367109664

[01:36:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build x86_64-unknown-linux-musl
---
travis_time:end:058ee388:start=1560794569753274113,finish=1560794569767440517,duration=14166404
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0732a27e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0357d7ae
travis_time:start:0357d7ae
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ca7f135
$ dmesg | grep -i kill
