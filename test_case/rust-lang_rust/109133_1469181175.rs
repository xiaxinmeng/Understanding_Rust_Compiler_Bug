plain
Executing benchmark cargo-0.60.0 (1/7)
Preparing cargo-0.60.0
[2023-03-15T00:37:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:37:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:37:01Z DEBUG collector::execute] cd "/tmp/.tmpoURx7v" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoURx7v#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-15T00:37:01Z DEBUG collector::execute] cd "/tmp/.tmpIhqavg" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIhqavg#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-15T00:37:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:37:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:37:46Z DEBUG collector::execute] cd "/tmp/.tmpd0TDS1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd0TDS1#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Opt + [Full]
Running cargo-0.60.0: Opt + [Full]
[2023-03-15T00:38:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:38:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:38:23Z DEBUG collector::execute] cd "/tmp/.tmperYDWY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmperYDWY#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/7)
Preparing clap-3.1.6
[2023-03-15T00:40:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:40:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:40:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:40:32Z DEBUG collector::execute] cd "/tmp/.tmp5FVMS0" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5FVMS0#clap@3.1.6" "--" "--skip-this-rustc"
[2023-03-15T00:40:32Z DEBUG collector::execute] cd "/tmp/.tmpGmEvnj" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGmEvnj#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-03-15T00:40:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:40:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:40:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:40:34Z DEBUG collector::execute] cd "/tmp/.tmponaMhQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmponaMhQ#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T00:40:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:40:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:40:39Z DEBUG collector::execute] cd "/tmp/.tmpZVn7kC" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZVn7kC#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark clap-3.1.6 (2/7)
Finished benchmark clap-3.1.6 (2/7)
Executing benchmark hyper-0.14.18 (3/7)
Preparing hyper-0.14.18
[2023-03-15T00:40:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:40:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:40:50Z DEBUG collector::execute] cd "/tmp/.tmps2DLZD" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps2DLZD#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-15T00:40:50Z DEBUG collector::execute] cd "/tmp/.tmpLVLc52" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLVLc52#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-15T00:41:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:41:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:41:17Z DEBUG collector::execute] cd "/tmp/.tmpui6O2i" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpui6O2i#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
Running hyper-0.14.18: Opt + [Full]
[2023-03-15T00:41:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:41:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:41:21Z DEBUG collector::execute] cd "/tmp/.tmpmHsdER" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmHsdER#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2023-03-15T00:41:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:41:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:41:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:41:30Z DEBUG collector::execute] cd "/tmp/.tmppK55gl" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppK55gl#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-03-15T00:41:30Z DEBUG collector::execute] cd "/tmp/.tmpgc0gGb" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgc0gGb#regex@1.5.5" "--" "--skip-this-rustc"
[2023-03-15T00:41:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:41:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:41:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:41:40Z DEBUG collector::execute] cd "/tmp/.tmpK9FjIT" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpK9FjIT#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T00:41:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:41:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:41:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:41:43Z DEBUG collector::execute] cd "/tmp/.tmpOBWDMQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOBWDMQ#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2023-03-15T00:41:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:41:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:41:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:41:56Z DEBUG collector::execute] cd "/tmp/.tmpu46g5S" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu46g5S#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-03-15T00:41:56Z DEBUG collector::execute] cd "/tmp/.tmpsHlT9Y" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsHlT9Y#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T00:42:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:42:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:42:21Z DEBUG collector::execute] cd "/tmp/.tmpPekqaB" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPekqaB#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
Running ripgrep-13.0.0: Opt + [Full]
[2023-03-15T00:42:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:42:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:42:26Z DEBUG collector::execute] cd "/tmp/.tmpU7wNEt" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU7wNEt#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2023-03-15T00:42:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:42:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:42:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:42:45Z DEBUG collector::execute] cd "/tmp/.tmptOoYOw" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptOoYOw#serde@1.0.136" "--" "--skip-this-rustc"
[2023-03-15T00:42:45Z DEBUG collector::execute] cd "/tmp/.tmp1TK2fH" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1TK2fH#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-03-15T00:42:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:42:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:42:46Z DEBUG collector::execute] cd "/tmp/.tmpkoJ2v7" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkoJ2v7#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
Running serde-1.0.136: Opt + [Full]
[2023-03-15T00:42:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:42:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:42:49Z DEBUG collector::execute] cd "/tmp/.tmpmJWFTs" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmJWFTs#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2023-03-15T00:42:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:42:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:42:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:42:53Z DEBUG collector::execute] cd "/tmp/.tmpgEqx5j" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgEqx5j#syn@1.0.89" "--" "--skip-this-rustc"
[2023-03-15T00:42:53Z DEBUG collector::execute] cd "/tmp/.tmpRWfwYJ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRWfwYJ#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-03-15T00:42:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:42:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:42:57Z DEBUG collector::execute] cd "/tmp/.tmppjPLK1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppjPLK1#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
---
Preparing bitmaps-3.1.0
[2023-03-15T00:53:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T00:53:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:53:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:53:50Z DEBUG collector::execute] cd "/tmp/.tmphtfXCG" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphtfXCG#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T00:53:50Z DEBUG collector::execute] cd "/tmp/.tmpkkSchz" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkkSchz#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T00:53:50Z DEBUG collector::execute] cd "/tmp/.tmpDJchqn" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDJchqn#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2023-03-15T00:53:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:53:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T00:53:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T00:53:51Z DEBUG collector::execute] cd "/tmp/.tmpohTfeq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpohTfeq#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T00:53:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T00:53:53Z DEBUG collector::execute] cd "/tmp/.tmpohTfeq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpohTfeq#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpohTfeq/incremental-state"
[2023-03-15T00:53:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T00:53:56Z DEBUG collector::execute] cd "/tmp/.tmpohTfeq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpohTfeq#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpohTfeq/incremental-state"
[2023-03-15T00:53:56Z DEBUG collector::execute] applying println to "/tmp/.tmpohTfeq"
[2023-03-15T00:53:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:53:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:53:56Z DEBUG collector::execute] cd "/tmp/.tmpohTfeq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpohTfeq#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpohTfeq/incremental-state"
[2023-03-15T00:53:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:53:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:53:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:53:57Z DEBUG collector::execute] cd "/tmp/.tmpTWbX1S" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTWbX1S#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T00:53:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T00:53:59Z DEBUG collector::execute] cd "/tmp/.tmpTWbX1S" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTWbX1S#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTWbX1S/incremental-state"
[2023-03-15T00:54:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T00:54:02Z DEBUG collector::execute] cd "/tmp/.tmpTWbX1S" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTWbX1S#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTWbX1S/incremental-state"
[2023-03-15T00:54:03Z DEBUG collector::execute] applying println to "/tmp/.tmpTWbX1S"
[2023-03-15T00:54:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:54:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:54:03Z DEBUG collector::execute] cd "/tmp/.tmpTWbX1S" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTWbX1S#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTWbX1S/incremental-state"
[2023-03-15T00:54:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:54:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:54:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:54:04Z DEBUG collector::execute] cd "/tmp/.tmpfxopDE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfxopDE#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T00:54:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T00:54:06Z DEBUG collector::execute] cd "/tmp/.tmpfxopDE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfxopDE#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfxopDE/incremental-state"
[2023-03-15T00:54:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T00:54:08Z DEBUG collector::execute] cd "/tmp/.tmpfxopDE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfxopDE#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfxopDE/incremental-state"
[2023-03-15T00:54:09Z DEBUG collector::execute] applying println to "/tmp/.tmpfxopDE"
[2023-03-15T00:54:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:54:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:54:09Z DEBUG collector::execute] cd "/tmp/.tmpfxopDE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfxopDE#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfxopDE/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-03-15T00:54:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T00:54:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:54:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T00:54:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T00:54:10Z DEBUG collector::execute] cd "/tmp/.tmpxk7m4X" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxk7m4X#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-15T00:54:10Z DEBUG collector::execute] cd "/tmp/.tmp1BWi7E" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1BWi7E#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-15T00:54:10Z DEBUG collector::execute] cd "/tmp/.tmpBUNz9W" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBUNz9W#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-03-15T00:55:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:55:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T00:55:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T00:55:04Z DEBUG collector::execute] cd "/tmp/.tmpadT2NS" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpadT2NS#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T00:55:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T00:55:23Z DEBUG collector::execute] cd "/tmp/.tmpadT2NS" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpadT2NS#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpadT2NS/incremental-state"
[2023-03-15T00:55:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T00:55:45Z DEBUG collector::execute] cd "/tmp/.tmpadT2NS" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpadT2NS#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpadT2NS/incremental-state"
[2023-03-15T00:55:49Z DEBUG collector::execute] applying println to "/tmp/.tmpadT2NS"
[2023-03-15T00:55:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:55:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:55:49Z DEBUG collector::execute] cd "/tmp/.tmpadT2NS" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpadT2NS#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpadT2NS/incremental-state"
[2023-03-15T00:55:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:55:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:55:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T00:55:54Z DEBUG collector::execute] cd "/tmp/.tmpwR2fxi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwR2fxi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T00:56:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T00:56:41Z DEBUG collector::execute] cd "/tmp/.tmpwR2fxi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwR2fxi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwR2fxi/incremental-state"
[2023-03-15T00:57:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T00:57:38Z DEBUG collector::execute] cd "/tmp/.tmpwR2fxi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwR2fxi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwR2fxi/incremental-state"
[2023-03-15T00:57:47Z DEBUG collector::execute] applying println to "/tmp/.tmpwR2fxi"
[2023-03-15T00:57:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:57:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T00:57:47Z DEBUG collector::execute] cd "/tmp/.tmpwR2fxi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwR2fxi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwR2fxi/incremental-state"
[2023-03-15T00:57:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T00:57:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T00:57:58Z DEBUG collector::execute] cd "/tmp/.tmp1J1ybD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1J1ybD#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T00:58:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing ctfe-stress-5
[2023-03-15T01:00:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:00:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:00:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:00:09Z DEBUG collector::execute] cd "/tmp/.tmp1cw3zX" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1cw3zX#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:00:09Z DEBUG collector::execute] cd "/tmp/.tmpwPpc8G" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwPpc8G#ctfe-stress-5@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:00:09Z DEBUG collector::execute] cd "/tmp/.tmp3KGGXb" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3KGGXb#ctfe-stress-5@0.1.0" "--" "--skip-this-rustc"
[2023-03-15T01:00:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:00:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:00:10Z DEBUG collector::execute] cd "/tmp/.tmpQ8Q54D" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ8Q54D#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:00:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:00:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:00:18Z DEBUG collector::execute] cd "/tmp/.tmpQ8Q54D" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ8Q54D#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQ8Q54D/incremental-state"
[2023-03-15T01:00:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:00:28Z DEBUG collector::execute] cd "/tmp/.tmpQ8Q54D" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ8Q54D#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQ8Q54D/incremental-state"
Running ctfe-stress-5: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-03-15T01:00:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:00:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:00:28Z DEBUG collector::execute] cd "/tmp/.tmpcvyHec" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcvyHec#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:00:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T01:00:36Z DEBUG collector::execute] cd "/tmp/.tmpcvyHec" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcvyHec#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcvyHec/incremental-state"
[2023-03-15T01:00:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:00:46Z DEBUG collector::execute] cd "/tmp/.tmpcvyHec" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcvyHec#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcvyHec/incremental-state"
[2023-03-15T01:00:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:00:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:00:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:00:47Z DEBUG collector::execute] cd "/tmp/.tmpCUrQg1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCUrQg1#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:00:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T01:00:55Z DEBUG collector::execute] cd "/tmp/.tmpCUrQg1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCUrQg1#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCUrQg1/incremental-state"
[2023-03-15T01:01:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:01:05Z DEBUG collector::execute] cd "/tmp/.tmpCUrQg1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCUrQg1#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCUrQg1/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2023-03-15T01:01:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:01:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:01:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:01:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:01:05Z DEBUG collector::execute] cd "/tmp/.tmp3UbxqB" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3UbxqB#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:01:05Z DEBUG collector::execute] cd "/tmp/.tmpfliAhL" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfliAhL#diesel@1.4.8" "--" "--skip-this-rustc"
[2023-03-15T01:01:05Z DEBUG collector::execute] cd "/tmp/.tmp87AsBr" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp87AsBr#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:01:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:01:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:01:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:01:24Z DEBUG collector::execute] cd "/tmp/.tmpjqmNWg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjqmNWg#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:01:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:01:38Z DEBUG collector::execute] cd "/tmp/.tmpjqmNWg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjqmNWg#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjqmNWg/incremental-state"
[2023-03-15T01:01:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:01:56Z DEBUG collector::execute] cd "/tmp/.tmpjqmNWg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjqmNWg#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjqmNWg/incremental-state"
[2023-03-15T01:01:59Z DEBUG collector::execute] applying println to "/tmp/.tmpjqmNWg"
[2023-03-15T01:01:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T01:01:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T01:01:59Z DEBUG collector::execute] cd "/tmp/.tmpjqmNWg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjqmNWg#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjqmNWg/incremental-state"
[2023-03-15T01:02:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:02:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:02:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:02:02Z DEBUG collector::execute] cd "/tmp/.tmpyaa3qF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyaa3qF#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:02:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T01:02:20Z DEBUG collector::execute] cd "/tmp/.tmpyaa3qF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyaa3qF#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyaa3qF/incremental-state"
[2023-03-15T01:02:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:02:40Z DEBUG collector::execute] cd "/tmp/.tmpyaa3qF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyaa3qF#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyaa3qF/incremental-state"
[2023-03-15T01:02:44Z DEBUG collector::execute] applying println to "/tmp/.tmpyaa3qF"
[2023-03-15T01:02:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T01:02:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T01:02:44Z DEBUG collector::execute] cd "/tmp/.tmpyaa3qF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyaa3qF#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyaa3qF/incremental-state"
[2023-03-15T01:02:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:02:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:02:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:02:48Z DEBUG collector::execute] cd "/tmp/.tmpRKWeuP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRKWeuP#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:03:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:06Z DEBUG collector::execute] cd "/tmp/.tmpRKWeuP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRKWeuP#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRKWeuP/incremental-state"
[2023-03-15T01:03:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:03:27Z DEBUG collector::execute] cd "/tmp/.tmpRKWeuP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRKWeuP#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRKWeuP/incremental-state"
[2023-03-15T01:03:31Z DEBUG collector::execute] applying println to "/tmp/.tmpRKWeuP"
[2023-03-15T01:03:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T01:03:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T01:03:31Z DEBUG collector::execute] cd "/tmp/.tmpRKWeuP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRKWeuP#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRKWeuP/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-03-15T01:03:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:03:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:03:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:03:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:03:34Z DEBUG collector::execute] cd "/tmp/.tmpSA5Uoc" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSA5Uoc#externs@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:03:34Z DEBUG collector::execute] cd "/tmp/.tmpdZJbVM" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdZJbVM#externs@0.1.0" "--" "--skip-this-rustc"
[2023-03-15T01:03:34Z DEBUG collector::execute] cd "/tmp/.tmp8gLEsg" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8gLEsg#externs@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:03:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:03:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:03:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:03:35Z DEBUG collector::execute] cd "/tmp/.tmpwxJaql" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxJaql#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:03:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:35Z DEBUG collector::execute] cd "/tmp/.tmpwxJaql" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxJaql#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwxJaql/incremental-state"
[2023-03-15T01:03:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:03:36Z DEBUG collector::execute] cd "/tmp/.tmpwxJaql" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwxJaql#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwxJaql/incremental-state"
[2023-03-15T01:03:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:03:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:03:37Z DEBUG collector::execute] cd "/tmp/.tmpERAxDO" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpERAxDO#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:03:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:37Z DEBUG collector::execute] cd "/tmp/.tmpERAxDO" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpERAxDO#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpERAxDO/incremental-state"
[2023-03-15T01:03:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:03:38Z DEBUG collector::execute] cd "/tmp/.tmpERAxDO" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpERAxDO#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpERAxDO/incremental-state"
[2023-03-15T01:03:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:03:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:03:39Z DEBUG collector::execute] cd "/tmp/.tmpa9mvUd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpa9mvUd#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:03:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:39Z DEBUG collector::execute] cd "/tmp/.tmpa9mvUd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpa9mvUd#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpa9mvUd/incremental-state"
[2023-03-15T01:03:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:03:40Z DEBUG collector::execute] cd "/tmp/.tmpa9mvUd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpa9mvUd#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpa9mvUd/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-03-15T01:03:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:03:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:03:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:03:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:03:41Z DEBUG collector::execute] cd "/tmp/.tmpgQJ4Es" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgQJ4Es#match-stress@0.1.0" "--" "--skip-this-rustc"
[2023-03-15T01:03:41Z DEBUG collector::execute] cd "/tmp/.tmpSunQSN" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSunQSN#match-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:03:41Z DEBUG collector::execute] cd "/tmp/.tmpGpmWkh" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGpmWkh#match-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:03:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:03:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:03:41Z DEBUG collector::execute] cd "/tmp/.tmph7BlNk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7BlNk#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:03:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:43Z DEBUG collector::execute] cd "/tmp/.tmph7BlNk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7BlNk#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmph7BlNk/incremental-state"
[2023-03-15T01:03:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:03:46Z DEBUG collector::execute] cd "/tmp/.tmph7BlNk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph7BlNk#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmph7BlNk/incremental-state"
[2023-03-15T01:03:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:03:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:03:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:03:47Z DEBUG collector::execute] cd "/tmp/.tmpfDfw0I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfDfw0I#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:03:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:49Z DEBUG collector::execute] cd "/tmp/.tmpfDfw0I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfDfw0I#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfDfw0I/incremental-state"
[2023-03-15T01:03:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:03:52Z DEBUG collector::execute] cd "/tmp/.tmpfDfw0I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfDfw0I#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfDfw0I/incremental-state"
[2023-03-15T01:03:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:03:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:03:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:03:53Z DEBUG collector::execute] cd "/tmp/.tmpCtVWOW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCtVWOW#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:03:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T01:03:56Z DEBUG collector::execute] cd "/tmp/.tmpCtVWOW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCtVWOW#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCtVWOW/incremental-state"
[2023-03-15T01:03:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:03:58Z DEBUG collector::execute] cd "/tmp/.tmpCtVWOW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCtVWOW#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCtVWOW/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-03-15T01:03:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:03:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:03:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:03:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:04:00Z DEBUG collector::execute] cd "/tmp/.tmpktrjLk" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpktrjLk#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-15T01:04:00Z DEBUG collector::execute] cd "/tmp/.tmpe9sDbf" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpe9sDbf#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-15T01:04:00Z DEBUG collector::execute] cd "/tmp/.tmpRb0ZLz" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRb0ZLz#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-15T01:04:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:04:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:04:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:04:00Z DEBUG collector::execute] cd "/tmp/.tmpyuw9im" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyuw9im#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:04:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:04:00Z DEBUG collector::execute] cd "/tmp/.tmpyuw9im" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyuw9im#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyuw9im/incremental-state"
[2023-03-15T01:04:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:04:01Z DEBUG collector::execute] cd "/tmp/.tmpyuw9im" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyuw9im#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyuw9im/incremental-state"
[2023-03-15T01:04:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:04:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:04:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:04:01Z DEBUG collector::execute] cd "/tmp/.tmpeDmjW1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeDmjW1#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:04:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T01:04:01Z DEBUG collector::execute] cd "/tmp/.tmpeDmjW1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeDmjW1#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeDmjW1/incremental-state"
[2023-03-15T01:04:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:04:01Z DEBUG collector::execute] cd "/tmp/.tmpeDmjW1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeDmjW1#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeDmjW1/incremental-state"
[2023-03-15T01:04:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:04:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:04:02Z DEBUG collector::execute] cd "/tmp/.tmp15LgHD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp15LgHD#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:04:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing tuple-stress
[2023-03-15T01:04:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:04:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:04:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:04:03Z DEBUG collector::execute] cd "/tmp/.tmpLGE8jr" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLGE8jr#tuple-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:04:03Z DEBUG collector::execute] cd "/tmp/.tmppZVOQz" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppZVOQz#tuple-stress@0.1.0" "--" "--skip-this-rustc"
[2023-03-15T01:04:03Z DEBUG collector::execute] cd "/tmp/.tmpkqs4Cm" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkqs4Cm#tuple-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:04:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:04:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:04:03Z DEBUG collector::execute] cd "/tmp/.tmpFBcxd8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFBcxd8#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:04:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:04:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T01:04:07Z DEBUG collector::execute] cd "/tmp/.tmpFBcxd8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFBcxd8#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFBcxd8/incremental-state"
[2023-03-15T01:04:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:04:12Z DEBUG collector::execute] cd "/tmp/.tmpFBcxd8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFBcxd8#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFBcxd8/incremental-state"
[2023-03-15T01:04:14Z DEBUG collector::execute] applying new row to "/tmp/.tmpFBcxd8"
[2023-03-15T01:04:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T01:04:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T01:04:14Z DEBUG collector::execute] cd "/tmp/.tmpFBcxd8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFBcxd8#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFBcxd8/incremental-state"
[2023-03-15T01:04:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:04:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:04:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:04:19Z DEBUG collector::execute] cd "/tmp/.tmpmPANSF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmPANSF#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:04:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T01:04:23Z DEBUG collector::execute] cd "/tmp/.tmpmPANSF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmPANSF#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmPANSF/incremental-state"
[2023-03-15T01:04:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T01:04:29Z DEBUG collector::execute] cd "/tmp/.tmpmPANSF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmPANSF#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmPANSF/incremental-state"
[2023-03-15T01:04:30Z DEBUG collector::execute] applying new row to "/tmp/.tmpmPANSF"
[2023-03-15T01:04:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T01:04:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T01:04:30Z DEBUG collector::execute] cd "/tmp/.tmpmPANSF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmPANSF#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpmPANSF/incremental-state"
[2023-03-15T01:04:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:04:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:04:35Z DEBUG collector::execute] cd "/tmp/.tmpwK3Xv0" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwK3Xv0#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:04:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
[RUSTC-TIMING] build_script_build test:false 2.303
[RUSTC-TIMING] rustc_ty_utils test:false 42.920
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvMs5_NtCsjMs3ndWfRYA_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE0ECsfofat0khBdq_10rustc_smir Hash = 1126541921141000735 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvMs5_NtCsjMs3ndWfRYA_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE0ECsfofat0khBdq_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvMs5_NtCsjMs3ndWfRYA_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE0ECsfofat0khBdq_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsjMs3ndWfRYA_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCsawMRqAqjvFj_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECsfofat0khBdq_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsjMs3ndWfRYA_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCsawMRqAqjvFj_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECsfofat0khBdq_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.13: no profile data available for function _RINvXs8_NtCsjMs3ndWfRYA_9hashbrown3setINtB6_7HashSetNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtNtNtB21_4iter6traits7collect6ExtendBO_E6extendINtNtNtB3t_8adapters6copied6CopiedINtNtNtB21_5slice4iter4IterBO_EEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.13: no profile data available for function _RNvMs2_NtCsjMs3ndWfRYA_9hashbrown3setINtB5_7HashSetNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE6insertCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVechEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNvNtNtNtNtCsJMicmUp0Se_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvXs2_NtNtNtCs8IbwvNkqcSG_4core3ops8function5implsQNCNCNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15all_local_items00INtB7_6FnOnceTRNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdEE9call_onceBW_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir11local_crate Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15external_crates Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir10find_crate Hash = 578412082070520035 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15all_local_items Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir10smir_crate Hash = 737863019117533973 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.9: no profile data available for function _RNvXNtNtCs8IbwvNkqcSG_4core5slice3cmpShNtNtB6_3cmp9PartialEq2eqCsfofat0khBdq_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RINvNtCsawMRqAqjvFj_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsfofat0khBdq_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RINvNvMs_NtCsawMRqAqjvFj_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RINvNvMs_NtCsawMRqAqjvFj_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvMs0_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvMs0_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvMs_NtCsawMRqAqjvFj_5alloc7raw_vecINtB4_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvMs_NtCsawMRqAqjvFj_5alloc7raw_vecINtB4_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvXs1_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateENtNtNtCs8IbwvNkqcSG_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvXs1_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemENtNtNtCs8IbwvNkqcSG_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvXs1_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVechENtNtNtCs8IbwvNkqcSG_4core3ops4drop4Drop4dropCsfofat0khBdq_10rustc_smir Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RINvNtCsdtixmnGaRGT_21rustc_data_structures4sync11assert_syncNtNtNtNtCsJMicmUp0Se_12rustc_middle2ty7context3tls12ImplicitCtxtECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RINvNtNtCs8IbwvNkqcSG_4core5alloc6layout10size_alignTNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RINvYINtNtNtCs8IbwvNkqcSG_4core5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexENtNtNtNtBa_4iter6traits8iterator8Iterator6copiedBJ_ECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvMs_NtNtNtNtNtCsbG9e8h9Em9n_3std3sys6common12thread_local10fast_local4fastINtB4_3KeyINtNtCs8IbwvNkqcSG_4core4cell4CellPuEE13register_dtorCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertINtNtNtB7_3ptr6unique6UniquehEINtB5_4IntoINtNtBD_8non_null7NonNullhEE4intoCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumINtB5_4IntojE4intoCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertjINtB5_4IntoNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumE4intoCsfofat0khBdq_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXs5_NtCs8IbwvNkqcSG_4core7convertjINtB5_7TryFromjE8try_fromCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXsV_NtCs8IbwvNkqcSG_4core3fmtRNtNtCsiozC8OM4Taz_10rustc_span6def_id5DefIdNtB5_5Debug3fmtCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXsV_NtCs8IbwvNkqcSG_4core3fmtRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvMs5_NtCs8IbwvNkqcSG_4core3anyNtB6_6TypeId2ofNtNtCsirGgnmgb2x5_12tracing_core8callsite15DefaultCallsiteECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCsJMicmUp0Se_12rustc_middle9dep_graph8dep_node7DepKindEEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvXNtCsJMicmUp0Se_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB16_5graphINtB2g_8DepGraphBG_E10read_index0ECsfofat0khBdq_10rustc_smir Hash = 269792486063750595 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvXsv_NtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCs8IbwvNkqcSG_4core4hash4Hash4hashNtCsbnfOzIHUSN5_10rustc_hash8FxHasherECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCsfofat0khBdq_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RNvNvNtNtNtNtCsJMicmUp0Se_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RNvXNtCs8IbwvNkqcSG_4core6borrowNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_6BorrowBu_E6borrowCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB5_4IntoNtNtCsdtixmnGaRGT_21rustc_data_structures9profiling17QueryInvocationIdE4intoCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVechEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeRNtNtCsiozC8OM4Taz_10rustc_span6def_id5DefIdECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeRbECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placejECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hccd60139a0a23410E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17hacca7d8c50a761b1E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10stable_mir11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10stable_mir10find_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10stable_mir15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10stable_mir15all_local_items Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvXs3_NtCsfofat0khBdq_10rustc_smir10stable_mirNtB5_5CrateNtNtCs8IbwvNkqcSG_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvXs9_NtCsfofat0khBdq_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCs8IbwvNkqcSG_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVechEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RINvXs_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateNCNCNvNtB3x_10rustc_smir10find_crate00E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 23700016549702699 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters5chainINtB2_5ChainINtNtNtB8_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumEBX_E3newCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateuNCNCNvNtB2a_10rustc_smir15external_crates00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsawMRqAqjvFj_5alloc3vecINtB4G_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RINvXs0_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsjMs3ndWfRYA_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RINvXs0_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumENCNCNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15external_crates00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3c_8for_each4callNtNtB2g_10stable_mir5CrateNCINvMsi_NtCsawMRqAqjvFj_5alloc3vecINtB4O_3VecB4f_E14extend_trustedBN_E0E0EB2g_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB2_3MapINtNtB4_6copied6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsjMs3ndWfRYA_9hashbrown3setINtB2Y_7HashSetB1F_INtNtB8_4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtNtB6_6traits7collect6ExtendB1F_E6extendBT_E0E3newCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB2_3MapINtNtCs5p2ukOp8VoK_8indexmap3set4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdENCNCNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15all_local_items00E3newB2q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB2_3MapINtNtNtB8_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumENCNCNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15external_crates00E3newB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.1: no profile data available for function _RINvCs6VvmvoquFuR_8smallvec10infallibleuECsfofat0khBdq_10rustc_smir Hash = 1063705160175073211 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.1: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtCs6VvmvoquFuR_8smallvec18CollectionAllocErrECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.1: no profile data available for function _RNvMsc_Cs6VvmvoquFuR_8smallvecINtB5_8SmallVecANtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexj8_E11try_reserveCsfofat0khBdq_10rustc_smir Hash = 832846567024380499 up to 0 count discarded
[RUSTC-TIMING] jemalloc_sys test:false 1.111
[RUSTC-TIMING] jemalloc_sys test:false 1.111
warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvMsH_NtCsirGgnmgb2x5_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs8IbwvNkqcSG_4core6option6OptionRDNtB6_5ValueEL_EEj3_ECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeRNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvNtCsirGgnmgb2x5_12tracing_core5field5debugRNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvNtCsirGgnmgb2x5_12tracing_core5field5debugRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvXs_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCsjMs3ndWfRYA_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECsfofat0khBdq_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters6copiedINtB2_6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEE3newCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RNvXs_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RNvXsu_NtCsirGgnmgb2x5_12tracing_core5fieldINtB5_10DebugValueRNtNtCsawMRqAqjvFj_5alloc6string6StringENtB5_5Value6recordCsfofat0khBdq_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RNvXsu_NtCsirGgnmgb2x5_12tracing_core5fieldINtB5_10DebugValueRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumENtB5_5Value6recordCsfofat0khBdq_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc3vec3VecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc3vec3VecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RNvXNtNtCsawMRqAqjvFj_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumENCNCNvNtBY_10rustc_smir15external_crates00EE9from_iterBY_ Hash = 1096621587988718700 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RNvXNtNtCsawMRqAqjvFj_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3map3MapINtNtCs5p2ukOp8VoK_8indexmap3set4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdENCNCNvNtBY_10rustc_smir15all_local_items00EE9from_iterBY_ Hash = 421769329463800046 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RNvXsn_NtCsawMRqAqjvFj_5alloc3vecINtB5_3VechENtNtNtCs8IbwvNkqcSG_4core3ops4drop4Drop4dropCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.15: no profile data available for function _RINvNtNtNtCsjMs3ndWfRYA_9hashbrown3raw5alloc5inner8do_allocNtNtCsawMRqAqjvFj_5alloc5alloc6GlobalECsfofat0khBdq_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.15: no profile data available for function _RNvMs2_NtCs5p2ukOp8VoK_8indexmap3setINtB5_8IndexSetNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE4iterCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.15: no profile data available for function _RNvXsb_NtCs5p2ukOp8VoK_8indexmap3setINtB5_4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdENtNtNtNtCs8IbwvNkqcSG_4core4iter6traits8iterator8Iterator4nextCsfofat0khBdq_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.15: no profile data available for function _RNvXsb_NtCs5p2ukOp8VoK_8indexmap3setINtB5_4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdENtNtNtNtCs8IbwvNkqcSG_4core4iter6traits8iterator8Iterator9size_hintCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvMs2_NtNtCsbG9e8h9Em9n_3std6thread5localINtB6_8LocalKeyINtNtCs8IbwvNkqcSG_4core4cell4CellPuEE4withNCNvNtNtNtNtCsJMicmUp0Se_12rustc_middle2ty7context3tls3tlv7get_tlv0B1s_ECsfofat0khBdq_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVechEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtNtCsbG9e8h9Em9n_3std6thread5local11AccessErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvYINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvMs0_Cs5p2ukOp8VoK_8indexmapINtB5_6BucketNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIduE7key_refCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertNtNtCsawMRqAqjvFj_5alloc11collections19TryReserveErrorKindINtB5_4IntoNtBA_15TryReserveErrorE4intoCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvXs1_NtNtNtCs8IbwvNkqcSG_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateNCNCNvNtB2E_10rustc_smir10find_crate00E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 1147234025963109369 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvXsV_NtCs8IbwvNkqcSG_4core3fmtRNtNtCsawMRqAqjvFj_5alloc6string6StringNtB5_5Debug3fmtCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvXsV_NtCs8IbwvNkqcSG_4core3fmtRbNtB5_5Debug3fmtCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir14rustc_internal11item_def_id Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir14rustc_internal9crate_num Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.5: no profile data available for function _RINvNtCsjMs3ndWfRYA_9hashbrown3map9make_hashNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexBG_INtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.5: no profile data available for function _RINvXs1x_NtCsjMs3ndWfRYA_9hashbrown3mapINtB7_7HashMapNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECsfofat0khBdq_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.5: no profile data available for function _RNvMs1_NtCsjMs3ndWfRYA_9hashbrown3mapINtB5_7HashMapNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE6insertCsfofat0khBdq_10rustc_smir Hash = 11922953328495428 up to 0 count discarded
[RUSTC-TIMING] rustc_middle test:false 139.952
[RUSTC-TIMING] rustc_smir test:false 7.639
warning: `rustc_smir` (lib) generated 111 warnings
[RUSTC-TIMING] rustc_trait_selection test:false 114.471
---
Preparing cargo-0.60.0
[2023-03-15T01:36:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:36:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:36:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:36:27Z DEBUG collector::execute] cd "/tmp/.tmpqYUljn" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqYUljn#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-15T01:36:27Z DEBUG collector::execute] cd "/tmp/.tmptTZ7p3" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptTZ7p3#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-15T01:36:27Z DEBUG collector::execute] cd "/tmp/.tmpCNSBkJ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCNSBkJ#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-03-15T01:38:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:38:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:38:21Z DEBUG collector::execute] cd "/tmp/.tmpBtYe9W" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBtYe9W#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Debug + [Full]
Running cargo-0.60.0: Debug + [Full]
[2023-03-15T01:38:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:38:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:38:29Z DEBUG collector::execute] cd "/tmp/.tmpmt6THo" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmt6THo#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:38:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:39:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:39:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:39:00Z DEBUG collector::execute] cd "/tmp/.tmpAxwq5s" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAxwq5s#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/7)
Preparing clap-3.1.6
[2023-03-15T01:40:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:40:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:40:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:40:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:40:15Z DEBUG collector::execute] cd "/tmp/.tmpnRxodL" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnRxodL#clap@3.1.6" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:40:15Z DEBUG collector::execute] cd "/tmp/.tmppJfjw4" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppJfjw4#clap@3.1.6" "--" "--skip-this-rustc"
[2023-03-15T01:40:15Z DEBUG collector::execute] cd "/tmp/.tmp9Tcqmx" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9Tcqmx#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:40:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:40:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:40:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:40:29Z DEBUG collector::execute] cd "/tmp/.tmpgOFtYl" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgOFtYl#clap@3.1.6" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:40:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:40:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:40:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:40:31Z DEBUG collector::execute] cd "/tmp/.tmpBhjVp5" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBhjVp5#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:40:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:40:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:40:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:40:37Z DEBUG collector::execute] cd "/tmp/.tmpddmjCa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpddmjCa#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/7)
Preparing hyper-0.14.18
[2023-03-15T01:40:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:40:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:40:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:40:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:40:51Z DEBUG collector::execute] cd "/tmp/.tmpmSSSky" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmSSSky#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-15T01:40:51Z DEBUG collector::execute] cd "/tmp/.tmpjKOWkI" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjKOWkI#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-15T01:40:51Z DEBUG collector::execute] cd "/tmp/.tmpaFALsz" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaFALsz#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-15T01:41:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:41:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:41:39Z DEBUG collector::execute] cd "/tmp/.tmpn450d4" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn450d4#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Debug + [Full]
Running hyper-0.14.18: Debug + [Full]
[2023-03-15T01:41:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:41:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:41:42Z DEBUG collector::execute] cd "/tmp/.tmpHbVKuc" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHbVKuc#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:41:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:41:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:41:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:41:48Z DEBUG collector::execute] cd "/tmp/.tmpzfiYRl" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzfiYRl#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2023-03-15T01:42:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:42:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:42:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:42:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:42:05Z DEBUG collector::execute] cd "/tmp/.tmpxaO8Gk" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxaO8Gk#regex@1.5.5" "--" "--skip-this-rustc"
[2023-03-15T01:42:05Z DEBUG collector::execute] cd "/tmp/.tmp2LS5B2" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2LS5B2#regex@1.5.5" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:42:05Z DEBUG collector::execute] cd "/tmp/.tmpboexez" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpboexez#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:42:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:42:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:42:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:42:22Z DEBUG collector::execute] cd "/tmp/.tmprCoaXE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprCoaXE#regex@1.5.5" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:42:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:42:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:42:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:42:23Z DEBUG collector::execute] cd "/tmp/.tmpZxesRv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZxesRv#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:42:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:42:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:42:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:42:28Z DEBUG collector::execute] cd "/tmp/.tmpySbDNO" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpySbDNO#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2023-03-15T01:42:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:42:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:42:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:42:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:42:44Z DEBUG collector::execute] cd "/tmp/.tmp2WMACZ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2WMACZ#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:42:44Z DEBUG collector::execute] cd "/tmp/.tmplNMUP6" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplNMUP6#ripgrep@13.0.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:42:44Z DEBUG collector::execute] cd "/tmp/.tmpb4uXf6" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb4uXf6#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-03-15T01:43:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:43:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:43:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:43:47Z DEBUG collector::execute] cd "/tmp/.tmpF5lntt" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF5lntt#ripgrep@13.0.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:43:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:43:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:43:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:43:48Z DEBUG collector::execute] cd "/tmp/.tmpXQisSD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXQisSD#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:43:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:43:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:43:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:43:55Z DEBUG collector::execute] cd "/tmp/.tmptD9bdT" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptD9bdT#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2023-03-15T01:44:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:44:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:44:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:44:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:44:14Z DEBUG collector::execute] cd "/tmp/.tmpYFA35J" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYFA35J#serde@1.0.136" "--" "--skip-this-rustc"
[2023-03-15T01:44:14Z DEBUG collector::execute] cd "/tmp/.tmpqFrFoH" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqFrFoH#serde@1.0.136" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:44:14Z DEBUG collector::execute] cd "/tmp/.tmpuX1FbM" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuX1FbM#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:44:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:44:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:44:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:44:19Z DEBUG collector::execute] cd "/tmp/.tmptYqa4E" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptYqa4E#serde@1.0.136" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:44:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:44:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:44:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:44:21Z DEBUG collector::execute] cd "/tmp/.tmpkOujqE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkOujqE#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:44:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:44:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:44:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:44:26Z DEBUG collector::execute] cd "/tmp/.tmppSsCuk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppSsCuk#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2023-03-15T01:44:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T01:44:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:44:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T01:44:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T01:44:35Z DEBUG collector::execute] cd "/tmp/.tmp5rkKF5" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5rkKF5#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-03-15T01:44:35Z DEBUG collector::execute] cd "/tmp/.tmpvhk9vZ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvhk9vZ#syn@1.0.89" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T01:44:35Z DEBUG collector::execute] cd "/tmp/.tmpUgmZ4f" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUgmZ4f#syn@1.0.89" "--" "--skip-this-rustc"
[2023-03-15T01:44:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:44:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:44:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T01:44:49Z DEBUG collector::execute] cd "/tmp/.tmp8NeCPA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8NeCPA#syn@1.0.89" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:44:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:44:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:44:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T01:44:50Z DEBUG collector::execute] cd "/tmp/.tmpEQkHMn" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEQkHMn#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T01:44:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T01:44:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T01:44:55Z DEBUG collector::execute] cd "/tmp/.tmp04Nz13" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp04Nz13#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
---
[RUSTC-TIMING] rustc_symbol_mangling test:false 18.330
[RUSTC-TIMING] rustc_query_impl test:false 152.125
[RUSTC-TIMING] rustc_monomorphize test:false 24.595
[RUSTC-TIMING] rustc_ast_lowering test:false 56.741
warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvMs5_NtCsjMs3ndWfRYA_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE0ECsfofat0khBdq_10rustc_smir Hash = 1126541921141000735 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvMs5_NtCsjMs3ndWfRYA_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE0ECsfofat0khBdq_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvMs5_NtCsjMs3ndWfRYA_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE0ECsfofat0khBdq_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsjMs3ndWfRYA_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCsawMRqAqjvFj_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECsfofat0khBdq_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.2: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsjMs3ndWfRYA_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCsawMRqAqjvFj_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECsfofat0khBdq_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.13: no profile data available for function _RINvXs8_NtCsjMs3ndWfRYA_9hashbrown3setINtB6_7HashSetNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtNtNtB21_4iter6traits7collect6ExtendBO_E6extendINtNtNtB3t_8adapters6copied6CopiedINtNtNtB21_5slice4iter4IterBO_EEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.13: no profile data available for function _RNvMs2_NtCsjMs3ndWfRYA_9hashbrown3setINtB5_7HashSetNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE6insertCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVechEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNvNtNtNtNtCsJMicmUp0Se_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvXs2_NtNtNtCs8IbwvNkqcSG_4core3ops8function5implsQNCNCNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15all_local_items00INtB7_6FnOnceTRNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdEE9call_onceBW_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir11local_crate Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15external_crates Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir10find_crate Hash = 578412082070520035 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15all_local_items Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.4: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10rustc_smir10smir_crate Hash = 737863019117533973 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.9: no profile data available for function _RNvXNtNtCs8IbwvNkqcSG_4core5slice3cmpShNtNtB6_3cmp9PartialEq2eqCsfofat0khBdq_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RINvNtCsawMRqAqjvFj_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsfofat0khBdq_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RINvNvMs_NtCsawMRqAqjvFj_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RINvNvMs_NtCsawMRqAqjvFj_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvMs0_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvMs0_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvMs_NtCsawMRqAqjvFj_5alloc7raw_vecINtB4_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvMs_NtCsawMRqAqjvFj_5alloc7raw_vecINtB4_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvXs1_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateENtNtNtCs8IbwvNkqcSG_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvXs1_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemENtNtNtCs8IbwvNkqcSG_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.0: no profile data available for function _RNvXs1_NtCsawMRqAqjvFj_5alloc7raw_vecINtB5_6RawVechENtNtNtCs8IbwvNkqcSG_4core3ops4drop4Drop4dropCsfofat0khBdq_10rustc_smir Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RINvNtCsdtixmnGaRGT_21rustc_data_structures4sync11assert_syncNtNtNtNtCsJMicmUp0Se_12rustc_middle2ty7context3tls12ImplicitCtxtECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RINvNtNtCs8IbwvNkqcSG_4core5alloc6layout10size_alignTNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RINvYINtNtNtCs8IbwvNkqcSG_4core5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexENtNtNtNtBa_4iter6traits8iterator8Iterator6copiedBJ_ECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvMs_NtNtNtNtNtCsbG9e8h9Em9n_3std3sys6common12thread_local10fast_local4fastINtB4_3KeyINtNtCs8IbwvNkqcSG_4core4cell4CellPuEE13register_dtorCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertINtNtNtB7_3ptr6unique6UniquehEINtB5_4IntoINtNtBD_8non_null7NonNullhEE4intoCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumINtB5_4IntojE4intoCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertjINtB5_4IntoNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumE4intoCsfofat0khBdq_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXs5_NtCs8IbwvNkqcSG_4core7convertjINtB5_7TryFromjE8try_fromCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXsV_NtCs8IbwvNkqcSG_4core3fmtRNtNtCsiozC8OM4Taz_10rustc_span6def_id5DefIdNtB5_5Debug3fmtCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.14: no profile data available for function _RNvXsV_NtCs8IbwvNkqcSG_4core3fmtRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvMs5_NtCs8IbwvNkqcSG_4core3anyNtB6_6TypeId2ofNtNtCsirGgnmgb2x5_12tracing_core8callsite15DefaultCallsiteECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCsJMicmUp0Se_12rustc_middle9dep_graph8dep_node7DepKindEEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvXNtCsJMicmUp0Se_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB16_5graphINtB2g_8DepGraphBG_E10read_index0ECsfofat0khBdq_10rustc_smir Hash = 269792486063750595 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RINvXsv_NtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCs8IbwvNkqcSG_4core4hash4Hash4hashNtCsbnfOzIHUSN5_10rustc_hash8FxHasherECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCsfofat0khBdq_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RNvNvNtNtNtNtCsJMicmUp0Se_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RNvXNtCs8IbwvNkqcSG_4core6borrowNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_6BorrowBu_E6borrowCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.12: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB5_4IntoNtNtCsdtixmnGaRGT_21rustc_data_structures9profiling17QueryInvocationIdE4intoCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVechEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeRNtNtCsiozC8OM4Taz_10rustc_span6def_id5DefIdECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeRbECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placejECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hccd60139a0a23410E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17hacca7d8c50a761b1E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10stable_mir11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10stable_mir10find_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10stable_mir15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir10stable_mir15all_local_items Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvXs3_NtCsfofat0khBdq_10rustc_smir10stable_mirNtB5_5CrateNtNtCs8IbwvNkqcSG_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.11: no profile data available for function _RNvXs9_NtCsfofat0khBdq_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCs8IbwvNkqcSG_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVechEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RINvXs_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateNCNCNvNtB3x_10rustc_smir10find_crate00E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 23700016549702699 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.6: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters5chainINtB2_5ChainINtNtNtB8_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumEBX_E3newCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateuNCNCNvNtB2a_10rustc_smir15external_crates00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsawMRqAqjvFj_5alloc3vecINtB4G_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RINvXs0_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsjMs3ndWfRYA_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RINvXs0_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumENCNCNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15external_crates00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3c_8for_each4callNtNtB2g_10stable_mir5CrateNCINvMsi_NtCsawMRqAqjvFj_5alloc3vecINtB4O_3VecB4f_E14extend_trustedBN_E0E0EB2g_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB2_3MapINtNtB4_6copied6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsjMs3ndWfRYA_9hashbrown3setINtB2Y_7HashSetB1F_INtNtB8_4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtNtB6_6traits7collect6ExtendB1F_E6extendBT_E0E3newCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB2_3MapINtNtCs5p2ukOp8VoK_8indexmap3set4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdENCNCNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15all_local_items00E3newB2q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.7: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3mapINtB2_3MapINtNtNtB8_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumENCNCNvNtCsfofat0khBdq_10rustc_smir10rustc_smir15external_crates00E3newB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.1: no profile data available for function _RINvCs6VvmvoquFuR_8smallvec10infallibleuECsfofat0khBdq_10rustc_smir Hash = 1063705160175073211 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.1: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtCs6VvmvoquFuR_8smallvec18CollectionAllocErrECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.1: no profile data available for function _RNvMsc_Cs6VvmvoquFuR_8smallvecINtB5_8SmallVecANtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexj8_E11try_reserveCsfofat0khBdq_10rustc_smir Hash = 832846567024380499 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvMsH_NtCsirGgnmgb2x5_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCs8IbwvNkqcSG_4core6option6OptionRDNtB6_5ValueEL_EEj3_ECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeRNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvNtCsirGgnmgb2x5_12tracing_core5field5debugRNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvNtCsirGgnmgb2x5_12tracing_core5field5debugRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RINvXs_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCsjMs3ndWfRYA_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECsfofat0khBdq_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RNvMNtNtNtCs8IbwvNkqcSG_4core4iter8adapters6copiedINtB2_6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEE3newCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RNvXs_NtNtNtCs8IbwvNkqcSG_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RNvXsu_NtCsirGgnmgb2x5_12tracing_core5fieldINtB5_10DebugValueRNtNtCsawMRqAqjvFj_5alloc6string6StringENtB5_5Value6recordCsfofat0khBdq_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.8: no profile data available for function _RNvXsu_NtCsirGgnmgb2x5_12tracing_core5fieldINtB5_10DebugValueRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumENtB5_5Value6recordCsfofat0khBdq_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc3vec3VecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc3vec3VecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RNvXNtNtCsawMRqAqjvFj_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumENCNCNvNtBY_10rustc_smir15external_crates00EE9from_iterBY_ Hash = 1096621587988718700 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RNvXNtNtCsawMRqAqjvFj_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsfofat0khBdq_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCs8IbwvNkqcSG_4core4iter8adapters3map3MapINtNtCs5p2ukOp8VoK_8indexmap3set4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdENCNCNvNtBY_10rustc_smir15all_local_items00EE9from_iterBY_ Hash = 421769329463800046 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.3: no profile data available for function _RNvXsn_NtCsawMRqAqjvFj_5alloc3vecINtB5_3VechENtNtNtCs8IbwvNkqcSG_4core3ops4drop4Drop4dropCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.15: no profile data available for function _RINvNtNtNtCsjMs3ndWfRYA_9hashbrown3raw5alloc5inner8do_allocNtNtCsawMRqAqjvFj_5alloc5alloc6GlobalECsfofat0khBdq_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.15: no profile data available for function _RNvMs2_NtCs5p2ukOp8VoK_8indexmap3setINtB5_8IndexSetNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE4iterCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.15: no profile data available for function _RNvXsb_NtCs5p2ukOp8VoK_8indexmap3setINtB5_4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdENtNtNtNtCs8IbwvNkqcSG_4core4iter6traits8iterator8Iterator4nextCsfofat0khBdq_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.15: no profile data available for function _RNvXsb_NtCs5p2ukOp8VoK_8indexmap3setINtB5_4IterNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIdENtNtNtNtCs8IbwvNkqcSG_4core4iter6traits8iterator8Iterator9size_hintCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvMs2_NtNtCsbG9e8h9Em9n_3std6thread5localINtB6_8LocalKeyINtNtCs8IbwvNkqcSG_4core4cell4CellPuEE4withNCNvNtNtNtNtCsJMicmUp0Se_12rustc_middle2ty7context3tls3tlv7get_tlv0B1s_ECsfofat0khBdq_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeINtNtCsawMRqAqjvFj_5alloc7raw_vec6RawVechEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtCsawMRqAqjvFj_5alloc6string6StringECsfofat0khBdq_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvNtCs8IbwvNkqcSG_4core3ptr13drop_in_placeNtNtNtCsbG9e8h9Em9n_3std6thread5local11AccessErrorECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RINvYINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvMs0_Cs5p2ukOp8VoK_8indexmapINtB5_6BucketNtNtCsiozC8OM4Taz_10rustc_span6def_id10LocalDefIduE7key_refCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvXs1_NtCs8IbwvNkqcSG_4core7convertNtNtCsawMRqAqjvFj_5alloc11collections19TryReserveErrorKindINtB5_4IntoNtBA_15TryReserveErrorE4intoCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvXs1_NtNtNtCs8IbwvNkqcSG_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCsiozC8OM4Taz_10rustc_span6def_id8CrateNumNtNtCsfofat0khBdq_10rustc_smir10stable_mir5CrateNCNCNvNtB2E_10rustc_smir10find_crate00E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 1147234025963109369 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvXsV_NtCs8IbwvNkqcSG_4core3fmtRNtNtCsawMRqAqjvFj_5alloc6string6StringNtB5_5Debug3fmtCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvXsV_NtCs8IbwvNkqcSG_4core3fmtRbNtB5_5Debug3fmtCsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir14rustc_internal11item_def_id Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.10: no profile data available for function _RNvNtCsfofat0khBdq_10rustc_smir14rustc_internal9crate_num Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.5: no profile data available for function _RINvNtCsjMs3ndWfRYA_9hashbrown3map9make_hashNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexBG_INtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEECsfofat0khBdq_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.5: no profile data available for function _RINvXs1x_NtCsjMs3ndWfRYA_9hashbrown3mapINtB7_7HashMapNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECsfofat0khBdq_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5e876fce-cgu.5: no profile data available for function _RNvMs1_NtCsjMs3ndWfRYA_9hashbrown3mapINtB5_7HashMapNtNtNtCs5lcugk0M96X_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs8IbwvNkqcSG_4core4hash18BuildHasherDefaultNtCsbnfOzIHUSN5_10rustc_hash8FxHasherEE6insertCsfofat0khBdq_10rustc_smir Hash = 11922953328495428 up to 0 count discarded
[RUSTC-TIMING] rustc_smir test:false 7.639
warning: `rustc_smir` (lib) generated 111 warnings
[RUSTC-TIMING] rustc_transmute test:false 14.631
[RUSTC-TIMING] rustc_metadata test:false 71.841
---
[TIMING] doc::RustbookSrc<bootstrap::doc::RustcBook> { target: x86_64-unknown-linux-gnu, name: "rustc", src: "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc/rustc", parent: Some(RustcBook { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, validate: false }) } -- 0.817
[TIMING] doc::RustcBook { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, validate: false } -- 17.506
Documenting stage2 cargo (x86_64-unknown-linux-gnu)
    Updating crates.io index
error: the lock file /checkout/src/tools/cargo/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
stage-build INFO: Section `Stage 4 (final build)` ended: FAIL (335.69s)
stage-build ERROR: The multi-stage build has failed
stage-build INFO: Timer results
------------------------------------------------
---
Total duration:                       1h 37m 21s
------------------------------------------------
root INFO: Free disk space: 560.66 GiB out of total 666.61 GiB (15.89% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 818, in execute_build_pipeline
    cmd(final_build_args)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['python3', '../x.py', 'dist', '--host', 'x86_64-unknown-linux-gnu', '--target', 'x86_64-unknown-linux-gnu', '--include-default-paths', 'build-manifest', 'bootstrap', '--llvm-profile-use', '/tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata', '--rust-profile-use', '/tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata', '--llvm-bolt-profile-use', '/tmp/tmp-multistage/opt-artifacts/bolt.profdata']' returned non-zero exit status 1.
