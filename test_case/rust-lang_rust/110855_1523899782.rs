plain
Executing benchmark cargo-0.60.0 (1/8)
Preparing cargo-0.60.0
[2023-04-26T17:51:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T17:51:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T17:51:20Z DEBUG collector::execute] cd "/tmp/.tmpR2sQe2" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR2sQe2#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-04-26T17:51:20Z DEBUG collector::execute] cd "/tmp/.tmpgVy4bZ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgVy4bZ#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-04-26T17:52:06Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:52:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:52:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:52:06Z DEBUG collector::execute] cd "/tmp/.tmpTFnTkn" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTFnTkn#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T17:52:28Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:52:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:52:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:52:28Z DEBUG collector::execute] cd "/tmp/.tmpScydSR" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpScydSR#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/8)
Preparing clap-3.1.6
[2023-04-26T17:53:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T17:53:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-04-26T17:53:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T17:53:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T17:53:38Z DEBUG collector::execute] cd "/tmp/.tmpBWUhON" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBWUhON#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-04-26T17:53:38Z DEBUG collector::execute] cd "/tmp/.tmpsKNZ1t" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsKNZ1t#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-04-26T17:53:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:53:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:53:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:53:59Z DEBUG collector::execute] cd "/tmp/.tmpnqRcUt" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnqRcUt#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T17:54:02Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:54:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:54:02Z DEBUG collector::execute] cd "/tmp/.tmptKG5v7" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptKG5v7#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/8)
Finished benchmark hyper-0.14.18 (3/8)
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-04-26T17:54:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T17:54:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T17:54:09Z DEBUG collector::execute] cd "/tmp/.tmpcflY2U" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcflY2U#regex@1.5.5" "--" "--skip-this-rustc"
[2023-04-26T17:54:09Z DEBUG collector::execute] cd "/tmp/.tmpWWscZj" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWWscZj#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-04-26T17:54:15Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:54:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:54:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:54:15Z DEBUG collector::execute] cd "/tmp/.tmpESuGLd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpESuGLd#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T17:54:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:54:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:54:17Z DEBUG collector::execute] cd "/tmp/.tmp0Nb11M" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0Nb11M#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/8)
---
[2023-04-26T17:54:48Z DEBUG collector::execute] cd "/tmp/.tmpRg72hX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRg72hX#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
[2023-04-26T17:54:51Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:54:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:54:51Z DEBUG collector::execute] cd "/tmp/.tmpF6Ttql" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF6Ttql#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0-tiny (6/8)
Preparing ripgrep-13.0.0-tiny
[2023-04-26T17:55:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T17:55:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T17:55:01Z DEBUG collector::execute] cd "/tmp/.tmpsbJnYS" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsbJnYS#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-04-26T17:55:12Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:55:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:55:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:55:12Z DEBUG collector::execute] cd "/tmp/.tmpBcvjs7" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBcvjs7#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (7/8)
Preparing serde-1.0.136
[2023-04-26T17:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T17:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T17:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T17:55:48Z DEBUG collector::execute] cd "/tmp/.tmpgVgHDW" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgVgHDW#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-04-26T17:55:48Z DEBUG collector::execute] cd "/tmp/.tmpqZOs1T" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqZOs1T#serde@1.0.136" "--" "--skip-this-rustc"
[2023-04-26T17:55:48Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:55:48Z DEBUG collector::execute] cd "/tmp/.tmplKwKEq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplKwKEq#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T17:55:51Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:55:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:55:51Z DEBUG collector::execute] cd "/tmp/.tmpFm7pEZ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFm7pEZ#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark serde-1.0.136 (7/8)
Finished benchmark serde-1.0.136 (7/8)
Executing benchmark syn-1.0.89 (8/8)
Preparing syn-1.0.89
[2023-04-26T17:55:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T17:55:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T17:55:53Z DEBUG collector::execute] cd "/tmp/.tmptrx7K0" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptrx7K0#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-04-26T17:55:53Z DEBUG collector::execute] cd "/tmp/.tmpeFiSFf" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeFiSFf#syn@1.0.89" "--" "--skip-this-rustc"
[2023-04-26T17:55:57Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:55:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T17:55:57Z DEBUG collector::execute] cd "/tmp/.tmpQiq3FQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQiq3FQ#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
Running syn-1.0.89: Opt + [Full]
[2023-04-26T17:55:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T17:55:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T17:55:59Z DEBUG collector::execute] cd "/tmp/.tmpSEyktC" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSEyktC#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging LLVM PGO profiles to /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata
stage-build INFO: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata /tmp/tmp-multistage/opt-artifacts/llvm-pgo`
stage-build INFO: LLVM PGO statistics
---
[2023-04-26T18:08:09Z DEBUG collector::execute] cd "/tmp/.tmpEXab9g" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEXab9g#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
Running bitmaps-3.1.0: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-04-26T18:08:09Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:08:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:08:09Z DEBUG collector::execute] cd "/tmp/.tmpDeiWTu" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDeiWTu#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:08:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:08:10Z DEBUG collector::execute] cd "/tmp/.tmpDeiWTu" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDeiWTu#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDeiWTu/incremental-state"
[2023-04-26T18:08:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:08:12Z DEBUG collector::execute] cd "/tmp/.tmpDeiWTu" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDeiWTu#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDeiWTu/incremental-state"
[2023-04-26T18:08:12Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpDeiWTu"
[2023-04-26T18:08:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:08:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:08:12Z DEBUG collector::execute] cd "/tmp/.tmpDeiWTu" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDeiWTu#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDeiWTu/incremental-state"
[2023-04-26T18:08:13Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:08:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:08:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:08:13Z DEBUG collector::execute] cd "/tmp/.tmpcgMogv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcgMogv#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:08:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-26T18:08:14Z DEBUG collector::execute] cd "/tmp/.tmpcgMogv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcgMogv#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcgMogv/incremental-state"
[2023-04-26T18:08:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:08:16Z DEBUG collector::execute] cd "/tmp/.tmpcgMogv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcgMogv#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcgMogv/incremental-state"
[2023-04-26T18:08:16Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpcgMogv"
[2023-04-26T18:08:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:08:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:08:16Z DEBUG collector::execute] cd "/tmp/.tmpcgMogv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcgMogv#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcgMogv/incremental-state"
[2023-04-26T18:08:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:08:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:08:17Z DEBUG collector::execute] cd "/tmp/.tmpirE1vP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpirE1vP#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:08:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing cargo-0.60.0
[2023-04-26T18:08:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:08:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:08:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:08:21Z DEBUG collector::execute] cd "/tmp/.tmpDj9daW" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDj9daW#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-04-26T18:08:21Z DEBUG collector::execute] cd "/tmp/.tmpwtKZ3R" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwtKZ3R#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-04-26T18:08:21Z DEBUG collector::execute] cd "/tmp/.tmpTGgZ5F" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTGgZ5F#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-04-26T18:09:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:09:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:09:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:09:17Z DEBUG collector::execute] cd "/tmp/.tmpyEpQFL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyEpQFL#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:09:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:09:29Z DEBUG collector::execute] cd "/tmp/.tmpyEpQFL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyEpQFL#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyEpQFL/incremental-state"
[2023-04-26T18:09:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:09:43Z DEBUG collector::execute] cd "/tmp/.tmpyEpQFL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyEpQFL#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyEpQFL/incremental-state"
[2023-04-26T18:09:46Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpyEpQFL"
[2023-04-26T18:09:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:09:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:09:46Z DEBUG collector::execute] cd "/tmp/.tmpyEpQFL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyEpQFL#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyEpQFL/incremental-state"
[2023-04-26T18:09:49Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:09:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:09:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:09:49Z DEBUG collector::execute] cd "/tmp/.tmpOtmhqN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOtmhqN#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:10:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-26T18:10:19Z DEBUG collector::execute] cd "/tmp/.tmpOtmhqN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOtmhqN#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOtmhqN/incremental-state"
[2023-04-26T18:10:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:10:54Z DEBUG collector::execute] cd "/tmp/.tmpOtmhqN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOtmhqN#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOtmhqN/incremental-state"
[2023-04-26T18:10:59Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpOtmhqN"
[2023-04-26T18:10:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:10:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:10:59Z DEBUG collector::execute] cd "/tmp/.tmpOtmhqN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOtmhqN#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOtmhqN/incremental-state"
[2023-04-26T18:11:06Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:11:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:11:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:11:06Z DEBUG collector::execute] cd "/tmp/.tmpcwDTrQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcwDTrQ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:11:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-26T18:11:48Z DEBUG collector::execute] cd "/tmp/.tmpcwDTrQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcwDTrQ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcwDTrQ/incremental-state"
[2023-04-26T18:12:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:12:29Z DEBUG collector::execute] cd "/tmp/.tmpcwDTrQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcwDTrQ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcwDTrQ/incremental-state"
[2023-04-26T18:12:34Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpcwDTrQ"
[2023-04-26T18:12:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:12:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-26T18:12:34Z DEBUG collector::execute] cd "/tmp/.tmpcwDTrQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcwDTrQ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcwDTrQ/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2023-04-26T18:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:12:45Z DEBUG collector::execute] cd "/tmp/.tmp6ZFWrL" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6ZFWrL#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:12:45Z DEBUG collector::execute] cd "/tmp/.tmpjsE28y" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjsE28y#ctfe-stress-5@0.1.0" "--" "--skip-this-rustc"
[2023-04-26T18:12:45Z DEBUG collector::execute] cd "/tmp/.tmpjeRvKJ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjeRvKJ#ctfe-stress-5@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:12:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:12:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:12:45Z DEBUG collector::execute] cd "/tmp/.tmplJuN9s" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplJuN9s#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:12:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:12:51Z DEBUG collector::execute] cd "/tmp/.tmplJuN9s" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplJuN9s#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplJuN9s/incremental-state"
[2023-04-26T18:12:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:12:58Z DEBUG collector::execute] cd "/tmp/.tmplJuN9s" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplJuN9s#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplJuN9s/incremental-state"
[2023-04-26T18:12:58Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:12:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:12:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:12:58Z DEBUG collector::execute] cd "/tmp/.tmpcyLSTs" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyLSTs#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:13:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-26T18:13:05Z DEBUG collector::execute] cd "/tmp/.tmpcyLSTs" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyLSTs#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcyLSTs/incremental-state"
[2023-04-26T18:13:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:13:12Z DEBUG collector::execute] cd "/tmp/.tmpcyLSTs" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcyLSTs#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcyLSTs/incremental-state"
[2023-04-26T18:13:12Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:13:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:13:12Z DEBUG collector::execute] cd "/tmp/.tmpE2urP8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE2urP8#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:13:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing diesel-1.4.8
[2023-04-26T18:13:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:13:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:13:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:13:25Z DEBUG collector::execute] cd "/tmp/.tmpvOZnYI" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvOZnYI#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:13:25Z DEBUG collector::execute] cd "/tmp/.tmpfirIMp" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfirIMp#diesel@1.4.8" "--" "--skip-this-rustc"
[2023-04-26T18:13:25Z DEBUG collector::execute] cd "/tmp/.tmpPl4B7e" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPl4B7e#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:13:36Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:13:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:13:36Z DEBUG collector::execute] cd "/tmp/.tmpujx424" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpujx424#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:13:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
Preparing externs
[2023-04-26T18:14:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:14:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:14:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:14:56Z DEBUG collector::execute] cd "/tmp/.tmpSOSq84" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSOSq84#externs@0.1.0" "--" "--skip-this-rustc"
[2023-04-26T18:14:56Z DEBUG collector::execute] cd "/tmp/.tmpF1FpS6" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF1FpS6#externs@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:14:56Z DEBUG collector::execute] cd "/tmp/.tmpfNolJu" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfNolJu#externs@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:14:56Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:14:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:14:56Z DEBUG collector::execute] cd "/tmp/.tmpwttK85" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwttK85#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:14:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:14:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:14:57Z DEBUG collector::execute] cd "/tmp/.tmpwttK85" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwttK85#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwttK85/incremental-state"
[2023-04-26T18:14:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:14:57Z DEBUG collector::execute] cd "/tmp/.tmpwttK85" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwttK85#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwttK85/incremental-state"
Running externs: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-04-26T18:14:57Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:14:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:14:57Z DEBUG collector::execute] cd "/tmp/.tmpmpZchM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmpZchM#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:14:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-26T18:14:58Z DEBUG collector::execute] cd "/tmp/.tmpmpZchM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmpZchM#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmpZchM/incremental-state"
[2023-04-26T18:14:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:14:58Z DEBUG collector::execute] cd "/tmp/.tmpmpZchM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmpZchM#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmpZchM/incremental-state"
[2023-04-26T18:14:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:14:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:14:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:14:59Z DEBUG collector::execute] cd "/tmp/.tmpOljSEY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOljSEY#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:14:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-26T18:14:59Z DEBUG collector::execute] cd "/tmp/.tmpOljSEY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOljSEY#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOljSEY/incremental-state"
[2023-04-26T18:14:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:14:59Z DEBUG collector::execute] cd "/tmp/.tmpOljSEY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOljSEY#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOljSEY/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-04-26T18:15:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:15:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:15:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:15:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:15:00Z DEBUG collector::execute] cd "/tmp/.tmpvdu1LK" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvdu1LK#match-stress@0.1.0" "--" "--skip-this-rustc"
[2023-04-26T18:15:00Z DEBUG collector::execute] cd "/tmp/.tmp8Nc0o8" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8Nc0o8#match-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:15:00Z DEBUG collector::execute] cd "/tmp/.tmpDk34ln" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDk34ln#match-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:15:00Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:15:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:15:00Z DEBUG collector::execute] cd "/tmp/.tmpMGus8S" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMGus8S#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:01Z DEBUG collector::execute] cd "/tmp/.tmpMGus8S" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMGus8S#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMGus8S/incremental-state"
[2023-04-26T18:15:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:03Z DEBUG collector::execute] cd "/tmp/.tmpMGus8S" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMGus8S#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMGus8S/incremental-state"
[2023-04-26T18:15:04Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:15:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:15:04Z DEBUG collector::execute] cd "/tmp/.tmpIXELIb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIXELIb#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:06Z DEBUG collector::execute] cd "/tmp/.tmpIXELIb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIXELIb#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIXELIb/incremental-state"
[2023-04-26T18:15:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:07Z DEBUG collector::execute] cd "/tmp/.tmpIXELIb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIXELIb#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIXELIb/incremental-state"
[2023-04-26T18:15:08Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:15:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:15:08Z DEBUG collector::execute] cd "/tmp/.tmpYAIjo2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYAIjo2#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:10Z DEBUG collector::execute] cd "/tmp/.tmpYAIjo2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYAIjo2#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYAIjo2/incremental-state"
[2023-04-26T18:15:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:12Z DEBUG collector::execute] cd "/tmp/.tmpYAIjo2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYAIjo2#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYAIjo2/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-04-26T18:15:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:15:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:15:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:15:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:15:12Z DEBUG collector::execute] cd "/tmp/.tmpZYHvJv" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZYHvJv#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-04-26T18:15:12Z DEBUG collector::execute] cd "/tmp/.tmpLx53Er" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLx53Er#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-04-26T18:15:12Z DEBUG collector::execute] cd "/tmp/.tmpzDHjQE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzDHjQE#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-04-26T18:15:13Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:15:13Z DEBUG collector::execute] cd "/tmp/.tmpiVG7Hs" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiVG7Hs#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:13Z DEBUG collector::execute] cd "/tmp/.tmpiVG7Hs" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiVG7Hs#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiVG7Hs/incremental-state"
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:13Z DEBUG collector::execute] cd "/tmp/.tmpiVG7Hs" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiVG7Hs#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiVG7Hs/incremental-state"
Running token-stream-stress: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-04-26T18:15:13Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:15:13Z DEBUG collector::execute] cd "/tmp/.tmpdEQeG8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdEQeG8#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:13Z DEBUG collector::execute] cd "/tmp/.tmpdEQeG8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdEQeG8#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdEQeG8/incremental-state"
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:13Z DEBUG collector::execute] cd "/tmp/.tmpdEQeG8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdEQeG8#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdEQeG8/incremental-state"
[2023-04-26T18:15:13Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:15:13Z DEBUG collector::execute] cd "/tmp/.tmpslkR8U" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpslkR8U#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:13Z DEBUG collector::execute] cd "/tmp/.tmpslkR8U" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpslkR8U#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpslkR8U/incremental-state"
[2023-04-26T18:15:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:14Z DEBUG collector::execute] cd "/tmp/.tmpslkR8U" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpslkR8U#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpslkR8U/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-04-26T18:15:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:15:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:15:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:15:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:15:14Z DEBUG collector::execute] cd "/tmp/.tmpFDAsR1" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFDAsR1#tuple-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:15:14Z DEBUG collector::execute] cd "/tmp/.tmph2z5bb" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph2z5bb#tuple-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:15:14Z DEBUG collector::execute] cd "/tmp/.tmpHurRfF" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHurRfF#tuple-stress@0.1.0" "--" "--skip-this-rustc"
[2023-04-26T18:15:14Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:15:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:15:14Z DEBUG collector::execute] cd "/tmp/.tmpm5XeEK" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm5XeEK#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:16Z DEBUG collector::execute] cd "/tmp/.tmpm5XeEK" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm5XeEK#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpm5XeEK/incremental-state"
[2023-04-26T18:15:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:19Z DEBUG collector::execute] cd "/tmp/.tmpm5XeEK" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm5XeEK#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpm5XeEK/incremental-state"
[2023-04-26T18:15:20Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmpm5XeEK"
[2023-04-26T18:15:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-04-26T18:15:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-04-26T18:15:20Z DEBUG collector::execute] cd "/tmp/.tmpm5XeEK" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm5XeEK#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpm5XeEK/incremental-state"
[2023-04-26T18:15:23Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:15:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:15:23Z DEBUG collector::execute] cd "/tmp/.tmpDTZPIY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDTZPIY#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:26Z DEBUG collector::execute] cd "/tmp/.tmpDTZPIY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDTZPIY#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDTZPIY/incremental-state"
[2023-04-26T18:15:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:29Z DEBUG collector::execute] cd "/tmp/.tmpDTZPIY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDTZPIY#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDTZPIY/incremental-state"
[2023-04-26T18:15:30Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmpDTZPIY"
[2023-04-26T18:15:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-04-26T18:15:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-04-26T18:15:30Z DEBUG collector::execute] cd "/tmp/.tmpDTZPIY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDTZPIY#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDTZPIY/incremental-state"
[2023-04-26T18:15:33Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:15:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:15:33Z DEBUG collector::execute] cd "/tmp/.tmpgU8UuA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgU8UuA#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:15:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-26T18:15:36Z DEBUG collector::execute] cd "/tmp/.tmpgU8UuA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgU8UuA#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgU8UuA/incremental-state"
[2023-04-26T18:15:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-26T18:15:39Z DEBUG collector::execute] cd "/tmp/.tmpgU8UuA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgU8UuA#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgU8UuA/incremental-state"
[2023-04-26T18:15:40Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmpgU8UuA"
[2023-04-26T18:15:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-04-26T18:15:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-04-26T18:15:40Z DEBUG collector::execute] cd "/tmp/.tmpgU8UuA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgU8UuA#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgU8UuA/incremental-state"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging Rustc PGO profiles to /tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata
stage-build INFO: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata /tmp/tmp-multistage/opt-artifacts/rustc-pgo`
stage-build INFO: Rustc PGO statistics
---
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRRAINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBO_5LocalNtNtBQ_2ty2TyEj0_ECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRRSINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBO_5LocalNtNtBQ_2ty2TyEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core9panicking13assert_failedRSINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBT_5LocalNtNtBV_2ty2TyERABO_j0_ECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvXs8_NtCs7sPQbBbHYIa_9hashbrown3setINtB6_7HashSetNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEINtNtNtNtB21_4iter6traits7collect6ExtendBO_E6extendINtNtNtB3t_8adapters6copied6CopiedINtNtNtB21_5slice4iter4IterBO_EEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvMs2_NtCs7sPQbBbHYIa_9hashbrown3setINtB5_7HashSetNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE6insertCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXs3_NtNtCs9EKI8NVlKdd_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsdVmcqkyzDD0_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRbNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRjNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXsu_NtCsi0zOMhS3zxm_12tracing_core5fieldINtB5_10DebugValueRNtNtCs2R6dSfLPeTF_5alloc6string6StringENtB5_5Value6recordCsdVmcqkyzDD0_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXsu_NtCsi0zOMhS3zxm_12tracing_core5fieldINtB5_10DebugValueRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumENtB5_5Value6recordCsdVmcqkyzDD0_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtB4_6option6OptionjEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRINtNtB4_6option6OptionjEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1m_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10TerminatorEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12UnwindActionEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body5PlaceEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRjECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placebECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placejECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeoECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10TerminatorNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12UnwindActionNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body5PlaceNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXso_NtCs9EKI8NVlKdd_4core3fmtSNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXso_NtCs9EKI8NVlKdd_4core3fmtSNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXso_NtCs9EKI8NVlKdd_4core3fmtSNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXso_NtCs9EKI8NVlKdd_4core3fmtSNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _ZN41_$LT$bool$u20$as$u20$core..fmt..Debug$GT$3fmt17h4cf438ec5a21c742E Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _ZN4core3fmt3num51_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u128$GT$3fmt17he828950c89d17b67E Hash = 1124680650125156080 up to 0 count discarded


warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb48ba2a616100910E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h944d9078e9b0a619E Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB4_4BodyNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs1_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_10BasicBlockNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs3_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_10TerminatorNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 414921811636321587 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs5_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_12UnwindActionNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 798733567640361085 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs7_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_9StatementNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs9_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_7OperandNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 536873290278838000 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsb_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_5PlaceNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsd_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_12SwitchTargetNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNtCs2R6dSfLPeTF_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsdVmcqkyzDD0_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handlehNtNtB9_5alloc6GlobalECsdVmcqkyzDD0_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs0_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVechE14grow_amortizedCsdVmcqkyzDD0_10rustc_smir Hash = 515675264369571770 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdE16reserve_for_pushCsdVmcqkyzDD0_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVechE16reserve_for_pushCsdVmcqkyzDD0_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBQ_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBQ_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBU_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBU_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVechENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 134732430909126014 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCsgvisn6nMsau_10rustc_span6symbol6SymbolEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtCscKrswBlIHJz_9rustc_abi8FieldIdxECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtCscKrswBlIHJz_9rustc_abi10VariantIdxECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCs32gLMq7cqRS_12rustc_middle2ty2TyECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCs32gLMq7cqRS_12rustc_middle3mir5LocalECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRbECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeyECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXs6_NtNtCs9EKI8NVlKdd_4core5array8equalityINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBM_5LocalNtNtBO_2ty2TyEINtB5_11SpecArrayEqBH_Kj0_E7spec_eqCsdVmcqkyzDD0_10rustc_smir Hash = 116406623519416496 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXs7_NtCs9EKI8NVlKdd_4core5arrayRAINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBD_5LocalNtNtBF_2ty2TyEj0_INtNtB7_7convert7TryFromRSBy_E8try_fromCsdVmcqkyzDD0_10rustc_smir Hash = 382993475055910911 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 974670608791895679 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRRAINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBC_5LocalNtNtBE_2ty2TyEj0_NtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRRSINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBC_5LocalNtNtBE_2ty2TyENtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h512d04776cfe48e8E Hash = 1124680650125156080 up to 0 count discarded


warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtNtCs5z09tgwLlbR_3std11collections4hash3map7HashMapNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdTINtNtNtCs32gLMq7cqRS_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtB4_4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_3fmt5ErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvXs2_NtNtNtCs9EKI8NVlKdd_4core3ops8function5implsQNCNCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir15all_local_items00INtB7_6FnOnceTRNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdEE9call_onceBW_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvXs2_NtNtNtCs9EKI8NVlKdd_4core3ops8function5implsQNCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir30rustc_terminator_to_terminator0INtB7_6FnOnceTToNtNtCs32gLMq7cqRS_12rustc_middle3mir10BasicBlockEEE9call_onceBU_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h387115571e623805E Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17hb9c1cad79af27c6dE Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir11local_crate Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir15external_crates Hash = 903233817071895769 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir10find_crate Hash = 345737795416134701 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir15all_local_items Hash = 903233817071895769 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir8entry_fn Hash = 315285099563786095 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir10smir_crate Hash = 512089718684738315 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir8mir_body Hash = 369518218308101349 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir28rustc_statement_to_statement Hash = 1117982120138886448 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir14rustc_op_to_op Hash = 650973719845048549 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir20rustc_place_to_place Hash = 1063705161840035144 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir30rustc_terminator_to_terminator Hash = 744640997393660924 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvMs5_NtCs7sPQbBbHYIa_9hashbrown3rawINtB6_8RawTableTNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE0ECsdVmcqkyzDD0_10rustc_smir Hash = 764294133655933514 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvMs5_NtCs7sPQbBbHYIa_9hashbrown3rawINtB6_8RawTableTNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE0ECsdVmcqkyzDD0_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvMs5_NtCs7sPQbBbHYIa_9hashbrown3rawINtB6_8RawTableTNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE0ECsdVmcqkyzDD0_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs7sPQbBbHYIa_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCs2R6dSfLPeTF_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECsdVmcqkyzDD0_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs7sPQbBbHYIa_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCs2R6dSfLPeTF_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECsdVmcqkyzDD0_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEBO_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 421769325739375430 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEBO_ Hash = 1063705162566655217 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumENCNCNvNtBY_10rustc_smir15external_crates00EE9from_iterBY_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtCsepcZhrxtL3J_8indexmap3set4IterNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdENCNCNvNtBY_10rustc_smir15all_local_items00EE9from_iterBY_ Hash = 303685790119406384 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtNtB2w_5slice4iter4IterNtNtCs32gLMq7cqRS_12rustc_middle3mir14BasicBlockDataENCNCNvNtB12_10rustc_smir8mir_body00EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapNtNtNtCs32gLMq7cqRS_12rustc_middle3mir10terminator17SwitchTargetsIterNCNvNtB12_10rustc_smir30rustc_terminator_to_terminator0EE9from_iterB12_ Hash = 141929913395094594 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtNtB2s_5slice4iter4IterNtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax7OperandENCNvNtB12_10rustc_smir30rustc_terminator_to_terminators_0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtNtB2u_5slice4iter4IterNtNtCs32gLMq7cqRS_12rustc_middle3mir9StatementENvNtB12_10rustc_smir28rustc_statement_to_statementEE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXs2_NtNtCs2R6dSfLPeTF_5alloc3vec11spec_extendINtB7_3VechEINtB5_10SpecExtendRhINtNtNtCs9EKI8NVlKdd_4core5slice4iter4IterhEE11spec_extendCsdVmcqkyzDD0_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockENtB5_5Debug3fmtB1a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBN_ Hash = 391331303854663571 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBN_ Hash = 93478049762468441 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VechENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsp_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsp_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandENtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsp_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementENtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RINvNtNtNtCs7sPQbBbHYIa_9hashbrown3raw5alloc5inner8do_allocNtNtCs2R6dSfLPeTF_5alloc5alloc6GlobalECsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvMs9_NtNtCs5z09tgwLlbR_3std4sync6rwlockINtB5_15RwLockReadGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEE3newCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvMsa_NtNtCs5z09tgwLlbR_3std4sync6rwlockINtB5_16RwLockWriteGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEE3newCsdVmcqkyzDD0_10rustc_smir Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvXs3_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter4IterINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtB1r_5LocalNtNtB1t_2ty2TyEEBW_EINtB5_7ZipImplBW_BW_E3newCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvXsi_NtNtCs5z09tgwLlbR_3std4sync6rwlockINtB5_15RwLockReadGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvXsj_NtNtCs5z09tgwLlbR_3std4sync6rwlockINtB5_16RwLockWriteGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 798733566711978324 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeQNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvXs8_NtCsgvisn6nMsau_10rustc_span6def_idNtB6_5DefIdNtNtCs9EKI8NVlKdd_4core4hash4Hash4hashNtCsioFC5BwGwOD_10rustc_hash8FxHasherECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvXs_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateNCNCNvNtB3x_10rustc_smir10find_crate00E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 629592967340505700 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvYINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core3fmtQNtNtCs2R6dSfLPeTF_5alloc6string6StringNtB2_5Write10write_charCsdVmcqkyzDD0_10rustc_smir Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core3fmtQNtNtCs2R6dSfLPeTF_5alloc6string6StringNtB2_5Write9write_fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core3fmtQNtNtCs2R6dSfLPeTF_5alloc6string6StringNtB2_5Write9write_strCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core6borrowNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdINtB2_6BorrowBu_E6borrowCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCs2R6dSfLPeTF_5alloc6string6StringNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvYNtNtCs2R6dSfLPeTF_5alloc6string6StringNtNtCs9EKI8NVlKdd_4core3fmt5Write9write_fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.4: no profile data available for function _RINvCsX89JnzQpfT_8smallvec10infallibleuECsdVmcqkyzDD0_10rustc_smir Hash = 1063705160175073211 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.4: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtCsX89JnzQpfT_8smallvec18CollectionAllocErrECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.4: no profile data available for function _RNvMsc_CsX89JnzQpfT_8smallvecINtB5_8SmallVecANtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexj8_E11try_reserveCsdVmcqkyzDD0_10rustc_smir Hash = 643870569470711575 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRbECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRjECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placejECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvXs_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCs7sPQbBbHYIa_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECsdVmcqkyzDD0_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvMs2_NtCsepcZhrxtL3J_8indexmap3setINtB5_8IndexSetNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE4iterCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXs1_NtNtNtCs9EKI8NVlKdd_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateNCNCNvNtB2E_10rustc_smir10find_crate00E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 505312708296135958 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtCscKrswBlIHJz_9rustc_abi10VariantIdxNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCs32gLMq7cqRS_12rustc_middle2ty2TyNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCs32gLMq7cqRS_12rustc_middle3mir5LocalNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCsgvisn6nMsau_10rustc_span6symbol6SymbolNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXs_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsb_NtCsepcZhrxtL3J_8indexmap3setINtB5_4IterNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdENtNtNtNtCs9EKI8NVlKdd_4core4iter6traits8iterator8Iterator4nextCsdVmcqkyzDD0_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsb_NtCsepcZhrxtL3J_8indexmap3setINtB5_4IterNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdENtNtNtNtCs9EKI8NVlKdd_4core4iter6traits8iterator8Iterator9size_hintCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvYINtNtNtCs9EKI8NVlKdd_4core5slice4iter4IterINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBN_5LocalNtNtBP_2ty2TyEENtNtNtNtB9_4iter8adapters3zip27TrustedRandomAccessNoCoerce4sizeCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb48ba2a616100910E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h944d9078e9b0a619E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvMNtCsdVmcqkyzDD0_10rustc_smir10stable_mirNtB2_9CrateItem4body Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir8entry_fn Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir10find_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir15all_local_items Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXs4_NtCsdVmcqkyzDD0_10rustc_smir10stable_mirNtB5_5CrateNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsa_NtCsdVmcqkyzDD0_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs32gLMq7cqRS_12rustc_middle3mir14BasicBlockDataNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockuNCNCNvNtB2k_10rustc_smir8mir_body00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2c_NCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB4X_3VecB2c_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3i_EE0E0E0EB2k_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs32gLMq7cqRS_12rustc_middle3mir9StatementNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementuNvNtB2e_10rustc_smir28rustc_statement_to_statementNCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB54_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3a_EE0E0E0EB2e_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateuNCNCNvNtB2a_10rustc_smir15external_crates00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB4G_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax7OperandNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperanduNCNvNtB2l_10rustc_smir30rustc_terminator_to_terminators_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2d_NCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB5g_3VecB2d_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3f_EE0E0E0EB2l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 421769325739375430 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCs7sPQbBbHYIa_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs32gLMq7cqRS_12rustc_middle3mir14BasicBlockDataENCNCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir8mir_body00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3a_8for_each4callNtNtNtNtB2m_10stable_mir3mir4body10BasicBlockNCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB55_3VecB4d_E14extend_trustedBN_E0E0EB2m_ Hash = 650973720849546769 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs32gLMq7cqRS_12rustc_middle3mir9StatementENvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir28rustc_statement_to_statementENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3j_8for_each4callNtNtNtNtB2c_10stable_mir3mir4body9StatementNCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB5c_3VecB4m_E14extend_trustedBN_E0E0EB2c_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumENCNCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir15external_crates00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3c_8for_each4callNtNtB2g_10stable_mir5CrateNCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB4O_3VecB4f_E14extend_trustedBN_E0E0EB2g_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax7OperandENCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir30rustc_terminator_to_terminators_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3x_8for_each4callNtNtNtNtB2l_10stable_mir3mir4body7OperandNCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB5o_3VecB4A_E14extend_trustedBN_E0E0EB2l_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.8: no profile data available for function _RINvMsz_NtCs7sPQbBbHYIa_9hashbrown3mapINtB6_15RawEntryBuilderNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdTINtNtNtCs32gLMq7cqRS_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE6searchNCINvB6_10equivalentBX_BX_E0ECsdVmcqkyzDD0_10rustc_smir Hash = 269365611345651381 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.8: no profile data available for function _RINvXs1x_NtCs7sPQbBbHYIa_9hashbrown3mapINtB7_7HashMapNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.8: no profile data available for function _RNvMs1_NtCs7sPQbBbHYIa_9hashbrown3mapINtB5_7HashMapNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE6insertCsdVmcqkyzDD0_10rustc_smir Hash = 11922956408974369 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtB19_5LocalNtNtB1b_2ty2TyEINtNtNtBa_5slice4iter4IterB14_EECsdVmcqkyzDD0_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBM_5LocalNtNtBO_2ty2TyEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCs32gLMq7cqRS_12rustc_middle9dep_graph8dep_node7DepKindEEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCsgvisn6nMsau_10rustc_span6symbol6SymbolECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRjECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtNtCs5z09tgwLlbR_3std4sync6poison10map_resultNtB2_5GuardINtNtB4_6rwlock16RwLockWriteGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEENCNvMsa_B10_BX_3new0ECsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtNtCs5z09tgwLlbR_3std4sync6poison10map_resultuINtNtB4_6rwlock15RwLockReadGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEENCNvMs9_BQ_BN_3new0ECsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvXNtCs32gLMq7cqRS_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECsdVmcqkyzDD0_10rustc_smir Hash = 816997251081395164 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvXsv_NtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCs9EKI8NVlKdd_4core4hash4Hash4hashNtCsioFC5BwGwOD_10rustc_hash8FxHasherECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RNvMNtNtNtCs9EKI8NVlKdd_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCsdVmcqkyzDD0_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core6borrowNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_6BorrowBu_E6borrowCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RNvXsM_NtCs9EKI8NVlKdd_4core6optionINtB5_6OptionNtNtCsgvisn6nMsau_10rustc_span6symbol6SymbolENtNtB7_3fmt5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RNvXsM_NtCs9EKI8NVlKdd_4core6optionINtB5_6OptionjENtNtB7_3fmt5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

