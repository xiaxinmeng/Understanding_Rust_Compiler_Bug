plain
[01:37:38] [RUSTC-TIMING] cargo_fmt test:false 14.689
[01:37:42] [RUSTC-TIMING] git_rustfmt test:false 18.168
[01:37:44] [RUSTC-TIMING] rustfmt test:false 19.769
[01:37:44]     Finished release [optimized] target(s) in 6m 20s
[01:37:44] duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:37:44] the following dependencies are duplicated although they have the same features enabled:
[01:37:44] the following dependencies are duplicated although they have the same features enabled:
[01:37:44]   failure 0.1.5 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libfailure-dfdb55d8424618f5.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libfailure-c2b80a33144175bf.rlib")
[01:37:44]   serde 1.0.92 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde-201079034d802c6e.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde-08db5828731d83c9.rlib")
[01:37:44]   semver 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libsemver-9a0c1422779f4c1b.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libsemver-0cf9ea6cc13f0007.rlib")
[01:37:44]   rand_pcg 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_pcg-7a5bd2cad3c530ab.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_pcg-55bb30cafb29544d.rlib")
[01:37:44]   rand_chacha 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_chacha-94d7240688ef2945.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_chacha-17327c1445f7f60e.rlib")
[01:37:44]   serde_json 1.0.33 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde_json-d2f8d5876d97d75e.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde_json-8e1822fbefffd454.rlib")
[01:37:44]   toml 0.5.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libtoml-8a4f8d5322d715ca.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libtoml-a1cf0b5a714a0ad9.rlib")
[01:37:44]   rand 0.6.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand-68e2f5b4d394760b.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand-16e80d7464516488.rlib")
[01:37:44]   parking_lot_core 0.4.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot_core-b64674a674a503f2.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot_core-ff37c2f3949cc050.rlib")
[01:37:44]   parking_lot 0.7.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot-da88124b8b78ca98.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot-713b7961156a5682.rlib")
[01:37:44]   openssl-sys 0.9.43 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl_sys-ba7bc02e1e222fba.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl_sys-c681161cfedf1477.rlib")
[01:37:44]   openssl 0.10.16 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl-cd173ebf7bbbcdd2.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl-e4f4a7f62ada7fe9.rlib")
[01:37:44]   curl-sys 0.4.18 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libcurl_sys-62bb58ac1d16724b.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libcurl_sys-1086a81ede1afc3e.rlib")
[01:37:44]   rustc-workspace-hack 1.0.0 (path+file:///checkout/src/tools/rustc-workspace-hack)
[01:37:44]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librustc_workspace_hack-536e3cf7d5e1e864.rlib")
[01:37:44]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librustc_workspace_hack-8dca53b1e1254fab.rlib")
[01:37:44] the following dependencies have different features:
[01:37:44] 
[01:37:44] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
[01:37:44] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:172:13
[01:37:44] travis_fold:end:stage1-rustfmt

[01:37:44] travis_time:end:stage1-rustfmt:start=1560484052873960602,finish=1560484433903042993,duration=381029082391

---
travis_time:end:005b9025:start=1560484435891575241,finish=1560484435901695891,duration=10120650
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:041ed5ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22d66eca
travis_time:start:22d66eca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a934587
$ dmesg | grep -i kill
