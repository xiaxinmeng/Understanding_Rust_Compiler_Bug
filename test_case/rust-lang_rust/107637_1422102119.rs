plain
[2023-02-07T15:59:23Z DEBUG collector::execute] cd "/tmp/.tmpJw6l85" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJw6l85#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
Running cargo-0.60.0: Debug + [Full]
[2023-02-07T16:00:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:00:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:00:09Z DEBUG collector::execute] cd "/tmp/.tmpdmC7Ow" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdmC7Ow#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-07T16:00:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:00:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-07T16:00:46Z DEBUG collector::execute] cd "/tmp/.tmp5xt9nv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5xt9nv#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark cargo-0.60.0 (1/7)
Finished benchmark cargo-0.60.0 (1/7)
Executing benchmark clap-3.1.6 (2/7)
Preparing clap-3.1.6
[2023-02-07T16:02:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-02-07T16:02:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-02-07T16:02:58Z DEBUG collector::execute] cd "/tmp/.tmpyI6z3x" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyI6z3x#clap@3.1.6" "--" "--skip-this-rustc"
[2023-02-07T16:02:58Z DEBUG collector::execute] cd "/tmp/.tmpTpBPBO" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTpBPBO#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-02-07T16:03:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:03:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:03:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:03:10Z DEBUG collector::execute] cd "/tmp/.tmpbJ8jcR" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbJ8jcR#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-07T16:03:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:03:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-07T16:03:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-07T16:03:15Z DEBUG collector::execute] cd "/tmp/.tmpXaknqa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXaknqa#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/7)
Preparing hyper-0.14.18
[2023-02-07T16:03:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-02-07T16:03:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-02-07T16:03:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-02-07T16:03:27Z DEBUG collector::execute] cd "/tmp/.tmpJWAiBi" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJWAiBi#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-02-07T16:03:27Z DEBUG collector::execute] cd "/tmp/.tmp4ISO5v" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4ISO5v#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-02-07T16:03:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:03:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:03:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:03:55Z DEBUG collector::execute] cd "/tmp/.tmp1jvPHf" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1jvPHf#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-07T16:03:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:03:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-07T16:03:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-07T16:03:59Z DEBUG collector::execute] cd "/tmp/.tmpqnOyE2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqnOyE2#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2023-02-07T16:04:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-02-07T16:04:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-02-07T16:04:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-02-07T16:04:09Z DEBUG collector::execute] cd "/tmp/.tmpuPTWlj" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuPTWlj#regex@1.5.5" "--" "--skip-this-rustc"
[2023-02-07T16:04:09Z DEBUG collector::execute] cd "/tmp/.tmpJSpz86" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJSpz86#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-02-07T16:04:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:04:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:04:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:04:20Z DEBUG collector::execute] cd "/tmp/.tmpRo3uko" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRo3uko#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-07T16:04:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:04:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-07T16:04:23Z DEBUG collector::execute] cd "/tmp/.tmpIkCZZ3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIkCZZ3#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/7)
Finished benchmark regex-1.5.5 (4/7)
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2023-02-07T16:04:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-02-07T16:04:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-02-07T16:04:37Z DEBUG collector::execute] cd "/tmp/.tmpDepj2z" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDepj2z#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-02-07T16:04:37Z DEBUG collector::execute] cd "/tmp/.tmpTFmBXj" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTFmBXj#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-02-07T16:04:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:04:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:04:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:04:39Z DEBUG collector::execute] cd "/tmp/.tmppoTsVS" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppoTsVS#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark ripgrep-13.0.0 (5/7)
collector error: Failed to profile 'ripgrep-13.0.0' with Eprintln, recorded: expected success, got exit status: 101
---
   Compiling bitflags v1.2.1
   Compiling serde v1.0.126
   Compiling ryu v1.0.5
   Compiling unicode-width v0.1.8
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/memchr-2.4.0/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=61d92d470e3a1f78", "-C", "extra-filename=-61d92d470e3a1f78", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/memchr-61d92d470e3a1f78", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "cfg_if", "--edition=2018", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/cfg-if-1.0.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=90e3626e3c41eec3", "-C", "extra-filename=-90e3626e3c41eec3", "--out-dir", "/tmp/.tmppoTsVS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "lazy_static", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/lazy_static-1.4.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=b33cd6b8ee13db74", "-C", "extra-filename=-b33cd6b8ee13db74", "--out-dir", "/tmp/.tmppoTsVS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/libc-0.2.97/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=600eeee196813159", "-C", "extra-filename=-600eeee196813159", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/libc-600eeee196813159", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.14/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=1563591b614d3947", "-C", "extra-filename=-1563591b614d3947", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/log-1563591b614d3947", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
warning: build failed, waiting for other jobs to finish...
error: could not compile `log`
error: could not compile `libc`
error: could not compile `libc`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "regex_automata", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.1.10/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=e58dfb6c4c62b1b2", "-C", "extra-filename=-e58dfb6c4c62b1b2", "--out-dir", "/tmp/.tmppoTsVS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `memchr`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/proc-macro2-1.0.27/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"proc-macro\"", "-C", "metadata=405cdf8ab9fb27a9", "-C", "extra-filename=-405cdf8ab9fb27a9", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/proc-macro2-405cdf8ab9fb27a9", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `lazy_static`
error: could not compile `proc-macro2`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_xid", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/unicode-xid-0.2.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=3c8a88541efd8c80", "-C", "extra-filename=-3c8a88541efd8c80", "--out-dir", "/tmp/.tmppoTsVS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/syn-1.0.73/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"clone-impls\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"derive\"", "--cfg", "feature=\"parsing\"", "--cfg", "feature=\"printing\"", "--cfg", "feature=\"proc-macro\"", "--cfg", "feature=\"quote\"", "-C", "metadata=3510f41199af0150", "-C", "extra-filename=-3510f41199af0150", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/syn-3510f41199af0150", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `regex-automata`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "regex_syntax", "--edition=2018", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-syntax-0.6.25/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"unicode\"", "--cfg", "feature=\"unicode-age\"", "--cfg", "feature=\"unicode-bool\"", "--cfg", "feature=\"unicode-case\"", "--cfg", "feature=\"unicode-gencat\"", "--cfg", "feature=\"unicode-perl\"", "--cfg", "feature=\"unicode-script\"", "--cfg", "feature=\"unicode-segment\"", "-C", "metadata=c25dc19d7487cdf7", "-C", "extra-filename=-c25dc19d7487cdf7", "--out-dir", "/tmp/.tmppoTsVS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
error: could not compile `unicode-xid`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/encoding_rs-0.8.28/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=ab4f24ec1c0746ce", "-C", "extra-filename=-ab4f24ec1c0746ce", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/encoding_rs-ab4f24ec1c0746ce", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
error: could not compile `syn`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_derive-1.0.126/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=0d5721e7949c13db", "-C", "extra-filename=-0d5721e7949c13db", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/serde_derive-0d5721e7949c13db", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
error: could not compile `encoding_rs`
exiting -- non-wrapped rustc
error: could not compile `serde_derive`
error: could not compile `serde_derive`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.126/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"derive\"", "--cfg", "feature=\"serde_derive\"", "--cfg", "feature=\"std\"", "-C", "metadata=7f6ad9a32b479ab2", "-C", "extra-filename=-7f6ad9a32b479ab2", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/serde-7f6ad9a32b479ab2", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.2.1/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=e7f4899e62da0372", "-C", "extra-filename=-e7f4899e62da0372", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/bitflags-e7f4899e62da0372", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
error: could not compile `bitflags`
error: could not compile `bitflags`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/ryu-1.0.5/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=6271e9a04c40424f", "-C", "extra-filename=-6271e9a04c40424f", "--out-dir", "/tmp/.tmppoTsVS/target/debug/build/ryu-6271e9a04c40424f", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `ryu`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_width", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/unicode-width-0.1.8/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=6da279cd4f865ec2", "-C", "extra-filename=-6da279cd4f865ec2", "--out-dir", "/tmp/.tmppoTsVS/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmppoTsVS/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `unicode-width`

 stdout=

