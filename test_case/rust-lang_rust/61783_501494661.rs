plain
[00:09:57]   Downloaded utf8parse v0.1.1
[00:09:57]   Downloaded im-rc v13.0.0
[00:09:57]   Downloaded constant_time_eq v0.1.3
[00:09:57]   Downloaded openssl-sys v0.9.43
[00:09:57]   Downloaded varisat-internal-proof v0.2.1
[00:09:57]   Downloaded lsp-codec v0.1.2
[00:09:57]   Downloaded rustc-ap-rustc_target v407.0.0
[00:09:57]   Downloaded curl v0.4.21
[00:09:57]   Downloaded regex v1.1.6
---
[00:09:58]   Downloaded tokio-process v0.2.3
[00:09:58]   Downloaded ucd-util v0.1.3
[00:09:58]   Downloaded quick-error v1.2.2
[00:09:58]   Downloaded vcpkg v0.2.6
[00:09:58]   Downloaded varisat-formula v0.2.1
[00:09:58]   Downloaded precomputed-hash v0.1.1
[00:09:58]   Downloaded syn v0.11.11
[00:09:58]   Downloaded iovec v0.1.2
[00:09:58]   Downloaded bytecount v0.5.1
---
[00:10:01]   Downloaded tokio-codec v0.1.1
[00:10:01]   Downloaded termcolor v1.0.4
[00:10:01]   Downloaded punycode v0.4.0
[00:10:01]   Downloaded percent-encoding v1.0.1
[00:10:01]   Downloaded varisat-checker v0.2.1
[00:10:01]   Downloaded wincolor v1.0.1
[00:10:01]   Downloaded leb128 v0.2.4
[00:10:01]   Downloaded url v1.7.2
[00:10:01]   Downloaded pest v2.1.0
---
[00:10:04]   Downloaded void v1.0.2
[00:10:04]   Downloaded measureme v0.3.0
[00:10:04]   Downloaded regex-syntax v0.5.6
[00:10:04]   Downloaded memmap v0.6.2
[00:10:04]   Downloaded varisat v0.2.1
[00:10:04]   Downloaded itertools v0.7.8
[00:10:04]   Downloaded rustc-rayon v0.2.0
[00:10:04]   Downloaded rusty-fork v0.2.1
[00:10:04]   Downloaded rustc-rayon-core v0.2.0
---
[00:10:05]   Downloaded futf v0.1.4
[00:10:05]   Downloaded fst v0.3.0
[00:10:05]   Downloaded shlex v0.1.1
[00:10:05]   Downloaded hashbrown v0.3.1
[00:10:05]   Downloaded varisat-internal-macros v0.2.1
[00:10:05]   Downloaded log v0.4.6
[00:10:05]   Downloaded varisat-dimacs v0.2.1
[00:10:05]   Downloaded unicode-xid v0.0.4
[00:10:06]   Downloaded annotate-snippets v0.5.0
[00:10:06]   Downloaded tokio-uds v0.2.5
[00:10:06]   Downloaded rls-data v0.19.0
---
[01:29:04]    Compiling cargo v0.38.0 (/checkout/src/tools/cargo)
[01:29:07] warning: use of deprecated item 'std::sync::ONCE_INIT': the `new` function is now preferred
[01:29:07]   --> src/tools/cargo/src/cargo/util/config.rs:13:23
[01:29:07]    |
[01:29:07] 13 | use std::sync::{Once, ONCE_INIT};
[01:29:07]    |
[01:29:07]    = note: #[warn(deprecated)] on by default
[01:29:07] 
[01:29:07] warning: use of deprecated item 'std::sync::ONCE_INIT': the `new` function is now preferred
[01:29:07] warning: use of deprecated item 'std::sync::ONCE_INIT': the `new` function is now preferred
[01:29:07]   --> src/tools/cargo/src/cargo/util/config.rs:92:29
[01:29:07]    |
[01:29:07] 92 |         static INIT: Once = ONCE_INIT;
[01:29:07]    |                             ^^^^^^^^^ help: replace the use of the deprecated item: `Once::new()`
[01:32:12] [RUSTC-TIMING] cargo test:false 188.296
[01:32:35] [RUSTC-TIMING] cargo test:false 22.470
[01:32:35]     Finished release [optimized] target(s) in 11m 18s
[01:32:35] travis_fold:end:stage1-cargo
---
[01:38:56] [RUSTC-TIMING] cargo_fmt test:false 13.227
[01:39:01] [RUSTC-TIMING] git_rustfmt test:false 17.942
[01:39:02] [RUSTC-TIMING] rustfmt test:false 19.601
[01:39:02]     Finished release [optimized] target(s) in 6m 16s
[01:39:02] duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:39:02] the following dependencies are duplicated although they have the same features enabled:
[01:39:02] the following dependencies are duplicated although they have the same features enabled:
[01:39:02]   failure 0.1.5 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libfailure-d0d57fac6967d073.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libfailure-f821fd9952a6ac41.rlib")
[01:39:02]   serde 1.0.92 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde-f47b72f50d7afe2f.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde-1ebfd4057e123933.rlib")
[01:39:02]   semver 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libsemver-0b2039d945c03ddc.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libsemver-27ac6b550874d378.rlib")
[01:39:02]   toml 0.5.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libtoml-059810112f4c789a.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libtoml-88d9b9fdbf87692d.rlib")
[01:39:02]   rand_pcg 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_pcg-ec7bf64835d11f84.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_pcg-da05c5272f25ee68.rlib")
[01:39:02]   serde_json 1.0.33 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde_json-54fa4959b98434c9.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libserde_json-512a8ec035721599.rlib")
[01:39:02]   rand_chacha 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_chacha-01457aabebd3f4aa.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand_chacha-c57a31651222c829.rlib")
[01:39:02]   rand 0.6.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand-86fee9e8ae8e0ff3.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librand-b97ee5e8426ef05a.rlib")
[01:39:02]   parking_lot_core 0.4.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot_core-467191642a8163ad.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot_core-9c9d009c77aea2b0.rlib")
[01:39:02]   parking_lot 0.7.1 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot-3bfd0b3a78b9ca1d.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libparking_lot-f3817df5f2784fd2.rlib")
[01:39:02]   openssl-sys 0.9.43 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl_sys-c7119caf06519f4d.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl_sys-b8c4a742138d2e01.rlib")
[01:39:02]   openssl 0.10.16 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl-0713f3f1b7513cfd.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libopenssl-52ce156f9e4872bf.rlib")
[01:39:02]   curl-sys 0.4.18 (registry+https://github.com/rust-lang/crates.io-index)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libcurl_sys-0b544bdaf2945795.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/libcurl_sys-8e254ee709343ea4.rlib")
[01:39:02]   rustc-workspace-hack 1.0.0 (path+file:///checkout/src/tools/rustc-workspace-hack)
[01:39:02]     `rustfmt` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librustc_workspace_hack-043d03fd773a9d9c.rlib")
[01:39:02]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-musl/stage1-tools/x86_64-unknown-linux-musl/release/deps/librustc_workspace_hack-11d09635256c95c8.rlib")
[01:39:02] the following dependencies have different features:
[01:39:02] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:172:13
[01:39:02] 
[01:39:02] 
[01:39:02] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set

[01:39:03] travis_time:end:stage1-rustfmt:start=1560382764408578506,finish=1560383140691233804,duration=376282655298

[01:39:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build x86_64-unknown-linux-musl
---
travis_time:end:30901000:start=1560383142662687297,finish=1560383142674685876,duration=11998579
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c0eae04
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00d3d371
travis_time:start:00d3d371
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14092b96
$ dmesg | grep -i kill
