
Building stage2 tool rustfmt (x86_64-unknown-linux-gnu)                                                                                                                                                                                        
running: "/root/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "12" "-v" "--release" "--manifest-path" "/root/rust/src/tools/rustfmt/Cargo.toml" "--message-fo
rmat" "json-render-diagnostics"                                                                                                                                                                                                                
       Fresh cfg-if v0.1.10
---snip---
   Compiling rustc-ap-rustc_span v705.0.0                                                                                                                                                                                            [75/10642]
       Fresh rustc-workspace-hack v1.0.0 (/root/rust/src/tools/rustc-workspace-hack)                                                                                                                                                           
       Fresh cargo_metadata v0.8.2                                                                                                                                                                                                             
     Running `/root/rust/build/bootstrap/debug/rustc --crate-name rustc_ap_rustc_span --edition=2018 /root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/lib.rs --error-format=json --json=diagnostic-rendere
d-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C metadata=352613143fa6e035 -C extra-filename=-352613143fa6e035 --out-dir /root/rust/build/x86_64-unknown-linux-gnu/stage2-t
ools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/root/rust/build/x86_64-unknown-linux-gnu
/stage2-tools/release/deps --extern cfg_if=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcfg_if-11ae2b7fd4c81dfa.rmeta --extern md5=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x8
6_64-unknown-linux-gnu/release/deps/libmd5-0be4da8d393b6bbe.rmeta --extern rustc_arena=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_arena-19aa0d7adb6ebb18.rmeta --extern rus
tc_data_structures=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_data_structures-03eeaed872934108.rmeta --extern rustc_index=/root/rust/build/x86_64-unknown-linux-gnu/stage2-
tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_index-6b03c4d9223fde7a.rmeta --extern rustc_macros=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/librustc_ap_rustc_macros-06c3b01e551972b1.so --extern rust
c_serialize=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_serialize-13d6b937870e98b8.rmeta --extern scoped_tls=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-u
nknown-linux-gnu/release/deps/libscoped_tls-88a9cd40247d3de1.rmeta --extern sha1=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsha1-4737bbe5a1772a83.rmeta --extern sha2=/root/rust/build/x86
_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsha2-fa232244b8685719.rmeta --extern tracing=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-9c0a56605c9d31
a6.rmeta --extern unicode_width=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libunicode_width-7278b98d76c71889.rmeta --cap-lints allow -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib
' -Ztls-model=initial-exec -Z binary-dep-depinfo -L native=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/psm-0e0757d09e87b698/out`
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[0]: "CARGO"="/root/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[1]: "CARGO_CRATE_NAME"="rustc_ap_rustc_span"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[2]: "CARGO_INCREMENTAL"="0"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[3]: "CARGO_MAKEFLAGS"="--jobserver-fds=6,7 -j --jobserver-auth=6,7 -j"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[4]: "CARGO_MANIFEST_DIR"="/root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[5]: "CARGO_PKG_AUTHORS"="The Rust Project Developers"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[6]: "CARGO_PKG_DESCRIPTION"="Automatically published version of the package `rustc_span` in the rust-lang/rust repository from commit 37d067f5d71331d909102d142ecc50f577e07ae4 The publishing script
 for this crate lives at: https://github.com/alexcrichton/rustc-auto-publish\n            "
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[7]: "CARGO_PKG_HOMEPAGE"=""
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[8]: "CARGO_PKG_LICENSE"="MIT / Apache-2.0"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[9]: "CARGO_PKG_LICENSE_FILE"=""
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[10]: "CARGO_PKG_NAME"="rustc-ap-rustc_span"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[11]: "CARGO_PKG_REPOSITORY"="https://github.com/rust-lang/rust"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[12]: "CARGO_PKG_VERSION"="705.0.0"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[13]: "CARGO_PKG_VERSION_MAJOR"="705"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[14]: "CARGO_PKG_VERSION_MINOR"="0"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[15]: "CARGO_PKG_VERSION_PATCH"="0"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[16]: "CARGO_PKG_VERSION_PRE"=""
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[17]: "CARGO_PROFILE_RELEASE_DEBUG"="0"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[18]: "CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS"="false"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[19]: "CARGO_TARGET_DIR"="/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[20]: "RUSTBUILD_NATIVE_DIR"="/root/rust/build/x86_64-unknown-linux-gnu/native"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[21]: "RUSTC"="/root/rust/build/bootstrap/debug/rustc"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[22]: "RUSTC_BOOTSTRAP"="1"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[23]: "RUSTC_BREAK_ON_ICE"="1"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[24]: "RUSTC_ERROR_METADATA_DST"="/root/rust/build/tmp/extended-error-metadata"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[25]: "RUSTC_INSTALL_BINDIR"="bin"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[26]: "RUSTC_LIBDIR"="/root/rust/build/x86_64-unknown-linux-gnu/stage2/lib"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[27]: "RUSTC_REAL"="/root/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[28]: "RUSTC_SNAPSHOT"="/root/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[29]: "RUSTC_SNAPSHOT_LIBDIR"="/root/rust/build/x86_64-unknown-linux-gnu/stage2/lib"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[30]: "RUSTC_STAGE"="2"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[31]: "RUSTC_SYSROOT"="/root/rust/build/x86_64-unknown-linux-gnu/stage2"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[32]: "RUSTC_VERBOSE"="2"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[33]: "RUSTDOC"="/root/rust/build/bootstrap/debug/rustdoc"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[34]: "RUSTDOCFLAGS"="--crate-version 1.53.0-dev"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[35]: "RUSTDOC_REAL"="/path/to/nowhere/rustdoc/not/required"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[36]: "RUSTFLAGS"="-Zmacro-backtrace -Clink-args=-Wl,-rpath,$ORIGIN/../lib -Ztls-model=initial-exec"
[RUSTC-SHIM] rustc rustc_ap_rustc_span env[37]: "RUST_TEST_THREADS"="12"
[RUSTC-SHIM] rustc rustc_ap_rustc_span working directory: /root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0
[RUSTC-SHIM] rustc rustc_ap_rustc_span command: "LD_LIBRARY_PATH"="/root/rust/build/x86_64-unknown-linux-gnu/stage2/lib:/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps:/root/rust/build/x86_64-unknown-linux-gnu/stage2/li
b" "/root/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "rustc_ap_rustc_span" "--edition=2018" "/root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/lib.rs" "--error-format=json" "--j
son=diagnostic-rendered-ansi,artifacts" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=3" "-C" "embed-bitcode=no" "-C" "debuginfo=0" "-C" "metadata=352613143fa6e035" "-C" "extra-filename=-352613143fa6e035" "--out-dir"
 "/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/