Executing benchmark serde-1.0.136 (6/7)
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2023-02-07T16:05:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-02-07T16:05:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-02-07T16:05:06Z DEBUG collector::execute] cd "/tmp/.tmpXOhKvF" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXOhKvF#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-02-07T16:05:06Z DEBUG collector::execute] cd "/tmp/.tmpMze3Na" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMze3Na#serde@1.0.136" "--" "--skip-this-rustc"
[2023-02-07T16:05:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:05:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:05:07Z DEBUG collector::execute] cd "/tmp/.tmprlH8MJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprlH8MJ#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
---
[2023-02-07T16:05:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-07T16:05:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-07T16:05:16Z DEBUG collector::execute] cd "/tmp/.tmp1c52V6" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1c52V6#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
collector error: Failed to profile 'syn-1.0.89' with Eprintln, recorded: expected success, got exit status: 101

stderr= Downloading crates ...
  Downloaded quote v1.0.16
   Compiling proc-macro2 v1.0.36
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.89 (/tmp/.tmp1c52V6)
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/proc-macro2-1.0.36/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"proc-macro\"", "-C", "metadata=c906eb5774d51dbe", "-C", "extra-filename=-c906eb5774d51dbe", "--out-dir", "/tmp/.tmp1c52V6/target/debug/build/proc-macro2-c906eb5774d51dbe", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp1c52V6/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `proc-macro2`
warning: build failed, waiting for other jobs to finish...
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_xid", "/cargo/registry/src/index.crates.io-6f17d22bba15001f/unicode-xid-0.2.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "-C", "metadata=3c8a88541efd8c80", "-C", "extra-filename=-3c8a88541efd8c80", "--out-dir", "/tmp/.tmp1c52V6/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp1c52V6/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `unicode-xid`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"clone-impls\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"derive\"", "--cfg", "feature=\"parsing\"", "--cfg", "feature=\"printing\"", "--cfg", "feature=\"proc-macro\"", "--cfg", "feature=\"quote\"", "-C", "metadata=374759948423cc19", "-C", "extra-filename=-374759948423cc19", "--out-dir", "/tmp/.tmp1c52V6/target/debug/build/syn-374759948423cc19", "-C", "linker=clang", "-L", "dependency=/tmp/.tmp1c52V6/target/debug/deps", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc


 stdout=

---
                                               
Total duration:                          28m 2s
-----------------------------------------------
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 733, in <module>
    raise e
  File "../src/ci/stage-build.py", line 730, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 668, in execute_build_pipeline
    gather_llvm_profiles(pipeline)
  File "../src/ci/stage-build.py", line 541, in gather_llvm_profiles
    crates=LLVM_PGO_CRATES
  File "../src/ci/stage-build.py", line 454, in run_compiler_benchmarks
    **env
  File "../src/ci/stage-build.py", line 404, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo', 'run', '-p', 'collector', '--bin', 'collector', '--', 'profile_local', 'eprintln', '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc', '--id', 'Test', '--cargo', '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo', '--profiles', 'Debug,Opt', '--scenarios', 'Full', '--include', 'syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18']' returned non-zero exit status 1.
