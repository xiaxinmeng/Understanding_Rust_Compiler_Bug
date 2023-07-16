plain
[2022-05-21T13:57:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsxvgJE#cargo:0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
Running cargo-0.60.0: Debug + [Full]
[2022-05-21T13:58:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-21T13:58:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-21T13:58:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp09XFLr#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
collector error: Failed to profile 'cargo-0.60.0' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling openssl-sys v0.9.72
   Compiling serde_derive v1.0.136
   Compiling libnghttp2-sys v0.1.7+1.45.0
   Compiling libssh2-sys v0.2.23
   Compiling curl-sys v0.4.52+curl-7.81.0
   Compiling curl-sys v0.4.52+curl-7.81.0
   Compiling libgit2-sys v0.12.26+1.3.0
   Compiling globset v0.4.8
   Compiling vte v0.10.1
   Compiling sized-chunks v0.6.5
   Compiling env_logger v0.9.0
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libnghttp2-sys-0.1.7+1.45.0/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=321b7d6edba832f9", "-C", "extra-filename=-321b7d6edba832f9", "--out-dir", "/tmp/.tmp09XFLr/target/debug/build/libnghttp2-sys-321b7d6edba832f9", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "cc=/tmp/.tmp09XFLr/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libssh2-sys-0.2.23/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=f39976f95e1938aa", "-C", "extra-filename=-f39976f95e1938aa", "--out-dir", "/tmp/.tmp09XFLr/target/debug/build/libssh2-sys-f39976f95e1938aa", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "cc=/tmp/.tmp09XFLr/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp09XFLr/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libz-sys-1.1.3/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=67d6ec06c57c2fc0", "-C", "extra-filename=-67d6ec06c57c2fc0", "--out-dir", "/tmp/.tmp09XFLr/target/debug/build/libz-sys-67d6ec06c57c2fc0", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "cc=/tmp/.tmp09XFLr/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp09XFLr/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/libgit2-sys-0.12.26+1.3.0/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"https\"", "--cfg", "feature=\"libssh2-sys\"", "--cfg", "feature=\"openssl-sys\"", "--cfg", "feature=\"ssh\"", "--cfg", "feature=\"ssh_key_from_memory\"", "-C", "metadata=32f741cbfe8619f4", "-C", "extra-filename=-32f741cbfe8619f4", "--out-dir", "/tmp/.tmp09XFLr/target/debug/build/libgit2-sys-32f741cbfe8619f4", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "cc=/tmp/.tmp09XFLr/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp09XFLr/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_main", "/cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.72/build/main.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=19518b57d5c8cf0c", "-C", "extra-filename=-19518b57d5c8cf0c", "--out-dir", "/tmp/.tmp09XFLr/target/debug/build/openssl-sys-19518b57d5c8cf0c", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "autocfg=/tmp/.tmp09XFLr/target/debug/deps/libautocfg-5788d3b0c0ad14fb.rlib", "--extern", "cc=/tmp/.tmp09XFLr/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp09XFLr/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `libssh2-sys`
warning: build failed, waiting for other jobs to finish...
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "serde_derive", "/cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.136/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "proc-macro", "--emit=dep-info,link", "-C", "prefer-dynamic", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=643399df20f5eb13", "-C", "extra-filename=-643399df20f5eb13", "--out-dir", "/tmp/.tmp09XFLr/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "proc_macro2=/tmp/.tmp09XFLr/target/debug/deps/libproc_macro2-d937f1f82ffb437d.rlib", "--extern", "quote=/tmp/.tmp09XFLr/target/debug/deps/libquote-4040e84c0ddb8645.rlib", "--extern", "syn=/tmp/.tmp09XFLr/target/debug/deps/libsyn-0ed3f484ddf76ce0.rlib", "--extern", "proc_macro", "--cap-lints", "allow", "--cfg", "underscore_consts", "--cfg", "ptr_addr_of", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `libgit2-sys`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libz-sys-1.1.3/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"libc\"", "-C", "metadata=b5422a3e997764b8", "-C", "extra-filename=-b5422a3e997764b8", "--out-dir", "/tmp/.tmp09XFLr/target/debug/build/libz-sys-b5422a3e997764b8", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "cc=/tmp/.tmp09XFLr/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp09XFLr/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/curl-sys-0.4.52+curl-7.81.0/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"http2\"", "--cfg", "feature=\"libnghttp2-sys\"", "--cfg", "feature=\"openssl-sys\"", "--cfg", "feature=\"ssl\"", "-C", "metadata=b5a36cbe16d55871", "-C", "extra-filename=-b5a36cbe16d55871", "--out-dir", "/tmp/.tmp09XFLr/target/debug/build/curl-sys-b5a36cbe16d55871", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "cc=/tmp/.tmp09XFLr/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp09XFLr/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `openssl-sys`
error: could not compile `libz-sys`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "globset", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/globset-0.4.8/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=4df63928bbaaef4e", "-C", "extra-filename=-4df63928bbaaef4e", "--out-dir", "/tmp/.tmp09XFLr/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "aho_corasick=/tmp/.tmp09XFLr/target/debug/deps/libaho_corasick-056f10d3df8fc98c.rmeta", "--extern", "bstr=/tmp/.tmp09XFLr/target/debug/deps/libbstr-a560c43b7410e632.rmeta", "--extern", "fnv=/tmp/.tmp09XFLr/target/debug/deps/libfnv-22cd72556da4b99c.rmeta", "--extern", "log=/tmp/.tmp09XFLr/target/debug/deps/liblog-41607b4997ff699e.rmeta", "--extern", "regex=/tmp/.tmp09XFLr/target/debug/deps/libregex-dd9363dd410b6a25.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
error: could not compile `curl-sys`
exiting -- non-wrapped rustc
error: could not compile `globset`
error: could not compile `libnghttp2-sys`
error: could not compile `libz-sys`
error: could not compile `libz-sys`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "vte", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/vte-0.10.1/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"arrayvec\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"no_std\"", "-C", "metadata=528493d8a4f8c6fe", "-C", "extra-filename=-528493d8a4f8c6fe", "--out-dir", "/tmp/.tmp09XFLr/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "arrayvec=/tmp/.tmp09XFLr/target/debug/deps/libarrayvec-57012cb827268945.rmeta", "--extern", "utf8parse=/tmp/.tmp09XFLr/target/debug/deps/libutf8parse-e24d111b7af9b494.rmeta", "--extern", "vte_generate_state_changes=/tmp/.tmp09XFLr/target/debug/deps/libvte_generate_state_changes-dac5466506844490.so", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `vte`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "sized_chunks", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/sized-chunks-0.6.5/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=cf864b775ab88dcc", "-C", "extra-filename=-cf864b775ab88dcc", "--out-dir", "/tmp/.tmp09XFLr/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "bitmaps=/tmp/.tmp09XFLr/target/debug/deps/libbitmaps-3c4e76aef4de50e3.rmeta", "--extern", "typenum=/tmp/.tmp09XFLr/target/debug/deps/libtypenum-ae91be759e5f8db3.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `sized-chunks`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "env_logger", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/env_logger-0.9.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"atty\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"humantime\"", "--cfg", "feature=\"regex\"", "--cfg", "feature=\"termcolor\"", "-C", "metadata=f287f47a0ad49782", "-C", "extra-filename=-f287f47a0ad49782", "--out-dir", "/tmp/.tmp09XFLr/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp09XFLr/target/debug/deps", "--extern", "atty=/tmp/.tmp09XFLr/target/debug/deps/libatty-8e27291e809e89e5.rmeta", "--extern", "humantime=/tmp/.tmp09XFLr/target/debug/deps/libhumantime-69fede984985242f.rmeta", "--extern", "log=/tmp/.tmp09XFLr/target/debug/deps/liblog-41607b4997ff699e.rmeta", "--extern", "regex=/tmp/.tmp09XFLr/target/debug/deps/libregex-dd9363dd410b6a25.rmeta", "--extern", "termcolor=/tmp/.tmp09XFLr/target/debug/deps/libtermcolor-6a61078d9610ba74.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `env_logger`

 stdout=

6 normal benchmarks remaining
---
5 normal benchmarks remaining
Preparing hyper-0.14.18
[2022-05-21T13:58:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-21T13:58:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-21T13:58:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqSgPyA#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-21T13:58:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYaooWa#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-21T13:59:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-21T13:59:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-21T13:59:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwa71Ub#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
Running hyper-0.14.18: Opt + [Full]
[2022-05-21T13:59:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-21T13:59:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-21T13:59:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpt0bq8A#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
4 normal benchmarks remaining
Preparing regex-1.5.5
[2022-05-21T13:59:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-21T13:59:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-21T13:59:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDiMbQo#regex:1.5.5" "--release" "--" "--skip-this-rustc"
[2022-05-21T13:59:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbYNT2G#regex:1.5.5" "--" "--skip-this-rustc"
[2022-05-21T14:00:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-21T14:00:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-21T14:00:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplNrR4N#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
Running regex-1.5.5: Opt + [Full]
[2022-05-21T14:00:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-21T14:00:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-21T14:00:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpv2rDIN#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
3 normal benchmarks remaining
Preparing ripgrep-13.0.0
[2022-05-21T14:00:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-21T14:00:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-21T14:00:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCmuIZS#ripgrep:13.0.0" "--release" "--" "--skip-this-rustc"
[2022-05-21T14:00:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJmBLrX#ripgrep:13.0.0" "--" "--skip-this-rustc"
[2022-05-21T14:01:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-21T14:01:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-21T14:01:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOcjY7F#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-21T14:01:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOcjY7F#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
collector error: Failed to profile 'ripgrep-13.0.0' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling ripgrep v13.0.0 (/tmp/.tmpOcjY7F)
   Compiling grep-printer v0.1.6
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "grep_printer", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/grep-printer-0.1.6/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"base64\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"serde\"", "--cfg", "feature=\"serde1\"", "--cfg", "feature=\"serde_json\"", "-C", "metadata=fa85f87ca76a5e7e", "-C", "extra-filename=-fa85f87ca76a5e7e", "--out-dir", "/tmp/.tmpOcjY7F/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpOcjY7F/target/debug/deps", "--extern", "base64=/tmp/.tmpOcjY7F/target/debug/deps/libbase64-4563507a4eb0b4b2.rmeta", "--extern", "bstr=/tmp/.tmpOcjY7F/target/debug/deps/libbstr-75533cde86f1d72a.rmeta", "--extern", "grep_matcher=/tmp/.tmpOcjY7F/target/debug/deps/libgrep_matcher-474896b564839c3e.rmeta", "--extern", "grep_searcher=/tmp/.tmpOcjY7F/target/debug/deps/libgrep_searcher-954df40f2577ad84.rmeta", "--extern", "serde=/tmp/.tmpOcjY7F/target/debug/deps/libserde-1c71b8572d8d81bd.rmeta", "--extern", "serde_json=/tmp/.tmpOcjY7F/target/debug/deps/libserde_json-32f45092aa8bbe64.rmeta", "--extern", "termcolor=/tmp/.tmpOcjY7F/target/debug/deps/libtermcolor-6a61078d9610ba74.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `grep-printer`


 stdout=


2 normal benchmarks remaining
Preparing serde-1.0.136
[2022-05-21T14:01:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-21T14:01:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-21T14:01:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplEL6CY#serde:1.0.136" "--" "--skip-this-rustc"
[2022-05-21T14:01:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfByzra#serde:1.0.136" "--release" "--" "--skip-this-rustc"
[2022-05-21T14:01:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-21T14:01:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-21T14:01:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp3K5Yg#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