deps" "-L" "dependency=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "cfg_if=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcfg_if-11ae2b7fd4c81dfa.rmeta" "
--extern" "md5=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libmd5-0be4da8d393b6bbe.rmeta" "--extern" "rustc_arena=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-
gnu/release/deps/librustc_ap_rustc_arena-19aa0d7adb6ebb18.rmeta" "--extern" "rustc_data_structures=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_data_structures-03eeaed872934
108.rmeta" "--extern" "rustc_index=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_index-6b03c4d9223fde7a.rmeta" "--extern" "rustc_macros=/root/rust/build/x86_64-unknown-linux-
gnu/stage2-tools/release/deps/librustc_ap_rustc_macros-06c3b01e551972b1.so" "--extern" "rustc_serialize=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_serialize-13d6b937870e98
b8.rmeta" "--extern" "scoped_tls=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-88a9cd40247d3de1.rmeta" "--extern" "sha1=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86
_64-unknown-linux-gnu/release/deps/libsha1-4737bbe5a1772a83.rmeta" "--extern" "sha2=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsha2-fa232244b8685719.rmeta" "--extern" "tracing=/root/rust
/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-9c0a56605c9d31a6.rmeta" "--extern" "unicode_width=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/li
bunicode_width-7278b98d76c71889.rmeta" "--cap-lints" "allow" "-Zmacro-backtrace" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Ztls-model=initial-exec" "-Z" "binary-dep-depinfo" "-L" "native=/root/rust/build/x86_64-unknown-linux-gnu/stage2-too
ls/x86_64-unknown-linux-gnu/release/build/psm-0e0757d09e87b698/out" "--sysroot" "/root/rust/build/x86_64-unknown-linux-gnu/stage2"
[RUSTC-SHIM] rustc rustc_ap_rustc_span sysroot: "/root/rust/build/x86_64-unknown-linux-gnu/stage2"
[RUSTC-SHIM] rustc rustc_ap_rustc_span libdir: "/root/rust/build/x86_64-unknown-linux-gnu/stage2/lib"
error[E0599]: no method named `expect_none` found for enum `Option<Fingerprint>` in the current scope
    --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/lib.rs:2003:48
     |
2003 |                 cache[index].replace(sub_hash).expect_none("Cache slot was filled");
     |                                                ^^^^^^^^^^^ method not found in `Option<Fingerprint>`

