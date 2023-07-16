
[RUSTC-TIMING] build_script_main test:false 5.100
The following warnings were emitted during compilation:

warning: error: Connection to server timed out

error: failed to run custom build command for `libnghttp2-sys v0.1.2`

Caused by:
  process didn't exit successfully: `/Users/runner/work/1/s/build/x86_64-apple-darwin/stage1-tools/release/build/libnghttp2-sys-4027708d4b09760f/build-script-build` (exit code: 1)
--- stdout
TARGET = Some("x86_64-apple-darwin")
OPT_LEVEL = Some("3")
HOST = Some("x86_64-apple-darwin")
CC_x86_64-apple-darwin = Some("sccache /Users/runner/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang")
CFLAGS_x86_64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/1/s=/rustc/7fe2b23bd5869bb62c8107d59d9ca145a3ac8e48")
CRATE_CC_NO_DEFAULTS = None
DEBUG = Some("false")
running: "sccache" "/Users/runner/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/1/s=/rustc/7fe2b23bd5869bb62c8107d59d9ca145a3ac8e48" "-I" "nghttp2/lib/includes" "-I" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-3462f163910044cd/out/i/include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-DHAVE_ARPA_INET_H" "-o" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-3462f163910044cd/out/i/lib/nghttp2/lib/nghttp2_buf.o" "-c" "nghttp2/lib/nghttp2_buf.c"
cargo:warning=error: Connection to server timed out
exit code: 2
running: "sccache" "/Users/runner/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/1/s=/rustc/7fe2b23bd5869bb62c8107d59d9ca145a3ac8e48" "-I" "nghttp2/lib/includes" "-I" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-3462f163910044cd/out/i/include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-DHAVE_ARPA_INET_H" "-o" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-3462f163910044cd/out/i/lib/nghttp2/lib/nghttp2_callbacks.o" "-c" "nghttp2/lib/nghttp2_callbacks.c"
exit code: 0

--- stderr


error occurred: Command "sccache" "/Users/runner/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/1/s=/rustc/7fe2b23bd5869bb62c8107d59d9ca145a3ac8e48" "-I" "nghttp2/lib/includes" "-I" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-3462f163910044cd/out/i/include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-DHAVE_ARPA_INET_H" "-o" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-3462f163910044cd/out/i/lib/nghttp2/lib/nghttp2_buf.o" "-c" "nghttp2/lib/nghttp2_buf.c" with args "clang" did not execute successfully (status code exit code: 2).