---
Preparing cargo-0.60.0
[2023-04-26T18:44:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:44:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:44:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:44:55Z DEBUG collector::execute] cd "/tmp/.tmp5auzq4" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5auzq4#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-04-26T18:44:55Z DEBUG collector::execute] cd "/tmp/.tmpJzSwd1" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJzSwd1#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-04-26T18:44:55Z DEBUG collector::execute] cd "/tmp/.tmpUAaJ4x" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUAaJ4x#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-04-26T18:46:02Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:46:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:46:03Z DEBUG collector::execute] cd "/tmp/.tmp4w0Uzj" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4w0Uzj#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Debug + [Full]
---
Preparing clap-3.1.6
[2023-04-26T18:47:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:47:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:47:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:47:01Z DEBUG collector::execute] cd "/tmp/.tmpZU4XUt" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZU4XUt#clap@3.1.6" "--" "--skip-this-rustc"
[2023-04-26T18:47:01Z DEBUG collector::execute] cd "/tmp/.tmpCSZVAS" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCSZVAS#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:47:01Z DEBUG collector::execute] cd "/tmp/.tmpqSCNQc" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqSCNQc#clap@3.1.6" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:47:07Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:47:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:47:07Z DEBUG collector::execute] cd "/tmp/.tmp5V2tDu" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5V2tDu#clap@3.1.6" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Debug + [Full]
Running clap-3.1.6: Debug + [Full]
[2023-04-26T18:47:08Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:47:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:47:08Z DEBUG collector::execute] cd "/tmp/.tmpv1c8xW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpv1c8xW#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
[2023-04-26T18:47:11Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:47:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:47:11Z DEBUG collector::execute] cd "/tmp/.tmptEcrxi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptEcrxi#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-04-26T18:47:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:47:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:47:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:47:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:47:17Z DEBUG collector::execute] cd "/tmp/.tmpPmzQRB" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPmzQRB#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-04-26T18:47:17Z DEBUG collector::execute] cd "/tmp/.tmpf9xU2a" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpf9xU2a#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-04-26T18:47:17Z DEBUG collector::execute] cd "/tmp/.tmpusC5bh" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpusC5bh#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-04-26T18:47:43Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:47:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:47:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:47:43Z DEBUG collector::execute] cd "/tmp/.tmptpu3Qi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptpu3Qi#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:47:44Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:47:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:47:45Z DEBUG collector::execute] cd "/tmp/.tmpmM3ojG" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmM3ojG#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
Running hyper-0.14.18: Opt + [Full]
[2023-04-26T18:47:47Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:47:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:47:48Z DEBUG collector::execute] cd "/tmp/.tmpxC1enM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxC1enM#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-04-26T18:47:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:47:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:47:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:47:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:47:55Z DEBUG collector::execute] cd "/tmp/.tmpN7RGDH" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN7RGDH#regex@1.5.5" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:47:55Z DEBUG collector::execute] cd "/tmp/.tmpHVxIP1" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHVxIP1#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:47:55Z DEBUG collector::execute] cd "/tmp/.tmp7y7tZU" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7y7tZU#regex@1.5.5" "--" "--skip-this-rustc"
[2023-04-26T18:48:03Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:48:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:48:03Z DEBUG collector::execute] cd "/tmp/.tmpJr0dfO" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJr0dfO#regex@1.5.5" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Debug + [Full]
Running regex-1.5.5: Debug + [Full]
[2023-04-26T18:48:03Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:48:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:48:03Z DEBUG collector::execute] cd "/tmp/.tmpicaSNB" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpicaSNB#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:48:05Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:48:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:48:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:48:05Z DEBUG collector::execute] cd "/tmp/.tmpzmUbiA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzmUbiA#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/8)
Preparing ripgrep-13.0.0
[2023-04-26T18:48:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:48:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:48:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:48:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:48:12Z DEBUG collector::execute] cd "/tmp/.tmpaAMjiW" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaAMjiW#ripgrep@13.0.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:48:12Z DEBUG collector::execute] cd "/tmp/.tmpb1Ot0z" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb1Ot0z#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-04-26T18:48:12Z DEBUG collector::execute] cd "/tmp/.tmps91PCd" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps91PCd#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:48:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:48:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:48:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:48:45Z DEBUG collector::execute] cd "/tmp/.tmpbJrWYj" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbJrWYj#ripgrep@13.0.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:48:46Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:48:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:48:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:48:46Z DEBUG collector::execute] cd "/tmp/.tmpfv9aDh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfv9aDh#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:48:49Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:48:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:48:49Z DEBUG collector::execute] cd "/tmp/.tmpcBUL9u" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcBUL9u#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark ripgrep-13.0.0 (5/8)
Finished benchmark ripgrep-13.0.0 (5/8)
Executing benchmark ripgrep-13.0.0-tiny (6/8)
Preparing ripgrep-13.0.0-tiny
[2023-04-26T18:48:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:48:58Z DEBUG collector::execute] cd "/tmp/.tmpZlkhAJ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlkhAJ#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:49:11Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:49:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:49:11Z DEBUG collector::execute] cd "/tmp/.tmpd8BoEc" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd8BoEc#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark ripgrep-13.0.0-tiny (6/8)
Finished benchmark ripgrep-13.0.0-tiny (6/8)
Executing benchmark serde-1.0.136 (7/8)
Preparing serde-1.0.136
[2023-04-26T18:49:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:49:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:49:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:49:42Z DEBUG collector::execute] cd "/tmp/.tmpoJTtcO" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoJTtcO#serde@1.0.136" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:49:42Z DEBUG collector::execute] cd "/tmp/.tmpYZrqQJ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYZrqQJ#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:49:42Z DEBUG collector::execute] cd "/tmp/.tmpC01z8W" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpC01z8W#serde@1.0.136" "--" "--skip-this-rustc"
[2023-04-26T18:49:44Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:49:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:49:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:49:44Z DEBUG collector::execute] cd "/tmp/.tmpROLPue" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpROLPue#serde@1.0.136" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:49:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:49:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:49:45Z DEBUG collector::execute] cd "/tmp/.tmpN4Lc6B" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN4Lc6B#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
Running serde-1.0.136: Opt + [Full]
[2023-04-26T18:49:48Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:49:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:49:48Z DEBUG collector::execute] cd "/tmp/.tmpFiHsGr" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFiHsGr#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (8/8)
Preparing syn-1.0.89
[2023-04-26T18:49:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-04-26T18:49:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:49:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-26T18:49:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-04-26T18:49:51Z DEBUG collector::execute] cd "/tmp/.tmpKMwOk7" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKMwOk7#syn@1.0.89" "--profile" "check" "--" "--skip-this-rustc"
[2023-04-26T18:49:51Z DEBUG collector::execute] cd "/tmp/.tmpftQxGD" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpftQxGD#syn@1.0.89" "--" "--skip-this-rustc"
[2023-04-26T18:49:51Z DEBUG collector::execute] cd "/tmp/.tmpGu2xkK" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGu2xkK#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-04-26T18:49:57Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:49:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:49:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-26T18:49:57Z DEBUG collector::execute] cd "/tmp/.tmppbIT9z" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppbIT9z#syn@1.0.89" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:49:57Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:49:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:49:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-26T18:49:57Z DEBUG collector::execute] cd "/tmp/.tmpaaMFj3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaaMFj3#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-26T18:50:00Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-26T18:50:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-26T18:50:00Z DEBUG collector::execute] cd "/tmp/.tmpM6N9yC" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpM6N9yC#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (8/8)
---
[RUSTC-TIMING] rustc_incremental test:false 8.355
[RUSTC-TIMING] rustc_monomorphize test:false 11.566
[RUSTC-TIMING] rustc_query_impl test:false 91.265
[RUSTC-TIMING] rustc_ast_lowering test:false 22.502
warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRRAINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBO_5LocalNtNtBQ_2ty2TyEj0_ECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRRSINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBO_5LocalNtNtBQ_2ty2TyEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core9panicking13assert_failedRSINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBT_5LocalNtNtBV_2ty2TyERABO_j0_ECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RINvXs8_NtCs7sPQbBbHYIa_9hashbrown3setINtB6_7HashSetNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEINtNtNtNtB21_4iter6traits7collect6ExtendBO_E6extendINtNtNtB3t_8adapters6copied6CopiedINtNtNtB21_5slice4iter4IterBO_EEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvMs2_NtCs7sPQbBbHYIa_9hashbrown3setINtB5_7HashSetNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE6insertCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXs3_NtNtCs9EKI8NVlKdd_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsdVmcqkyzDD0_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRbNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRjNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXsu_NtCsi0zOMhS3zxm_12tracing_core5fieldINtB5_10DebugValueRNtNtCs2R6dSfLPeTF_5alloc6string6StringENtB5_5Value6recordCsdVmcqkyzDD0_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.14: no profile data available for function _RNvXsu_NtCsi0zOMhS3zxm_12tracing_core5fieldINtB5_10DebugValueRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumENtB5_5Value6recordCsdVmcqkyzDD0_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtB4_6option6OptionjEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRINtNtB4_6option6OptionjEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1m_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10TerminatorEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12UnwindActionEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body5PlaceEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRjECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placebECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placejECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeoECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10TerminatorNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12UnwindActionNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body5PlaceNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXso_NtCs9EKI8NVlKdd_4core3fmtSNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXso_NtCs9EKI8NVlKdd_4core3fmtSNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXso_NtCs9EKI8NVlKdd_4core3fmtSNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXso_NtCs9EKI8NVlKdd_4core3fmtSNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementNtB5_5Debug3fmtBD_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _ZN41_$LT$bool$u20$as$u20$core..fmt..Debug$GT$3fmt17h4cf438ec5a21c742E Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _ZN4core3fmt3num51_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u128$GT$3fmt17he828950c89d17b67E Hash = 1124680650125156080 up to 0 count discarded


warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb48ba2a616100910E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h944d9078e9b0a619E Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB4_4BodyNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs1_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_10BasicBlockNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs3_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_10TerminatorNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 414921811636321587 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs5_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_12UnwindActionNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 798733567640361085 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs7_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_9StatementNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXs9_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_7OperandNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 536873290278838000 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsb_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_5PlaceNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.5: no profile data available for function _RNvXsd_NtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4bodyNtB5_12SwitchTargetNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNtCs2R6dSfLPeTF_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsdVmcqkyzDD0_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RINvNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handlehNtNtB9_5alloc6GlobalECsdVmcqkyzDD0_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs0_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVechE14grow_amortizedCsdVmcqkyzDD0_10rustc_smir Hash = 515675264369571770 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdE16reserve_for_pushCsdVmcqkyzDD0_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvMs_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB4_6RawVechE16reserve_for_pushCsdVmcqkyzDD0_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBQ_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBQ_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBU_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBU_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.0: no profile data available for function _RNvXs1_NtCs2R6dSfLPeTF_5alloc7raw_vecINtB5_6RawVechENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 134732430909126014 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCsgvisn6nMsau_10rustc_span6symbol6SymbolEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtCscKrswBlIHJz_9rustc_abi8FieldIdxECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtCscKrswBlIHJz_9rustc_abi10VariantIdxECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCs32gLMq7cqRS_12rustc_middle2ty2TyECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCs32gLMq7cqRS_12rustc_middle3mir5LocalECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRbECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeyECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXs6_NtNtCs9EKI8NVlKdd_4core5array8equalityINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBM_5LocalNtNtBO_2ty2TyEINtB5_11SpecArrayEqBH_Kj0_E7spec_eqCsdVmcqkyzDD0_10rustc_smir Hash = 116406623519416496 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXs7_NtCs9EKI8NVlKdd_4core5arrayRAINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBD_5LocalNtNtBF_2ty2TyEj0_INtNtB7_7convert7TryFromRSBy_E8try_fromCsdVmcqkyzDD0_10rustc_smir Hash = 382993475055910911 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 974670608791895679 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRRAINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBC_5LocalNtNtBE_2ty2TyEj0_NtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRRSINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBC_5LocalNtNtBE_2ty2TyENtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.a231d1908c589318-cgu.7: no profile data available for function _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h512d04776cfe48e8E Hash = 1124680650125156080 up to 0 count discarded


warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtNtCs5z09tgwLlbR_3std11collections4hash3map7HashMapNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdTINtNtNtCs32gLMq7cqRS_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtB4_4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_3fmt5ErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvXs2_NtNtNtCs9EKI8NVlKdd_4core3ops8function5implsQNCNCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir15all_local_items00INtB7_6FnOnceTRNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdEE9call_onceBW_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvXs2_NtNtNtCs9EKI8NVlKdd_4core3ops8function5implsQNCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir30rustc_terminator_to_terminator0INtB7_6FnOnceTToNtNtCs32gLMq7cqRS_12rustc_middle3mir10BasicBlockEEE9call_onceBU_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h387115571e623805E Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17hb9c1cad79af27c6dE Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir11local_crate Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir15external_crates Hash = 903233817071895769 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir10find_crate Hash = 345737795416134701 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir15all_local_items Hash = 903233817071895769 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir8entry_fn Hash = 315285099563786095 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir10smir_crate Hash = 512089718684738315 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir8mir_body Hash = 369518218308101349 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir28rustc_statement_to_statement Hash = 1117982120138886448 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir14rustc_op_to_op Hash = 650973719845048549 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir20rustc_place_to_place Hash = 1063705161840035144 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.2: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir30rustc_terminator_to_terminator Hash = 744640997393660924 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvMs5_NtCs7sPQbBbHYIa_9hashbrown3rawINtB6_8RawTableTNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE0ECsdVmcqkyzDD0_10rustc_smir Hash = 764294133655933514 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvMs5_NtCs7sPQbBbHYIa_9hashbrown3rawINtB6_8RawTableTNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE0ECsdVmcqkyzDD0_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvMs5_NtCs7sPQbBbHYIa_9hashbrown3rawINtB6_8RawTableTNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE0ECsdVmcqkyzDD0_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs7sPQbBbHYIa_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCs2R6dSfLPeTF_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECsdVmcqkyzDD0_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.3: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs7sPQbBbHYIa_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCs2R6dSfLPeTF_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECsdVmcqkyzDD0_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEBO_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 421769325739375430 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEBO_ Hash = 1063705162566655217 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumENCNCNvNtBY_10rustc_smir15external_crates00EE9from_iterBY_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtCsepcZhrxtL3J_8indexmap3set4IterNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdENCNCNvNtBY_10rustc_smir15all_local_items00EE9from_iterBY_ Hash = 303685790119406384 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtNtB2w_5slice4iter4IterNtNtCs32gLMq7cqRS_12rustc_middle3mir14BasicBlockDataENCNCNvNtB12_10rustc_smir8mir_body00EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapNtNtNtCs32gLMq7cqRS_12rustc_middle3mir10terminator17SwitchTargetsIterNCNvNtB12_10rustc_smir30rustc_terminator_to_terminator0EE9from_iterB12_ Hash = 141929913395094594 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtNtB2s_5slice4iter4IterNtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax7OperandENCNvNtB12_10rustc_smir30rustc_terminator_to_terminators_0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXNtNtCs2R6dSfLPeTF_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEINtB2_12SpecFromIterBU_INtNtNtNtCs9EKI8NVlKdd_4core4iter8adapters3map3MapINtNtNtB2u_5slice4iter4IterNtNtCs32gLMq7cqRS_12rustc_middle3mir9StatementENvNtB12_10rustc_smir28rustc_statement_to_statementEE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXs2_NtNtCs2R6dSfLPeTF_5alloc3vec11spec_extendINtB7_3VechEINtB5_10SpecExtendRhINtNtNtCs9EKI8NVlKdd_4core5slice4iter4IterhEE11spec_extendCsdVmcqkyzDD0_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockENtB5_5Debug3fmtB1a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBN_ Hash = 391331303854663571 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropBN_ Hash = 93478049762468441 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsn_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VechENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsp_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsp_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandENtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.1: no profile data available for function _RNvXsp_NtCs2R6dSfLPeTF_5alloc3vecINtB5_3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementENtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RINvNtNtNtCs7sPQbBbHYIa_9hashbrown3raw5alloc5inner8do_allocNtNtCs2R6dSfLPeTF_5alloc5alloc6GlobalECsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvMs9_NtNtCs5z09tgwLlbR_3std4sync6rwlockINtB5_15RwLockReadGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEE3newCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvMsa_NtNtCs5z09tgwLlbR_3std4sync6rwlockINtB5_16RwLockWriteGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEE3newCsdVmcqkyzDD0_10rustc_smir Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvXs3_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter4IterINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtB1r_5LocalNtNtB1t_2ty2TyEEBW_EINtB5_7ZipImplBW_BW_E3newCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvXsi_NtNtCs5z09tgwLlbR_3std4sync6rwlockINtB5_15RwLockReadGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.12: no profile data available for function _RNvXsj_NtNtCs5z09tgwLlbR_3std4sync6rwlockINtB5_16RwLockWriteGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEENtNtNtCs9EKI8NVlKdd_4core3ops4drop4Drop4dropCsdVmcqkyzDD0_10rustc_smir Hash = 798733566711978324 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeQNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvXs8_NtCsgvisn6nMsau_10rustc_span6def_idNtB6_5DefIdNtNtCs9EKI8NVlKdd_4core4hash4Hash4hashNtCsioFC5BwGwOD_10rustc_hash8FxHasherECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvXs_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateNCNCNvNtB3x_10rustc_smir10find_crate00E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 629592967340505700 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RINvYINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core3fmtQNtNtCs2R6dSfLPeTF_5alloc6string6StringNtB2_5Write10write_charCsdVmcqkyzDD0_10rustc_smir Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core3fmtQNtNtCs2R6dSfLPeTF_5alloc6string6StringNtB2_5Write9write_fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core3fmtQNtNtCs2R6dSfLPeTF_5alloc6string6StringNtB2_5Write9write_strCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core6borrowNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdINtB2_6BorrowBu_E6borrowCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCs2R6dSfLPeTF_5alloc6string6StringNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.10: no profile data available for function _RNvYNtNtCs2R6dSfLPeTF_5alloc6string6StringNtNtCs9EKI8NVlKdd_4core3fmt5Write9write_fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.4: no profile data available for function _RINvCsX89JnzQpfT_8smallvec10infallibleuECsdVmcqkyzDD0_10rustc_smir Hash = 1063705160175073211 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.4: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtCsX89JnzQpfT_8smallvec18CollectionAllocErrECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.4: no profile data available for function _RNvMsc_CsX89JnzQpfT_8smallvecINtB5_8SmallVecANtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexj8_E11try_reserveCsdVmcqkyzDD0_10rustc_smir Hash = 643870569470711575 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRbECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRjECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placejECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RINvXs_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCs7sPQbBbHYIa_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECsdVmcqkyzDD0_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvMs2_NtCsepcZhrxtL3J_8indexmap3setINtB5_8IndexSetNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE4iterCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXs1_NtNtNtCs9EKI8NVlKdd_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateNCNCNvNtB2E_10rustc_smir10find_crate00E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 505312708296135958 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtCscKrswBlIHJz_9rustc_abi10VariantIdxNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCs32gLMq7cqRS_12rustc_middle2ty2TyNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCs32gLMq7cqRS_12rustc_middle3mir5LocalNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsV_NtCs9EKI8NVlKdd_4core3fmtRNtNtCsgvisn6nMsau_10rustc_span6symbol6SymbolNtB5_5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXs_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsb_NtCsepcZhrxtL3J_8indexmap3setINtB5_4IterNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdENtNtNtNtCs9EKI8NVlKdd_4core4iter6traits8iterator8Iterator4nextCsdVmcqkyzDD0_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsb_NtCsepcZhrxtL3J_8indexmap3setINtB5_4IterNtNtCsgvisn6nMsau_10rustc_span6def_id10LocalDefIdENtNtNtNtCs9EKI8NVlKdd_4core4iter6traits8iterator8Iterator9size_hintCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvYINtNtNtCs9EKI8NVlKdd_4core5slice4iter4IterINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBN_5LocalNtNtBP_2ty2TyEENtNtNtNtB9_4iter8adapters3zip27TrustedRandomAccessNoCoerce4sizeCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb48ba2a616100910E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h944d9078e9b0a619E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvMNtCsdVmcqkyzDD0_10rustc_smir10stable_mirNtB2_9CrateItem4body Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir8entry_fn Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir10find_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvNtCsdVmcqkyzDD0_10rustc_smir10stable_mir15all_local_items Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXs4_NtCsdVmcqkyzDD0_10rustc_smir10stable_mirNtB5_5CrateNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.15: no profile data available for function _RNvXsa_NtCsdVmcqkyzDD0_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCs9EKI8NVlKdd_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVecNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtCs2R6dSfLPeTF_5alloc7raw_vec6RawVechEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs32gLMq7cqRS_12rustc_middle3mir14BasicBlockDataNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockuNCNCNvNtB2k_10rustc_smir8mir_body00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2c_NCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB4X_3VecB2c_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3i_EE0E0E0EB2k_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs32gLMq7cqRS_12rustc_middle3mir9StatementNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementuNvNtB2e_10rustc_smir28rustc_statement_to_statementNCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB54_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3a_EE0E0E0EB2e_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir5CrateuNCNCNvNtB2a_10rustc_smir15external_crates00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB4G_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax7OperandNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperanduNCNvNtB2l_10rustc_smir30rustc_terminator_to_terminators_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2d_NCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB5g_3VecB2d_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3f_EE0E0E0EB2l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtCs2R6dSfLPeTF_5alloc6string6StringECsdVmcqkyzDD0_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 421769325739375430 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCs7sPQbBbHYIa_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs32gLMq7cqRS_12rustc_middle3mir14BasicBlockDataENCNCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir8mir_body00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3a_8for_each4callNtNtNtNtB2m_10stable_mir3mir4body10BasicBlockNCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB55_3VecB4d_E14extend_trustedBN_E0E0EB2m_ Hash = 650973720849546769 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs32gLMq7cqRS_12rustc_middle3mir9StatementENvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir28rustc_statement_to_statementENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3j_8for_each4callNtNtNtNtB2c_10stable_mir3mir4body9StatementNCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB5c_3VecB4m_E14extend_trustedBN_E0E0EB2c_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCsgvisn6nMsau_10rustc_span6def_id8CrateNumENCNCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir15external_crates00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3c_8for_each4callNtNtB2g_10stable_mir5CrateNCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB4O_3VecB4f_E14extend_trustedBN_E0E0EB2g_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.13: no profile data available for function _RINvXs0_NtNtNtCs9EKI8NVlKdd_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax7OperandENCNvNtCsdVmcqkyzDD0_10rustc_smir10rustc_smir30rustc_terminator_to_terminators_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3x_8for_each4callNtNtNtNtB2l_10stable_mir3mir4body7OperandNCINvMsi_NtCs2R6dSfLPeTF_5alloc3vecINtB5o_3VecB4A_E14extend_trustedBN_E0E0EB2l_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.8: no profile data available for function _RINvMsz_NtCs7sPQbBbHYIa_9hashbrown3mapINtB6_15RawEntryBuilderNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdTINtNtNtCs32gLMq7cqRS_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE6searchNCINvB6_10equivalentBX_BX_E0ECsdVmcqkyzDD0_10rustc_smir Hash = 269365611345651381 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.8: no profile data available for function _RINvXs1x_NtCs7sPQbBbHYIa_9hashbrown3mapINtB7_7HashMapNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.8: no profile data available for function _RNvMs1_NtCs7sPQbBbHYIa_9hashbrown3mapINtB5_7HashMapNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs9EKI8NVlKdd_4core4hash18BuildHasherDefaultNtCsioFC5BwGwOD_10rustc_hash8FxHasherEE6insertCsdVmcqkyzDD0_10rustc_smir Hash = 11922956408974369 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtB19_5LocalNtNtB1b_2ty2TyEINtNtNtBa_5slice4iter4IterB14_EECsdVmcqkyzDD0_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvMs5_NtNtCs9EKI8NVlKdd_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRINtNtNtCs32gLMq7cqRS_12rustc_middle3mir6syntax14ProjectionElemNtBM_5LocalNtNtBO_2ty2TyEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body10BasicBlockEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body12SwitchTargetEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body7OperandEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.9: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtNtNtCsdVmcqkyzDD0_10rustc_smir10stable_mir3mir4body9StatementEBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCs32gLMq7cqRS_12rustc_middle9dep_graph8dep_node7DepKindEEECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRNtNtCsgvisn6nMsau_10rustc_span6symbol6SymbolECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtCs9EKI8NVlKdd_4core3ptr13drop_in_placeRjECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtNtCs5z09tgwLlbR_3std4sync6poison10map_resultNtB2_5GuardINtNtB4_6rwlock16RwLockWriteGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEENCNvMsa_B10_BX_3new0ECsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvNtNtCs5z09tgwLlbR_3std4sync6poison10map_resultuINtNtB4_6rwlock15RwLockReadGuardINtNtCs2R6dSfLPeTF_5alloc3vec3VecNtNtCsgvisn6nMsau_10rustc_span6def_id5DefIdEENCNvMs9_BQ_BN_3new0ECsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvXNtCs32gLMq7cqRS_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECsdVmcqkyzDD0_10rustc_smir Hash = 816997251081395164 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RINvXsv_NtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCs9EKI8NVlKdd_4core4hash4Hash4hashNtCsioFC5BwGwOD_10rustc_hash8FxHasherECsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RNvMNtNtNtCs9EKI8NVlKdd_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCsdVmcqkyzDD0_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RNvXNtCs9EKI8NVlKdd_4core6borrowNtNtNtCsaOekt60KoCk_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_6BorrowBu_E6borrowCsdVmcqkyzDD0_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RNvXsM_NtCs9EKI8NVlKdd_4core6optionINtB5_6OptionNtNtCsgvisn6nMsau_10rustc_span6symbol6SymbolENtNtB7_3fmt5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.a231d1908c589318-cgu.11: no profile data available for function _RNvXsM_NtCs9EKI8NVlKdd_4core6optionINtB5_6OptionjENtNtB7_3fmt5Debug3fmtCsdVmcqkyzDD0_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

