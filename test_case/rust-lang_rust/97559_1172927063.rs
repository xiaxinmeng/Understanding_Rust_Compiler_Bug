plain
[2022-07-02T16:48:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T16:48:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T16:48:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6UiYAb#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark cargo-0.60.0 (1/7)
collector error: Failed to profile 'cargo-0.60.0' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling pkg-config v0.3.24
   Compiling unicode-xid v0.2.2
   Compiling once_cell v1.9.0
   Compiling unicode-bidi v0.3.7
   Compiling unicode-bidi v0.3.7
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "pkg_config", "/cargo/registry/src/github.com-1ecc6299db9ec823/pkg-config-0.3.24/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=57a3e9bd3f4f7d73", "-C", "extra-filename=-57a3e9bd3f4f7d73", "--out-dir", "/tmp/.tmp6UiYAb/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp6UiYAb/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `pkg-config`
warning: build failed, waiting for other jobs to finish...
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "autocfg", "/cargo/registry/src/github.com-1ecc6299db9ec823/autocfg-1.1.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=5788d3b0c0ad14fb", "-C", "extra-filename=-5788d3b0c0ad14fb", "--out-dir", "/tmp/.tmp6UiYAb/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp6UiYAb/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `autocfg`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_xid", "/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.2.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=3c8a88541efd8c80", "-C", "extra-filename=-3c8a88541efd8c80", "--out-dir", "/tmp/.tmp6UiYAb/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp6UiYAb/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `unicode-xid`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "once_cell", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/once_cell-1.9.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"alloc\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"race\"", "--cfg", "feature=\"std\"", "-C", "metadata=e4834b062d069de1", "-C", "extra-filename=-e4834b062d069de1", "--out-dir", "/tmp/.tmp6UiYAb/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp6UiYAb/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `once_cell`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_bidi", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-bidi-0.3.7/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=d0a053bbcb729d32", "-C", "extra-filename=-d0a053bbcb729d32", "--out-dir", "/tmp/.tmp6UiYAb/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp6UiYAb/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `unicode-bidi`

 stdout=

Executing benchmark clap-3.1.6 (2/7)
---
[2022-07-02T16:48:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T16:48:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T16:48:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpklpE28#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark clap-3.1.6 (2/7)
collector error: Failed to profile 'clap-3.1.6' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling autocfg v1.0.1
   Compiling termcolor v1.1.2
   Compiling textwrap v0.15.0
   Compiling libc v0.2.107
   Compiling libc v0.2.107
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "autocfg", "/cargo/registry/src/github.com-1ecc6299db9ec823/autocfg-1.0.1/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=cea8f49e72b83951", "-C", "extra-filename=-cea8f49e72b83951", "--out-dir", "/tmp/.tmpklpE28/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpklpE28/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `autocfg`
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "textwrap", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/textwrap-0.15.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=d5080db8d5a544fe", "-C", "extra-filename=-d5080db8d5a544fe", "--out-dir", "/tmp/.tmpklpE28/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpklpE28/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "hashbrown", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.11.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"raw\"", "-C", "metadata=238efa6a2ed05171", "-C", "extra-filename=-238efa6a2ed05171", "--out-dir", "/tmp/.tmpklpE28/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpklpE28/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
error: could not compile `textwrap`
error: could not compile `hashbrown`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "termcolor", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/termcolor-1.1.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=6a61078d9610ba74", "-C", "extra-filename=-6a61078d9610ba74", "--out-dir", "/tmp/.tmpklpE28/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpklpE28/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `termcolor`
error: failed to run custom build command for `libc v0.2.107`
Caused by:
Caused by:
  process didn't exit successfully: `/tmp/.tmpklpE28/target/debug/build/libc-084cb9b1988b985d/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-changed=build.rs
  --- stderr
  --- stderr
  thread 'main' panicked at 'Failed to get rustc version', /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.107/build.rs:9:63


 stdout=

---
[2022-07-02T16:48:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T16:48:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T16:48:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjMMAJS#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/7)
collector error: Failed to profile 'hyper-0.14.18' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling unicode-xid v0.2.2
   Compiling autocfg v1.0.1
   Compiling fnv v1.0.7
   Compiling itoa v1.0.1
   Compiling futures-sink v0.3.19
   Compiling futures-sink v0.3.19
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_xid", "/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.2.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=3c8a88541efd8c80", "-C", "extra-filename=-3c8a88541efd8c80", "--out-dir", "/tmp/.tmpjMMAJS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpjMMAJS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `unicode-xid`
warning: build failed, waiting for other jobs to finish...
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "bytes", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/bytes-1.1.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=5e8424a0f7103369", "-C", "extra-filename=-5e8424a0f7103369", "--out-dir", "/tmp/.tmpjMMAJS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpjMMAJS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "autocfg", "/cargo/registry/src/github.com-1ecc6299db9ec823/autocfg-1.0.1/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=cea8f49e72b83951", "-C", "extra-filename=-cea8f49e72b83951", "--out-dir", "/tmp/.tmpjMMAJS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpjMMAJS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `bytes`
error: could not compile `autocfg`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "fnv", "/cargo/registry/src/github.com-1ecc6299db9ec823/fnv-1.0.7/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=22cd72556da4b99c", "-C", "extra-filename=-22cd72556da4b99c", "--out-dir", "/tmp/.tmpjMMAJS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpjMMAJS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `fnv`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "itoa", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/itoa-1.0.1/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=02bdd2be5ac8e9f1", "-C", "extra-filename=-02bdd2be5ac8e9f1", "--out-dir", "/tmp/.tmpjMMAJS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpjMMAJS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `itoa`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "futures_sink", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/futures-sink-0.3.19/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"alloc\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=5a29710ac8e9aa10", "-C", "extra-filename=-5a29710ac8e9aa10", "--out-dir", "/tmp/.tmpjMMAJS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpjMMAJS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `futures-sink`

 stdout=

Executing benchmark regex-1.5.5 (4/7)
---
[2022-07-02T16:48:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T16:48:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T16:48:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHSBIYS#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/7)
collector error: Failed to profile 'regex-1.5.5' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling regex-syntax v0.6.25
   Compiling memchr v2.4.0
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "regex_syntax", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/regex-syntax-0.6.25/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"unicode\"", "--cfg", "feature=\"unicode-age\"", "--cfg", "feature=\"unicode-bool\"", "--cfg", "feature=\"unicode-case\"", "--cfg", "feature=\"unicode-gencat\"", "--cfg", "feature=\"unicode-perl\"", "--cfg", "feature=\"unicode-script\"", "--cfg", "feature=\"unicode-segment\"", "-C", "metadata=c25dc19d7487cdf7", "-C", "extra-filename=-c25dc19d7487cdf7", "--out-dir", "/tmp/.tmpHSBIYS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpHSBIYS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "memchr", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=334ead3495f848fb", "-C", "extra-filename=-334ead3495f848fb", "--out-dir", "/tmp/.tmpHSBIYS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpHSBIYS/target/debug/deps", "--cap-lints", "allow", "--cfg", "memchr_runtime_simd", "--cfg", "memchr_runtime_sse2", "--cfg", "memchr_runtime_sse42", "--cfg", "memchr_runtime_avx", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
warning: build failed, waiting for other jobs to finish...
exiting -- non-wrapped rustc
error: could not compile `memchr`

---
[2022-07-02T16:48:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T16:48:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T16:48:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpluRbtM#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark ripgrep-13.0.0 (5/7)
collector error: Failed to profile 'ripgrep-13.0.0' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling regex-automata v0.1.10
   Compiling regex-syntax v0.6.25
   Compiling fnv v1.0.7
   Compiling unicode-width v0.1.8
   Compiling unicode-width v0.1.8
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "regex_automata", "/cargo/registry/src/github.com-1ecc6299db9ec823/regex-automata-0.1.10/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=e58dfb6c4c62b1b2", "-C", "extra-filename=-e58dfb6c4c62b1b2", "--out-dir", "/tmp/.tmpluRbtM/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpluRbtM/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_xid", "/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.2.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=3c8a88541efd8c80", "-C", "extra-filename=-3c8a88541efd8c80", "--out-dir", "/tmp/.tmpluRbtM/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpluRbtM/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `unicode-xid`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "regex_syntax", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/regex-syntax-0.6.25/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"unicode\"", "--cfg", "feature=\"unicode-age\"", "--cfg", "feature=\"unicode-bool\"", "--cfg", "feature=\"unicode-case\"", "--cfg", "feature=\"unicode-gencat\"", "--cfg", "feature=\"unicode-perl\"", "--cfg", "feature=\"unicode-script\"", "--cfg", "feature=\"unicode-segment\"", "-C", "metadata=c25dc19d7487cdf7", "-C", "extra-filename=-c25dc19d7487cdf7", "--out-dir", "/tmp/.tmpluRbtM/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpluRbtM/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `regex-syntax`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_width", "/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-width-0.1.8/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=6da279cd4f865ec2", "-C", "extra-filename=-6da279cd4f865ec2", "--out-dir", "/tmp/.tmpluRbtM/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpluRbtM/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `unicode-width`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "fnv", "/cargo/registry/src/github.com-1ecc6299db9ec823/fnv-1.0.7/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=22cd72556da4b99c", "-C", "extra-filename=-22cd72556da4b99c", "--out-dir", "/tmp/.tmpluRbtM/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpluRbtM/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `fnv`

 stdout=

Executing benchmark serde-1.0.136 (6/7)
---
[2022-07-02T16:48:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T16:48:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T16:48:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpK5Rnay#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark serde-1.0.136 (6/7)
collector error: Failed to profile 'serde-1.0.136' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling serde v1.0.136 (/tmp/.tmpK5Rnay)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "serde" "src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "--cfg" "feature=\"default\"" "--cfg" "feature=\"std\"" "-C" "metadata=e5b649e642031ad7" "-C" "extra-filename=-e5b649e642031ad7" "--out-dir" "/tmp/.tmpK5Rnay/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpK5Rnay/target/debug/deps" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `serde`


 stdout=
---
[2022-07-02T16:48:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-02T16:48:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-02T16:48:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnmAglM#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
collector error: Failed to profile 'syn-1.0.89' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.89 (/tmp/.tmpnmAglM)
   Compiling syn v1.0.89 (/tmp/.tmpnmAglM)
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_xid", "/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.2.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=3c8a88541efd8c80", "-C", "extra-filename=-3c8a88541efd8c80", "--out-dir", "/tmp/.tmpnmAglM/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpnmAglM/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `unicode-xid`


 stdout=