error[E0599]: no method named `expect_none` found for enum `Option<u32>` in the current scope
   --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/hygiene.rs:121:54
    |
121 |             expn_data.orig_id.replace(self.as_u32()).expect_none("orig_id should be None");
    |                                                      ^^^^^^^^^^^ method not found in `Option<u32>`

error[E0599]: no method named `expect_none` found for enum `Option<u32>` in the current scope
   --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/hygiene.rs:205:42
    |
205 |             data.orig_id.replace(raw_id).expect_none("orig_id should be None");
    |                                          ^^^^^^^^^^^ method not found in `Option<u32>`

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.
    Building 
Did not run successfully: exit code: 1ustc-ap-rustc_span                                                                                                                                                                         
"/root/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "rustc_ap_rustc_span" "--edition=2018" "/root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/lib.rs" "--error-format=json" "--json
=diagnostic-rendered-ansi,artifacts" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=3" "-C" "embed-bitcode=no" "-C" "debuginfo=0" "-C" "metadata=352613143fa6e035" "-C" "extra-filename=-352613143fa6e035" "--out-dir" "/
root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/dep
s" "-L" "dependency=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "cfg_if=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcfg_if-11ae2b7fd4c81dfa.rmeta" "--e
xtern" "md5=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libmd5-0be4da8d393b6bbe.rmeta" "--extern" "rustc_arena=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
/release/deps/librustc_ap_rustc_arena-19aa0d7adb6ebb18.rmeta" "--extern" "rustc_data_structures=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_data_structures-03eeaed872934108
.rmeta" "--extern" "rustc_index=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_index-6b03c4d9223fde7a.rmeta" "--extern" "rustc_macros=/root/rust/build/x86_64-unknown-linux-gnu
/stage2-tools/release/deps/librustc_ap_rustc_macros-06c3b01e551972b1.so" "--extern" "rustc_serialize=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_serialize-13d6b937870e98b8.
rmeta" "--extern" "scoped_tls=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-88a9cd40247d3de1.rmeta" "--extern" "sha1=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64
-unknown-linux-gnu/release/deps/libsha1-4737bbe5a1772a83.rmeta" "--extern" "sha2=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsha2-fa232244b8685719.rmeta" "--extern" "tracing=/root/rust/bu
ild/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-9c0a56605c9d31a6.rmeta" "--extern" "unicode_width=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libun
icode_width-7278b98d76c71889.rmeta" "--cap-lints" "allow" "-Zmacro-backtrace" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Ztls-model=initial-exec" "-Z" "binary-dep-depinfo" "-L" "native=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/
x86_64-unknown-linux-gnu/release/build/psm-0e0757d09e87b698/out" "--sysroot" "/root/rust/build/x86_64-unknown-linux-gnu/stage2"
-------------
error: could not compile `rustc-ap-rustc_span`

Caused by:
  process didn't exit successfully: `/root/rust/build/bootstrap/debug/rustc --crate-name rustc_ap_rustc_span --edition=2018 /root/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-705.0.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C metadata=352613143fa6e035 -C extra-filename=-352613143fa6e035 --out-dir /root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern cfg_if=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcfg_if-11ae2b7fd4c81dfa.rmeta --extern md5=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libmd5-0be4da8d393b6bbe.rmeta --extern rustc_arena=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_arena-19aa0d7adb6ebb18.rmeta --extern rustc_data_structures=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_data_structures-03eeaed872934108.rmeta --extern rustc_index=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_index-6b03c4d9223fde7a.rmeta --extern rustc_macros=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/librustc_ap_rustc_macros-06c3b01e551972b1.so --extern rustc_serialize=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_ap_rustc_serialize-13d6b937870e98b8.rmeta --extern scoped_tls=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-88a9cd40247d3de1.rmeta --extern sha1=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsha1-4737bbe5a1772a83.rmeta --extern sha2=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsha2-fa232244b8685719.rmeta --extern tracing=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-9c0a56605c9d31a6.rmeta --extern unicode_width=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libunicode_width-7278b98d76c71889.rmeta --cap-lints allow -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Z binary-dep-depinfo -L native=/root/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/psm-0e0757d09e87b698/out` (exit code: 1)
command did not execute successfully: "/root/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "12" "-v" "--release" "--manifest-path" "/root/rust/src/tools/rustfmt/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
  < ToolBuild { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, tool: "rustfmt", path: "src/tools/rustfmt", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] }
< Rustfmt { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, extra_features: [] }
Build completed successfully in 0:00:02
