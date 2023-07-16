plain
[2022-04-30T12:44:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWRkG7F#cargo:0.60.0" "--lib" "--" "--skip-this-rustc"
Running cargo-0.60.0: Debug + [Full]
[2022-04-30T12:45:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:45:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-30T12:45:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp24OvoS#cargo:0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
collector error: Failed to profile 'cargo-0.60.0' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling openssl-sys v0.9.72
   Compiling libnghttp2-sys v0.1.7+1.45.0
   Compiling serde_derive v1.0.136
   Compiling curl-sys v0.4.52+curl-7.81.0
   Compiling libssh2-sys v0.2.23
   Compiling libssh2-sys v0.2.23
   Compiling libgit2-sys v0.12.26+1.3.0
   Compiling url v2.2.2
   Compiling env_logger v0.9.0
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_main", "/cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.72/build/main.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=19518b57d5c8cf0c", "-C", "extra-filename=-19518b57d5c8cf0c", "--out-dir", "/tmp/.tmp24OvoS/target/debug/build/openssl-sys-19518b57d5c8cf0c", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "autocfg=/tmp/.tmp24OvoS/target/debug/deps/libautocfg-5788d3b0c0ad14fb.rlib", "--extern", "cc=/tmp/.tmp24OvoS/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp24OvoS/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/curl-sys-0.4.52+curl-7.81.0/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"http2\"", "--cfg", "feature=\"libnghttp2-sys\"", "--cfg", "feature=\"openssl-sys\"", "--cfg", "feature=\"ssl\"", "-C", "metadata=b5a36cbe16d55871", "-C", "extra-filename=-b5a36cbe16d55871", "--out-dir", "/tmp/.tmp24OvoS/target/debug/build/curl-sys-b5a36cbe16d55871", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "cc=/tmp/.tmp24OvoS/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp24OvoS/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libssh2-sys-0.2.23/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=f39976f95e1938aa", "-C", "extra-filename=-f39976f95e1938aa", "--out-dir", "/tmp/.tmp24OvoS/target/debug/build/libssh2-sys-f39976f95e1938aa", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "cc=/tmp/.tmp24OvoS/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp24OvoS/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/libgit2-sys-0.12.26+1.3.0/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"https\"", "--cfg", "feature=\"libssh2-sys\"", "--cfg", "feature=\"openssl-sys\"", "--cfg", "feature=\"ssh\"", "--cfg", "feature=\"ssh_key_from_memory\"", "-C", "metadata=32f741cbfe8619f4", "-C", "extra-filename=-32f741cbfe8619f4", "--out-dir", "/tmp/.tmp24OvoS/target/debug/build/libgit2-sys-32f741cbfe8619f4", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "cc=/tmp/.tmp24OvoS/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp24OvoS/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
   Compiling strip-ansi-escapes v0.1.1
   Compiling strip-ansi-escapes v0.1.1
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "serde_derive", "/cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.136/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "proc-macro", "--emit=dep-info,link", "-C", "prefer-dynamic", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=643399df20f5eb13", "-C", "extra-filename=-643399df20f5eb13", "--out-dir", "/tmp/.tmp24OvoS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "proc_macro2=/tmp/.tmp24OvoS/target/debug/deps/libproc_macro2-d937f1f82ffb437d.rlib", "--extern", "quote=/tmp/.tmp24OvoS/target/debug/deps/libquote-4040e84c0ddb8645.rlib", "--extern", "syn=/tmp/.tmp24OvoS/target/debug/deps/libsyn-0ed3f484ddf76ce0.rlib", "--extern", "proc_macro", "--cap-lints", "allow", "--cfg", "underscore_consts", "--cfg", "ptr_addr_of", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libz-sys-1.1.3/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"libc\"", "-C", "metadata=b5422a3e997764b8", "-C", "extra-filename=-b5422a3e997764b8", "--out-dir", "/tmp/.tmp24OvoS/target/debug/build/libz-sys-b5422a3e997764b8", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "cc=/tmp/.tmp24OvoS/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp24OvoS/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libnghttp2-sys-0.1.7+1.45.0/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=321b7d6edba832f9", "-C", "extra-filename=-321b7d6edba832f9", "--out-dir", "/tmp/.tmp24OvoS/target/debug/build/libnghttp2-sys-321b7d6edba832f9", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "cc=/tmp/.tmp24OvoS/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libz-sys-1.1.3/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=67d6ec06c57c2fc0", "-C", "extra-filename=-67d6ec06c57c2fc0", "--out-dir", "/tmp/.tmp24OvoS/target/debug/build/libz-sys-67d6ec06c57c2fc0", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "cc=/tmp/.tmp24OvoS/target/debug/deps/libcc-e8103b02c5a08642.rlib", "--extern", "pkg_config=/tmp/.tmp24OvoS/target/debug/deps/libpkg_config-57a3e9bd3f4f7d73.rlib", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `curl-sys`
error: could not compile `libssh2-sys`
error: could not compile `libz-sys`
error: could not compile `libgit2-sys`
error: could not compile `libgit2-sys`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "url", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/url-2.2.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=d12fc40f8e40722d", "-C", "extra-filename=-d12fc40f8e40722d", "--out-dir", "/tmp/.tmp24OvoS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "form_urlencoded=/tmp/.tmp24OvoS/target/debug/deps/libform_urlencoded-1a48ab4b6760ee48.rmeta", "--extern", "idna=/tmp/.tmp24OvoS/target/debug/deps/libidna-6a34d38410bd9527.rmeta", "--extern", "matches=/tmp/.tmp24OvoS/target/debug/deps/libmatches-a104dd362fa85146.rmeta", "--extern", "percent_encoding=/tmp/.tmp24OvoS/target/debug/deps/libpercent_encoding-b196e1663297fef1.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `libnghttp2-sys`
error: could not compile `openssl-sys`
error: could not compile `url`
error: could not compile `libz-sys`
error: could not compile `libz-sys`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "env_logger", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/env_logger-0.9.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"atty\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"humantime\"", "--cfg", "feature=\"regex\"", "--cfg", "feature=\"termcolor\"", "-C", "metadata=f287f47a0ad49782", "-C", "extra-filename=-f287f47a0ad49782", "--out-dir", "/tmp/.tmp24OvoS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "atty=/tmp/.tmp24OvoS/target/debug/deps/libatty-8e27291e809e89e5.rmeta", "--extern", "humantime=/tmp/.tmp24OvoS/target/debug/deps/libhumantime-69fede984985242f.rmeta", "--extern", "log=/tmp/.tmp24OvoS/target/debug/deps/liblog-41607b4997ff699e.rmeta", "--extern", "regex=/tmp/.tmp24OvoS/target/debug/deps/libregex-dd9363dd410b6a25.rmeta", "--extern", "termcolor=/tmp/.tmp24OvoS/target/debug/deps/libtermcolor-6a61078d9610ba74.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `env_logger`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "strip_ansi_escapes", "/cargo/registry/src/github.com-1ecc6299db9ec823/strip-ansi-escapes-0.1.1/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=248c11128790595c", "-C", "extra-filename=-248c11128790595c", "--out-dir", "/tmp/.tmp24OvoS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "vte=/tmp/.tmp24OvoS/target/debug/deps/libvte-528493d8a4f8c6fe.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `strip-ansi-escapes`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "ignore", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=9987dcd45169aa16", "-C", "extra-filename=-9987dcd45169aa16", "--out-dir", "/tmp/.tmp24OvoS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp24OvoS/target/debug/deps", "--extern", "crossbeam_utils=/tmp/.tmp24OvoS/target/debug/deps/libcrossbeam_utils-9e6760e1329100d6.rmeta", "--extern", "globset=/tmp/.tmp24OvoS/target/debug/deps/libglobset-4df63928bbaaef4e.rmeta", "--extern", "lazy_static=/tmp/.tmp24OvoS/target/debug/deps/liblazy_static-b33cd6b8ee13db74.rmeta", "--extern", "log=/tmp/.tmp24OvoS/target/debug/deps/liblog-41607b4997ff699e.rmeta", "--extern", "memchr=/tmp/.tmp24OvoS/target/debug/deps/libmemchr-e6bbee201eb478c9.rmeta", "--extern", "regex=/tmp/.tmp24OvoS/target/debug/deps/libregex-dd9363dd410b6a25.rmeta", "--extern", "same_file=/tmp/.tmp24OvoS/target/debug/deps/libsame_file-d3ddf74b396c2e8b.rmeta", "--extern", "thread_local=/tmp/.tmp24OvoS/target/debug/deps/libthread_local-391fa987f0e18243.rmeta", "--extern", "walkdir=/tmp/.tmp24OvoS/target/debug/deps/libwalkdir-85c3caa7668e091f.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `ignore`

 stdout=

6 normal benchmarks remaining
6 normal benchmarks remaining
Preparing clap-3.1.6
[2022-04-30T12:45:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-30T12:45:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-30T12:45:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCuENlR#clap:3.1.6" "--" "--skip-this-rustc"
[2022-04-30T12:45:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEXMTbJ#clap:3.1.6" "--release" "--" "--skip-this-rustc"
[2022-04-30T12:45:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:45:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-30T12:45:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIfcqQ2#clap:3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
Running clap-3.1.6: Opt + [Full]
[2022-04-30T12:45:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:45:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-30T12:45:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJGUSmg#clap:3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing hyper-0.14.18
[2022-04-30T12:45:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-30T12:45:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-30T12:45:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWiwTyM#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-04-30T12:45:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWiwTyM#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-04-30T12:45:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwByLHS#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
Running hyper-0.14.18: Debug + [Full]
[2022-04-30T12:46:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:46:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-30T12:46:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFjsjYP#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-30T12:46:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:46:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-30T12:46:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpr07Tno#hyper:0.14.18" "--release" "--features" "client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
4 normal benchmarks remaining
---
[2022-04-30T12:46:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplmd3sF#regex:1.5.5" "--release" "--" "--skip-this-rustc"
Running regex-1.5.5: Debug + [Full]
[2022-04-30T12:46:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:46:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-30T12:46:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGqBhJt#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-30T12:46:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:46:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-30T12:46:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkvV1iZ#regex:1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
3 normal benchmarks remaining
---
[2022-04-30T12:46:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7pP4hH#ripgrep:13.0.0" "--release" "--" "--skip-this-rustc"
Running ripgrep-13.0.0: Debug + [Full]
[2022-04-30T12:47:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:47:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-30T12:47:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpblVFHy#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-30T12:47:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:47:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-30T12:47:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMVl1zD#ripgrep:13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
2 normal benchmarks remaining
---
[2022-04-30T12:47:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfoIOef#serde:1.0.136" "--release" "--" "--skip-this-rustc"
Running serde-1.0.136: Debug + [Full]
[2022-04-30T12:47:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:47:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-04-30T12:47:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOezGMM#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2022-04-30T12:47:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:47:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-30T12:47:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-30T12:47:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgWcIxd#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing syn-1.0.89
[2022-04-30T12:47:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-04-30T12:47:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-04-30T12:47:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiPLd9y#syn:1.0.89" "--release" "--" "--skip-this-rustc"
---
[2022-04-30T12:48:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxLGQ5c#syn:1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
[2022-04-30T12:48:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-30T12:48:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-04-30T12:48:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAyzEFb#syn:1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
collector error: 1 benchmarks failed
