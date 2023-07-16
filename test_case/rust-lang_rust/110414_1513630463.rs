plain
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/release/build/libnghttp2-sys-501a42d5d1e0b9a6/build-script-build` (exit status: 1)
  --- stdout
  TARGET = Some("x86_64-apple-darwin")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-apple-darwin")
  cargo:rerun-if-env-changed=CC_x86_64-apple-darwin
  CC_x86_64-apple-darwin = None
  cargo:rerun-if-env-changed=CC_x86_64_apple_darwin
  CC_x86_64_apple_darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang")
  cargo:rerun-if-env-changed=CFLAGS_x86_64-apple-darwin
  CFLAGS_x86_64-apple-darwin = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64_apple_darwin
  CFLAGS_x86_64_apple_darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/e9d48e9d574fb30e8017a824a0534006a5f9364a")
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  DEBUG = Some("false")
  DEBUG = Some("false")
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/e9d48e9d574fb30e8017a824a0534006a5f9364a" "-I" "nghttp2/lib/includes" "-I" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-2511d6699e96c0f1/out/i/include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-DHAVE_ARPA_INET_H" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-2511d6699e96c0f1/out/i/lib/nghttp2/lib/nghttp2_buf.o" "-c" "nghttp2/lib/nghttp2_buf.c"
  cargo:warning=sccache: error: Timed out waiting for server startup

  --- stderr



  error occurred: Command "sccache" "/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/e9d48e9d574fb30e8017a824a0534006a5f9364a" "-I" "nghttp2/lib/includes" "-I" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-2511d6699e96c0f1/out/i/include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-DHAVE_ARPA_INET_H" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-2511d6699e96c0f1/out/i/lib/nghttp2/lib/nghttp2_buf.o" "-c" "nghttp2/lib/nghttp2_buf.c" with args "clang" did not execute successfully (status code exit status: 2).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] annotate_snippets test:false 329.803
[RUSTC-TIMING] diff test:false 153.393
---
[RUSTC-TIMING] term test:false 510.823
[RUSTC-TIMING] toml test:false 579.727
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 1, host: x86_64-apple-darwin }, target: x86_64-apple-darwin, tool: "rustfmt", path: "src/tools/rustfmt", mode: ToolRustc, is_optional_tool: true, source_type: InTree, extra_features: [], allow_features: "" } -- 581.176
[TIMING] tool::Rustfmt { compiler: Compiler { stage: 1, host: x86_64-apple-darwin }, target: x86_64-apple-darwin, extra_features: [] } -- 0.001
thread 'main' panicked at 'rustfmt expected to build - essential tool', dist.rs:1306:14
Build completed unsuccessfully in 4:37:15
