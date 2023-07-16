plain
[RUSTC-TIMING] build_script_build test:false 0.226
   Compiling openssl v0.10.38
   Compiling os_str_bytes v6.0.0
[RUSTC-TIMING] build_script_build test:false 0.333
   Compiling pem-rfc7468 v0.6.0
[RUSTC-TIMING] build_script_build test:false 0.376
[RUSTC-TIMING] build_script_build test:false 0.386
[RUSTC-TIMING] build_script_build test:false 0.219
[RUSTC-TIMING] const_oid test:false 0.471
---
[RUSTC-TIMING] proc_macro2 test:false 3.644
[RUSTC-TIMING] proc_macro_error_attr test:false 0.743
   Compiling ppv-lite86 v0.2.8
[RUSTC-TIMING] build_script_build test:false 0.390
[RUSTC-TIMING] pem_rfc7468 test:false 1.208
[RUSTC-TIMING] crc32fast test:false 0.219
[RUSTC-TIMING] crc32fast test:false 0.219
   Compiling spki v0.6.0
   Compiling strsim v0.10.0
[RUSTC-TIMING] hashbrown test:false 0.804
   Compiling pkcs8 v0.9.0
[RUSTC-TIMING] heck test:false 0.327
---
   Compiling openssl-src v111.22.0+1.1.1q
[RUSTC-TIMING] same_file test:false 0.194
   Compiling hmac v0.12.1
[RUSTC-TIMING] build_script_build test:false 0.332
   Compiling hkdf v0.12.3
[RUSTC-TIMING] hmac test:false 0.123
[RUSTC-TIMING] strsim test:false 1.498
   Compiling sha2 v0.10.6
[RUSTC-TIMING] openssl_src test:false 0.283
   Compiling walkdir v2.3.2
   Compiling walkdir v2.3.2
[RUSTC-TIMING] signature test:false 0.110
   Compiling tar v0.4.38
[RUSTC-TIMING] hkdf test:false 0.115
   Compiling rand_chacha v0.3.0
[RUSTC-TIMING] spki test:false 1.642
   Compiling bitmaps v2.1.0
[RUSTC-TIMING] rand_chacha test:false 0.331
   Compiling rfc6979 v0.3.1
[RUSTC-TIMING] filetime test:false 0.301
[RUSTC-TIMING] filetime test:false 0.301
   Compiling elliptic-curve v0.12.3
[RUSTC-TIMING] pkcs8 test:false 1.816
[RUSTC-TIMING] rfc6979 test:false 0.083
   Compiling clap_lex v0.2.2
[RUSTC-TIMING] cc test:false 1.544
   Compiling libz-sys v1.1.3
   Compiling libz-sys v1.1.3
[RUSTC-TIMING] socket2 test:false 1.219
   Compiling openssl-sys v0.9.72
[RUSTC-TIMING] elliptic_curve test:false 0.348
[RUSTC-TIMING] build_script_build test:false 0.417
   Compiling curl-sys v0.4.59+curl-7.86.0
[RUSTC-TIMING] build_script_build test:false 0.398
[RUSTC-TIMING] sec1 test:false 1.789
---
   Compiling static_assertions v1.1.0
[RUSTC-TIMING] build_script_build test:false 0.557
   Compiling time-core v0.1.0
[RUSTC-TIMING] build_script_build test:false 0.266
   Compiling fiat-crypto v0.1.17
[RUSTC-TIMING] either test:false 0.131
   Compiling textwrap v0.15.0
[RUSTC-TIMING] proc_macro_error test:false 0.469
   Compiling vte v0.3.3
---
[RUSTC-TIMING] time test:false 7.534
[RUSTC-TIMING] tar test:false 2.580
[RUSTC-TIMING] combine test:false 7.657
[RUSTC-TIMING] serde_derive test:false 6.035
   Compiling orion v0.17.3
[RUSTC-TIMING] fiat_crypto test:false 8.270
[RUSTC-TIMING] serde_derive test:false 6.324
   Compiling serde v1.0.152
[RUSTC-TIMING] libz_sys test:false 0.067
   Compiling flate2 v1.0.23
---
[RUSTC-TIMING] rustfix test:false 1.253
[RUSTC-TIMING] serde_json test:false 3.529
[RUSTC-TIMING] semver test:false 0.832
[RUSTC-TIMING] os_info test:false 1.053
[RUSTC-TIMING] pasetors test:false 2.729
[RUSTC-TIMING] libssh2_sys test:false 0.134
[RUSTC-TIMING] curl_sys test:false 0.129
   Compiling crates-io v0.35.1 (/checkout/src/tools/cargo/crates/crates-io)
   Compiling crypto-hash v0.3.4
---
[RUSTC-TIMING] rustc_workspace_hack test:false 0.037
   Compiling rls v2.0.0 (/checkout/src/tools/rls)
[RUSTC-TIMING] rls test:false 0.898
    Finished release [optimized] target(s) in 32.46s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies have different features:
the following dependencies have different features:
  getrandom 0.2.8 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/mips64el-unknown-linux-gnuabi64/release/deps/libgetrandom-fe41900f8ca686fb.rlib"
    `cargo` additionally enabled features {"wasm-bindgen", "js", "js-sys"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/mips64el-unknown-linux-gnuabi64/release/deps/libgetrandom-704177f6a79dc3e8.rlib"


to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:201:13
