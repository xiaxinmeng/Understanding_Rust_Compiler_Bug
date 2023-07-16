plain
Executing benchmark cargo-0.60.0 (1/8)
Preparing cargo-0.60.0
[2023-03-20T15:12:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:12:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:12:01Z DEBUG collector::execute] cd "/tmp/.tmpzksvUX" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzksvUX#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-20T15:12:01Z DEBUG collector::execute] cd "/tmp/.tmpstEE2H" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpstEE2H#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-20T15:12:51Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:12:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:12:51Z DEBUG collector::execute] cd "/tmp/.tmp1fKdPO" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1fKdPO#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Opt + [Full]
---
Executing benchmark clap-3.1.6 (2/8)
Preparing clap-3.1.6
[2023-03-20T15:14:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:14:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:14:37Z DEBUG collector::execute] cd "/tmp/.tmpAE3PxQ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAE3PxQ#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:14:37Z DEBUG collector::execute] cd "/tmp/.tmpjf3fhr" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjf3fhr#clap@3.1.6" "--" "--skip-this-rustc"
[2023-03-20T15:14:40Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:14:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:14:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:14:40Z DEBUG collector::execute] cd "/tmp/.tmpxFoPHi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxFoPHi#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:14:43Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:14:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:14:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:14:43Z DEBUG collector::execute] cd "/tmp/.tmpfyzYMv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfyzYMv#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-03-20T15:14:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:14:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:14:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:14:51Z DEBUG collector::execute] cd "/tmp/.tmpH40q4v" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH40q4v#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-20T15:14:51Z DEBUG collector::execute] cd "/tmp/.tmp8NlziS" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8NlziS#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-20T15:15:14Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:15:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:15:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:15:14Z DEBUG collector::execute] cd "/tmp/.tmpXkNwW3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXkNwW3#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:15:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:15:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:15:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:15:17Z DEBUG collector::execute] cd "/tmp/.tmp3tjONk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3tjONk#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-03-20T15:15:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:15:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:15:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:15:25Z DEBUG collector::execute] cd "/tmp/.tmpYhrW2F" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYhrW2F#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:15:25Z DEBUG collector::execute] cd "/tmp/.tmpBaUhPZ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBaUhPZ#regex@1.5.5" "--" "--skip-this-rustc"
[2023-03-20T15:15:32Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:15:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:15:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:15:32Z DEBUG collector::execute] cd "/tmp/.tmpADspNX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpADspNX#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:15:34Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:15:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:15:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:15:34Z DEBUG collector::execute] cd "/tmp/.tmpaFzqJC" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaFzqJC#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/8)
Preparing ripgrep-13.0.0
[2023-03-20T15:15:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:15:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:15:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:15:43Z DEBUG collector::execute] cd "/tmp/.tmpQhn1vu" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQhn1vu#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:15:43Z DEBUG collector::execute] cd "/tmp/.tmpferOzd" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpferOzd#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-03-20T15:16:10Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:16:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:16:10Z DEBUG collector::execute] cd "/tmp/.tmp8Pcljm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8Pcljm#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
Running ripgrep-13.0.0: Opt + [Full]
[2023-03-20T15:16:14Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:16:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:16:14Z DEBUG collector::execute] cd "/tmp/.tmpihA6Ca" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpihA6Ca#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0-tiny (6/8)
Preparing ripgrep-13.0.0-tiny
[2023-03-20T15:16:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:16:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:16:26Z DEBUG collector::execute] cd "/tmp/.tmpKuvaxb" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKuvaxb#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:16:37Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:16:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:16:37Z DEBUG collector::execute] cd "/tmp/.tmpypldPI" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpypldPI#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark ripgrep-13.0.0-tiny (6/8)
---
[2023-03-20T15:17:14Z DEBUG collector::execute] cd "/tmp/.tmpD7Q63O" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpD7Q63O#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
[2023-03-20T15:17:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:17:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:17:17Z DEBUG collector::execute] cd "/tmp/.tmpUKlaRr" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUKlaRr#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (8/8)
Preparing syn-1.0.89
[2023-03-20T15:17:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:17:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:17:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:17:20Z DEBUG collector::execute] cd "/tmp/.tmpRCXblZ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRCXblZ#syn@1.0.89" "--" "--skip-this-rustc"
[2023-03-20T15:17:20Z DEBUG collector::execute] cd "/tmp/.tmpbNf6ti" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbNf6ti#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:17:23Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:17:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:17:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:17:23Z DEBUG collector::execute] cd "/tmp/.tmpQsQIid" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQsQIid#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:17:26Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:17:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:17:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:17:26Z DEBUG collector::execute] cd "/tmp/.tmp2EVjJT" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2EVjJT#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging LLVM PGO profiles to /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata
stage-build INFO: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata /tmp/tmp-multistage/opt-artifacts/llvm-pgo`
stage-build INFO: LLVM PGO statistics
---
Preparing bitmaps-3.1.0
[2023-03-20T15:29:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T15:29:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:29:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:29:07Z DEBUG collector::execute] cd "/tmp/.tmp35DqAd" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp35DqAd#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T15:29:07Z DEBUG collector::execute] cd "/tmp/.tmpqYntkt" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqYntkt#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2023-03-20T15:29:07Z DEBUG collector::execute] cd "/tmp/.tmpj0toRE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj0toRE#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:29:07Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:29:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:29:07Z DEBUG collector::execute] cd "/tmp/.tmp1uwXr8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1uwXr8#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:29:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:29:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:29:08Z DEBUG collector::execute] cd "/tmp/.tmp1uwXr8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1uwXr8#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp1uwXr8/incremental-state"
[2023-03-20T15:29:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:29:10Z DEBUG collector::execute] cd "/tmp/.tmp1uwXr8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1uwXr8#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp1uwXr8/incremental-state"
[2023-03-20T15:29:11Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmp1uwXr8"
[2023-03-20T15:29:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:29:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:29:11Z DEBUG collector::execute] cd "/tmp/.tmp1uwXr8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1uwXr8#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp1uwXr8/incremental-state"
[2023-03-20T15:29:11Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:29:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:29:12Z DEBUG collector::execute] cd "/tmp/.tmp7XN7g6" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7XN7g6#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:29:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-03-20T15:29:16Z DEBUG collector::execute] cd "/tmp/.tmp7XN7g6" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7XN7g6#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7XN7g6/incremental-state"
Running bitmaps-3.1.0: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-03-20T15:29:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:29:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:29:17Z DEBUG collector::execute] cd "/tmp/.tmpBJmU4v" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBJmU4v#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:29:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-20T15:29:18Z DEBUG collector::execute] cd "/tmp/.tmpBJmU4v" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBJmU4v#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBJmU4v/incremental-state"
[2023-03-20T15:29:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:29:20Z DEBUG collector::execute] cd "/tmp/.tmpBJmU4v" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBJmU4v#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBJmU4v/incremental-state"
[2023-03-20T15:29:21Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpBJmU4v"
[2023-03-20T15:29:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:29:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:29:21Z DEBUG collector::execute] cd "/tmp/.tmpBJmU4v" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBJmU4v#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBJmU4v/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-03-20T15:29:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T15:29:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:29:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:29:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:29:22Z DEBUG collector::execute] cd "/tmp/.tmpjbwvue" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjbwvue#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-20T15:29:22Z DEBUG collector::execute] cd "/tmp/.tmpXNS8yR" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXNS8yR#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-20T15:29:22Z DEBUG collector::execute] cd "/tmp/.tmpQtJTqj" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQtJTqj#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-03-20T15:30:25Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:30:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:30:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:30:25Z DEBUG collector::execute] cd "/tmp/.tmpQYeTnU" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQYeTnU#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:30:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:30:40Z DEBUG collector::execute] cd "/tmp/.tmpQYeTnU" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQYeTnU#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQYeTnU/incremental-state"
[2023-03-20T15:30:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:30:58Z DEBUG collector::execute] cd "/tmp/.tmpQYeTnU" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQYeTnU#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQYeTnU/incremental-state"
[2023-03-20T15:31:01Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpQYeTnU"
[2023-03-20T15:31:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:31:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:31:01Z DEBUG collector::execute] cd "/tmp/.tmpQYeTnU" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQYeTnU#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQYeTnU/incremental-state"
[2023-03-20T15:31:04Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:31:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:31:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:31:05Z DEBUG collector::execute] cd "/tmp/.tmpENEhVX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpENEhVX#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:31:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-20T15:31:42Z DEBUG collector::execute] cd "/tmp/.tmpENEhVX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpENEhVX#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpENEhVX/incremental-state"
[2023-03-20T15:32:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:32:27Z DEBUG collector::execute] cd "/tmp/.tmpENEhVX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpENEhVX#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpENEhVX/incremental-state"
[2023-03-20T15:32:34Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpENEhVX"
[2023-03-20T15:32:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:32:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:32:34Z DEBUG collector::execute] cd "/tmp/.tmpENEhVX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpENEhVX#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpENEhVX/incremental-state"
[2023-03-20T15:32:42Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:32:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:32:42Z DEBUG collector::execute] cd "/tmp/.tmpvViFpH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvViFpH#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:33:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-20T15:33:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-20T15:33:32Z DEBUG collector::execute] cd "/tmp/.tmpvViFpH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvViFpH#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvViFpH/incremental-state"
[2023-03-20T15:34:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:34:21Z DEBUG collector::execute] cd "/tmp/.tmpvViFpH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvViFpH#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvViFpH/incremental-state"
[2023-03-20T15:34:28Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpvViFpH"
[2023-03-20T15:34:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:34:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:34:28Z DEBUG collector::execute] cd "/tmp/.tmpvViFpH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvViFpH#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvViFpH/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2023-03-20T15:34:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:34:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:34:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:34:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T15:34:37Z DEBUG collector::execute] cd "/tmp/.tmpCnuA0M" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCnuA0M#ctfe-stress-5@0.1.0" "--" "--skip-this-rustc"
[2023-03-20T15:34:37Z DEBUG collector::execute] cd "/tmp/.tmpLzJld5" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLzJld5#ctfe-stress-5@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:34:37Z DEBUG collector::execute] cd "/tmp/.tmpqwHzke" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqwHzke#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T15:34:38Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:34:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:34:38Z DEBUG collector::execute] cd "/tmp/.tmp2utnop" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2utnop#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:34:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:34:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:34:45Z DEBUG collector::execute] cd "/tmp/.tmp2utnop" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2utnop#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2utnop/incremental-state"
[2023-03-20T15:34:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:34:54Z DEBUG collector::execute] cd "/tmp/.tmp2utnop" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2utnop#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2utnop/incremental-state"
[2023-03-20T15:34:54Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:34:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:34:54Z DEBUG collector::execute] cd "/tmp/.tmpHm4b4j" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHm4b4j#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:35:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
Preparing diesel-1.4.8
[2023-03-20T15:35:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T15:35:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:35:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:35:28Z DEBUG collector::execute] cd "/tmp/.tmpsaFl8S" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsaFl8S#diesel@1.4.8" "--" "--skip-this-rustc"
[2023-03-20T15:35:28Z DEBUG collector::execute] cd "/tmp/.tmp8JTnPV" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8JTnPV#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T15:35:28Z DEBUG collector::execute] cd "/tmp/.tmp23e3Or" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp23e3Or#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:35:41Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:35:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:35:41Z DEBUG collector::execute] cd "/tmp/.tmpKPUgT4" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKPUgT4#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:35:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2023-03-20T15:36:09Z DEBUG collector::execute] cd "/tmp/.tmpKPUgT4" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKPUgT4#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKPUgT4/incremental-state"
Running diesel-1.4.8: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-03-20T15:36:12Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:36:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:36:12Z DEBUG collector::execute] cd "/tmp/.tmpZlgECv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlgECv#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:36:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-20T15:36:26Z DEBUG collector::execute] cd "/tmp/.tmpZlgECv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlgECv#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZlgECv/incremental-state"
[2023-03-20T15:36:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:36:42Z DEBUG collector::execute] cd "/tmp/.tmpZlgECv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlgECv#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZlgECv/incremental-state"
[2023-03-20T15:36:45Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpZlgECv"
[2023-03-20T15:36:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:36:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:36:45Z DEBUG collector::execute] cd "/tmp/.tmpZlgECv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZlgECv#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpZlgECv/incremental-state"
[2023-03-20T15:36:48Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:36:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:36:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:36:48Z DEBUG collector::execute] cd "/tmp/.tmpkkTYtr" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkkTYtr#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:02Z DEBUG collector::execute] cd "/tmp/.tmpkkTYtr" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkkTYtr#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkkTYtr/incremental-state"
[2023-03-20T15:37:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:19Z DEBUG collector::execute] cd "/tmp/.tmpkkTYtr" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkkTYtr#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkkTYtr/incremental-state"
[2023-03-20T15:37:22Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpkkTYtr"
[2023-03-20T15:37:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:37:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-20T15:37:22Z DEBUG collector::execute] cd "/tmp/.tmpkkTYtr" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkkTYtr#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpkkTYtr/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-03-20T15:37:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T15:37:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:37:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:37:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:37:25Z DEBUG collector::execute] cd "/tmp/.tmpjXkGIv" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjXkGIv#externs@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:37:25Z DEBUG collector::execute] cd "/tmp/.tmpZaclF6" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZaclF6#externs@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T15:37:25Z DEBUG collector::execute] cd "/tmp/.tmpotDUUi" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpotDUUi#externs@0.1.0" "--" "--skip-this-rustc"
[2023-03-20T15:37:25Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:37:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:37:25Z DEBUG collector::execute] cd "/tmp/.tmpJLDZA7" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJLDZA7#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:25Z DEBUG collector::execute] cd "/tmp/.tmpJLDZA7" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJLDZA7#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJLDZA7/incremental-state"
[2023-03-20T15:37:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:26Z DEBUG collector::execute] cd "/tmp/.tmpJLDZA7" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJLDZA7#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJLDZA7/incremental-state"
[2023-03-20T15:37:26Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:37:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:37:26Z DEBUG collector::execute] cd "/tmp/.tmpQDsVKE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQDsVKE#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:27Z DEBUG collector::execute] cd "/tmp/.tmpQDsVKE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQDsVKE#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQDsVKE/incremental-state"
[2023-03-20T15:37:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:27Z DEBUG collector::execute] cd "/tmp/.tmpQDsVKE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQDsVKE#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQDsVKE/incremental-state"
[2023-03-20T15:37:28Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:37:28Z DEBUG collector::execute] cd "/tmp/.tmppD2413" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppD2413#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing match-stress
[2023-03-20T15:37:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T15:37:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:37:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:37:29Z DEBUG collector::execute] cd "/tmp/.tmpYhg4Wi" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYhg4Wi#match-stress@0.1.0" "--" "--skip-this-rustc"
[2023-03-20T15:37:29Z DEBUG collector::execute] cd "/tmp/.tmp9IeEJE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9IeEJE#match-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T15:37:29Z DEBUG collector::execute] cd "/tmp/.tmppHcpoU" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppHcpoU#match-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:37:29Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:37:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:37:29Z DEBUG collector::execute] cd "/tmp/.tmp8eILFv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8eILFv#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:31Z DEBUG collector::execute] cd "/tmp/.tmp8eILFv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8eILFv#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp8eILFv/incremental-state"
[2023-03-20T15:37:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:33Z DEBUG collector::execute] cd "/tmp/.tmp8eILFv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8eILFv#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp8eILFv/incremental-state"
[2023-03-20T15:37:34Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:37:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:37:34Z DEBUG collector::execute] cd "/tmp/.tmpVcdDGa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVcdDGa#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:36Z DEBUG collector::execute] cd "/tmp/.tmpVcdDGa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVcdDGa#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVcdDGa/incremental-state"
[2023-03-20T15:37:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:38Z DEBUG collector::execute] cd "/tmp/.tmpVcdDGa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVcdDGa#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpVcdDGa/incremental-state"
[2023-03-20T15:37:39Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:37:39Z DEBUG collector::execute] cd "/tmp/.tmpek4oh1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpek4oh1#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing token-stream-stress
[2023-03-20T15:37:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:37:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:37:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T15:37:44Z DEBUG collector::execute] cd "/tmp/.tmpUI2C9P" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUI2C9P#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-20T15:37:44Z DEBUG collector::execute] cd "/tmp/.tmpdHymkf" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdHymkf#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-20T15:37:44Z DEBUG collector::execute] cd "/tmp/.tmpsLHBvV" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsLHBvV#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-20T15:37:44Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:37:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:37:44Z DEBUG collector::execute] cd "/tmp/.tmpHgeTjz" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHgeTjz#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:44Z DEBUG collector::execute] cd "/tmp/.tmpHgeTjz" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHgeTjz#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHgeTjz/incremental-state"
[2023-03-20T15:37:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:44Z DEBUG collector::execute] cd "/tmp/.tmpHgeTjz" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHgeTjz#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHgeTjz/incremental-state"
[2023-03-20T15:37:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:37:45Z DEBUG collector::execute] cd "/tmp/.tmp2VeAJJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2VeAJJ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:45Z DEBUG collector::execute] cd "/tmp/.tmp2VeAJJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2VeAJJ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2VeAJJ/incremental-state"
[2023-03-20T15:37:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:45Z DEBUG collector::execute] cd "/tmp/.tmp2VeAJJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2VeAJJ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2VeAJJ/incremental-state"
[2023-03-20T15:37:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:37:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:37:45Z DEBUG collector::execute] cd "/tmp/.tmpvNNQf2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvNNQf2#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:45Z DEBUG collector::execute] cd "/tmp/.tmpvNNQf2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvNNQf2#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvNNQf2/incremental-state"
[2023-03-20T15:37:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:45Z DEBUG collector::execute] cd "/tmp/.tmpvNNQf2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvNNQf2#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvNNQf2/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-03-20T15:37:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T15:37:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:37:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T15:37:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T15:37:46Z DEBUG collector::execute] cd "/tmp/.tmpFrBDv1" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFrBDv1#tuple-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T15:37:46Z DEBUG collector::execute] cd "/tmp/.tmpwTYdQm" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwTYdQm#tuple-stress@0.1.0" "--" "--skip-this-rustc"
[2023-03-20T15:37:46Z DEBUG collector::execute] cd "/tmp/.tmpGdnhyt" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGdnhyt#tuple-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T15:37:46Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:37:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T15:37:46Z DEBUG collector::execute] cd "/tmp/.tmp0pKmQF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0pKmQF#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:37:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-20T15:37:49Z DEBUG collector::execute] cd "/tmp/.tmp0pKmQF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0pKmQF#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0pKmQF/incremental-state"
[2023-03-20T15:37:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:37:52Z DEBUG collector::execute] cd "/tmp/.tmp0pKmQF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0pKmQF#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0pKmQF/incremental-state"
[2023-03-20T15:37:53Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmp0pKmQF"
[2023-03-20T15:37:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-20T15:37:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-20T15:37:53Z DEBUG collector::execute] cd "/tmp/.tmp0pKmQF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0pKmQF#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0pKmQF/incremental-state"
[2023-03-20T15:37:57Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:37:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T15:37:57Z DEBUG collector::execute] cd "/tmp/.tmpQa5OOW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQa5OOW#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:38:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-20T15:38:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-20T15:38:00Z DEBUG collector::execute] cd "/tmp/.tmpQa5OOW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQa5OOW#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQa5OOW/incremental-state"
[2023-03-20T15:38:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:38:04Z DEBUG collector::execute] cd "/tmp/.tmpQa5OOW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQa5OOW#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQa5OOW/incremental-state"
[2023-03-20T15:38:05Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmpQa5OOW"
[2023-03-20T15:38:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-20T15:38:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-20T15:38:05Z DEBUG collector::execute] cd "/tmp/.tmpQa5OOW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQa5OOW#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQa5OOW/incremental-state"
[2023-03-20T15:38:09Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T15:38:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T15:38:09Z DEBUG collector::execute] cd "/tmp/.tmpsx2lku" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsx2lku#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T15:38:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-20T15:38:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-20T15:38:12Z DEBUG collector::execute] cd "/tmp/.tmpsx2lku" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsx2lku#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsx2lku/incremental-state"
[2023-03-20T15:38:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-20T15:38:16Z DEBUG collector::execute] cd "/tmp/.tmpsx2lku" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsx2lku#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsx2lku/incremental-state"
[2023-03-20T15:38:17Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmpsx2lku"
[2023-03-20T15:38:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-20T15:38:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-20T15:38:17Z DEBUG collector::execute] cd "/tmp/.tmpsx2lku" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsx2lku#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsx2lku/incremental-state"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging Rustc PGO profiles to /tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata
stage-build INFO: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata /tmp/tmp-multistage/opt-artifacts/rustc-pgo`
stage-build INFO: Rustc PGO statistics
---
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
warning: rustc_smir.3c19d3f1-cgu.9: no profile data available for function _RNvXNtNtCsjIMh3oThLKZ_4core5slice3cmpShNtNtB6_3cmp9PartialEq2eqCsj1lHTfP6edN_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvMs5_NtCsbE5LYh4I2LC_9hashbrown3rawINtB6_8RawTableTNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE0ECsj1lHTfP6edN_10rustc_smir Hash = 1126541921141000735 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvMs5_NtCsbE5LYh4I2LC_9hashbrown3rawINtB6_8RawTableTNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE0ECsj1lHTfP6edN_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvMs5_NtCsbE5LYh4I2LC_9hashbrown3rawINtB6_8RawTableTNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE0ECsj1lHTfP6edN_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCsbE5LYh4I2LC_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCshjKJD5yuCTO_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECsj1lHTfP6edN_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCsbE5LYh4I2LC_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCshjKJD5yuCTO_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECsj1lHTfP6edN_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RINvNtNtCsjIMh3oThLKZ_4core5alloc6layout10size_alignTNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RINvYINtNtNtCsjIMh3oThLKZ_4core5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexENtNtNtNtBa_4iter6traits8iterator8Iterator6copiedBJ_ECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvMs0_CsktCRv2KoZRe_8indexmapINtB5_6BucketNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIduE7key_refCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvMs_NtNtNtNtNtCs1JDRsBSSWmt_3std3sys6common12thread_local10fast_local4fastINtB4_3KeyINtNtCsjIMh3oThLKZ_4core4cell4CellPuEE13register_dtorCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertINtNtNtB7_3ptr6unique6UniquehEINtB5_4IntoINtNtBD_8non_null7NonNullhEE4intoCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumINtB5_4IntojE4intoCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertjINtB5_4IntoNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumE4intoCsj1lHTfP6edN_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXs5_NtCsjIMh3oThLKZ_4core7convertjINtB5_7TryFromjE8try_fromCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXsV_NtCsjIMh3oThLKZ_4core3fmtRNtNtCs6OU47sCiABw_10rustc_span6def_id5DefIdNtB5_5Debug3fmtCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXsV_NtCsjIMh3oThLKZ_4core3fmtRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVechEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeRNtNtCs6OU47sCiABw_10rustc_span6def_id5DefIdECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeRbECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placejECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h6919db7d9fe242a8E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17hb0500e3b4c9eec0eE Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10stable_mir11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10stable_mir10find_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10stable_mir15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10stable_mir15all_local_items Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvXs3_NtCsj1lHTfP6edN_10rustc_smir10stable_mirNtB5_5CrateNtNtCsjIMh3oThLKZ_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvXs9_NtCsj1lHTfP6edN_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCsjIMh3oThLKZ_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVechEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNvNtNtNtNtCskIPaQApQrQC_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvXs2_NtNtNtCsjIMh3oThLKZ_4core3ops8function5implsQNCNCNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15all_local_items00INtB7_6FnOnceTRNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdEE9call_onceBW_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir11local_crate Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15external_crates Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir10find_crate Hash = 578412082070520035 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15all_local_items Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir10smir_crate Hash = 737863019117533973 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RINvNtCshjKJD5yuCTO_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsj1lHTfP6edN_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RINvNvMs_NtCshjKJD5yuCTO_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RINvNvMs_NtCshjKJD5yuCTO_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvMs0_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvMs0_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvMs_NtCshjKJD5yuCTO_5alloc7raw_vecINtB4_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvMs_NtCshjKJD5yuCTO_5alloc7raw_vecINtB4_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvXs1_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateENtNtNtCsjIMh3oThLKZ_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvXs1_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemENtNtNtCsjIMh3oThLKZ_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvXs1_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVechENtNtNtCsjIMh3oThLKZ_4core3ops4drop4Drop4dropCsj1lHTfP6edN_10rustc_smir Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.15: no profile data available for function _RINvNtNtNtCsbE5LYh4I2LC_9hashbrown3raw5alloc5inner8do_allocNtNtCshjKJD5yuCTO_5alloc5alloc6GlobalECsj1lHTfP6edN_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.15: no profile data available for function _RINvXs8_NtCsbE5LYh4I2LC_9hashbrown3setINtB6_7HashSetNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtNtNtB21_4iter6traits7collect6ExtendBO_E6extendINtNtNtB3t_8adapters6copied6CopiedINtNtNtB21_5slice4iter4IterBO_EEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.15: no profile data available for function _RNvMs2_NtCsbE5LYh4I2LC_9hashbrown3setINtB5_7HashSetNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE6insertCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvMs5_NtCsjIMh3oThLKZ_4core3anyNtB6_6TypeId2ofNtNtCs8EKSgVI6S3c_12tracing_core8callsite15DefaultCallsiteECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCskIPaQApQrQC_12rustc_middle9dep_graph8dep_node7DepKindEEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvXNtCskIPaQApQrQC_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECsj1lHTfP6edN_10rustc_smir Hash = 269792486063750595 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvXsv_NtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCsjIMh3oThLKZ_4core4hash4Hash4hashNtCseuWXvfe3zED_10rustc_hash8FxHasherECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCsj1lHTfP6edN_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RNvNvNtNtNtNtCskIPaQApQrQC_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RNvXNtCsjIMh3oThLKZ_4core6borrowNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_6BorrowBu_E6borrowCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB5_4IntoNtNtCs72tAHySJL1F_21rustc_data_structures9profiling17QueryInvocationIdE4intoCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVechEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RINvXs_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateNCNCNvNtB3x_10rustc_smir10find_crate00E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 23700016549702699 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters5chainINtB2_5ChainINtNtNtB8_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumEBX_E3newCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.1: no profile data available for function _RINvCscdmc6WHFWhN_8smallvec10infallibleuECsj1lHTfP6edN_10rustc_smir Hash = 1063705160175073211 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.1: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtCscdmc6WHFWhN_8smallvec18CollectionAllocErrECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.1: no profile data available for function _RNvMsc_Cscdmc6WHFWhN_8smallvecINtB5_8SmallVecANtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexj8_E11try_reserveCsj1lHTfP6edN_10rustc_smir Hash = 832846567024380499 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateuNCNCNvNtB2a_10rustc_smir15external_crates00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCshjKJD5yuCTO_5alloc3vecINtB4G_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RINvXs0_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsbE5LYh4I2LC_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RINvXs0_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumENCNCNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15external_crates00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3c_8for_each4callNtNtB2g_10stable_mir5CrateNCINvMsi_NtCshjKJD5yuCTO_5alloc3vecINtB4O_3VecB4f_E14extend_trustedBN_E0E0EB2g_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB2_3MapINtNtB4_6copied6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsbE5LYh4I2LC_9hashbrown3setINtB2Y_7HashSetB1F_INtNtB8_4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtNtB6_6traits7collect6ExtendB1F_E6extendBT_E0E3newCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB2_3MapINtNtCsktCRv2KoZRe_8indexmap3set4IterNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdENCNCNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15all_local_items00E3newB2q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB2_3MapINtNtNtB8_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumENCNCNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15external_crates00E3newB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvMsH_NtCs8EKSgVI6S3c_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsjIMh3oThLKZ_4core6option6OptionRDNtB6_5ValueEL_EEj3_ECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvNtCs8EKSgVI6S3c_12tracing_core5field5debugRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvNtCs8EKSgVI6S3c_12tracing_core5field5debugRNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeRNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvXs_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCsbE5LYh4I2LC_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECsj1lHTfP6edN_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters6copiedINtB2_6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEE3newCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RNvXs_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RNvXsu_NtCs8EKSgVI6S3c_12tracing_core5fieldINtB5_10DebugValueRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumENtB5_5Value6recordCsj1lHTfP6edN_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RNvXsu_NtCs8EKSgVI6S3c_12tracing_core5fieldINtB5_10DebugValueRNtNtCshjKJD5yuCTO_5alloc6string6StringENtB5_5Value6recordCsj1lHTfP6edN_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc3vec3VecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc3vec3VecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RNvXNtNtCshjKJD5yuCTO_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumENCNCNvNtBY_10rustc_smir15external_crates00EE9from_iterBY_ Hash = 1096621587988718700 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RNvXNtNtCshjKJD5yuCTO_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3map3MapINtNtCsktCRv2KoZRe_8indexmap3set4IterNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdENCNCNvNtBY_10rustc_smir15all_local_items00EE9from_iterBY_ Hash = 421769329463800046 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RNvXsn_NtCshjKJD5yuCTO_5alloc3vecINtB5_3VechENtNtNtCsjIMh3oThLKZ_4core3ops4drop4Drop4dropCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.13: no profile data available for function _RNvMs2_NtCsktCRv2KoZRe_8indexmap3setINtB5_8IndexSetNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE4iterCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.13: no profile data available for function _RNvXsb_NtCsktCRv2KoZRe_8indexmap3setINtB5_4IterNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdENtNtNtNtCsjIMh3oThLKZ_4core4iter6traits8iterator8Iterator4nextCsj1lHTfP6edN_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.13: no profile data available for function _RNvXsb_NtCsktCRv2KoZRe_8indexmap3setINtB5_4IterNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdENtNtNtNtCsjIMh3oThLKZ_4core4iter6traits8iterator8Iterator9size_hintCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.5: no profile data available for function _RINvNtCsbE5LYh4I2LC_9hashbrown3map9make_hashNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexBG_INtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.5: no profile data available for function _RINvXs1x_NtCsbE5LYh4I2LC_9hashbrown3mapINtB7_7HashMapNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECsj1lHTfP6edN_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.5: no profile data available for function _RNvMs1_NtCsbE5LYh4I2LC_9hashbrown3mapINtB5_7HashMapNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE6insertCsj1lHTfP6edN_10rustc_smir Hash = 11922953328495428 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvMs2_NtNtCs1JDRsBSSWmt_3std6thread5localINtB6_8LocalKeyINtNtCsjIMh3oThLKZ_4core4cell4CellPuEE4withNCNvNtNtNtNtCskIPaQApQrQC_12rustc_middle2ty7context3tls3tlv7get_tlv0B1s_ECsj1lHTfP6edN_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCs72tAHySJL1F_21rustc_data_structures4sync11assert_syncNtNtNtNtCskIPaQApQrQC_12rustc_middle2ty7context3tls12ImplicitCtxtECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVechEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtNtCs1JDRsBSSWmt_3std6thread5local11AccessErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvYINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertNtNtCshjKJD5yuCTO_5alloc11collections19TryReserveErrorKindINtB5_4IntoNtBA_15TryReserveErrorE4intoCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvXs1_NtNtNtCsjIMh3oThLKZ_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateNCNCNvNtB2E_10rustc_smir10find_crate00E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 1147234025963109369 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvXsV_NtCsjIMh3oThLKZ_4core3fmtRNtNtCshjKJD5yuCTO_5alloc6string6StringNtB5_5Debug3fmtCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvXsV_NtCsjIMh3oThLKZ_4core3fmtRbNtB5_5Debug3fmtCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir14rustc_internal11item_def_id Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir14rustc_internal9crate_num Hash = 742261418966908927 up to 0 count discarded
[RUSTC-TIMING] rustc_smir test:false 2.878
warning: `rustc_smir` (lib) generated 111 warnings
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
[RUSTC-TIMING] rustc_symbol_mangling test:false 5.553
---
Preparing cargo-0.60.0
[2023-03-20T16:07:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T16:07:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:07:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:07:43Z DEBUG collector::execute] cd "/tmp/.tmpAAvMZG" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAAvMZG#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-20T16:07:43Z DEBUG collector::execute] cd "/tmp/.tmp9wSPeg" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9wSPeg#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-20T16:07:43Z DEBUG collector::execute] cd "/tmp/.tmp4qZlnB" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4qZlnB#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-03-20T16:08:49Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:08:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:08:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:08:49Z DEBUG collector::execute] cd "/tmp/.tmpTZwm2r" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTZwm2r#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:08:55Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:08:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:08:55Z DEBUG collector::execute] cd "/tmp/.tmp4g7haD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4g7haD#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Opt + [Full]
Running cargo-0.60.0: Opt + [Full]
[2023-03-20T16:09:14Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:09:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:09:15Z DEBUG collector::execute] cd "/tmp/.tmpfqmWH5" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfqmWH5#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/8)
Preparing clap-3.1.6
[2023-03-20T16:09:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T16:09:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:09:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:09:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:09:58Z DEBUG collector::execute] cd "/tmp/.tmpg97515" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpg97515#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-03-20T16:09:58Z DEBUG collector::execute] cd "/tmp/.tmpok6Q0H" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpok6Q0H#clap@3.1.6" "--" "--skip-this-rustc"
[2023-03-20T16:09:58Z DEBUG collector::execute] cd "/tmp/.tmpghWE1Y" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpghWE1Y#clap@3.1.6" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T16:10:03Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:10:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:10:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:10:03Z DEBUG collector::execute] cd "/tmp/.tmpksZFaY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpksZFaY#clap@3.1.6" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:10:04Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:10:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:10:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:10:04Z DEBUG collector::execute] cd "/tmp/.tmpglVdzB" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpglVdzB#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:10:07Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:10:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:10:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:10:07Z DEBUG collector::execute] cd "/tmp/.tmpISCXmb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpISCXmb#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-03-20T16:10:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T16:10:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:10:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:10:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:10:13Z DEBUG collector::execute] cd "/tmp/.tmpnFPXS8" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnFPXS8#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-20T16:10:13Z DEBUG collector::execute] cd "/tmp/.tmpiLNtNs" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiLNtNs#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-20T16:10:13Z DEBUG collector::execute] cd "/tmp/.tmpMMqx7Q" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMMqx7Q#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-20T16:10:38Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:10:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:10:38Z DEBUG collector::execute] cd "/tmp/.tmpJoAf16" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJoAf16#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Debug + [Full]
Running hyper-0.14.18: Debug + [Full]
[2023-03-20T16:10:40Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:10:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:10:40Z DEBUG collector::execute] cd "/tmp/.tmpHWAb1u" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHWAb1u#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:10:43Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:10:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:10:44Z DEBUG collector::execute] cd "/tmp/.tmp524AXf" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp524AXf#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/8)
Finished benchmark hyper-0.14.18 (3/8)
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-03-20T16:10:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T16:10:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:10:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:10:51Z DEBUG collector::execute] cd "/tmp/.tmpwi8gdo" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwi8gdo#regex@1.5.5" "--" "--skip-this-rustc"
[2023-03-20T16:10:51Z DEBUG collector::execute] cd "/tmp/.tmp3Z3MJT" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3Z3MJT#regex@1.5.5" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T16:10:51Z DEBUG collector::execute] cd "/tmp/.tmprPCHuR" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprPCHuR#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-03-20T16:10:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:10:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:10:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:10:59Z DEBUG collector::execute] cd "/tmp/.tmpkfmxlq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkfmxlq#regex@1.5.5" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:10:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:10:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:10:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:10:59Z DEBUG collector::execute] cd "/tmp/.tmpeYs2Ub" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeYs2Ub#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:11:02Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:11:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:11:02Z DEBUG collector::execute] cd "/tmp/.tmpd23RJy" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd23RJy#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/8)
Finished benchmark regex-1.5.5 (4/8)
Executing benchmark ripgrep-13.0.0 (5/8)
Preparing ripgrep-13.0.0
[2023-03-20T16:11:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:11:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T16:11:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:11:09Z DEBUG collector::execute] cd "/tmp/.tmpQwYk0f" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQwYk0f#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-03-20T16:11:09Z DEBUG collector::execute] cd "/tmp/.tmptzvYpn" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptzvYpn#ripgrep@13.0.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T16:11:09Z DEBUG collector::execute] cd "/tmp/.tmp2ClTd7" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2ClTd7#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T16:11:41Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:11:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:11:42Z DEBUG collector::execute] cd "/tmp/.tmpuujV6u" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuujV6u#ripgrep@13.0.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Debug + [Full]
Running ripgrep-13.0.0: Debug + [Full]
[2023-03-20T16:11:42Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:11:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:11:42Z DEBUG collector::execute] cd "/tmp/.tmp9Q7hGl" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9Q7hGl#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:11:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:11:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:11:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:11:46Z DEBUG collector::execute] cd "/tmp/.tmpL6RJWv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL6RJWv#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0-tiny (6/8)
Preparing ripgrep-13.0.0-tiny
[2023-03-20T16:11:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:11:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:11:55Z DEBUG collector::execute] cd "/tmp/.tmpSThktI" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSThktI#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-03-20T16:12:08Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:12:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:12:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:12:08Z DEBUG collector::execute] cd "/tmp/.tmprUZsZy" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprUZsZy#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (7/8)
Preparing serde-1.0.136
[2023-03-20T16:12:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:12:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T16:12:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T16:12:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:12:40Z DEBUG collector::execute] cd "/tmp/.tmpbhUYX1" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbhUYX1#serde@1.0.136" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T16:12:40Z DEBUG collector::execute] cd "/tmp/.tmpQAqH0Q" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQAqH0Q#serde@1.0.136" "--" "--skip-this-rustc"
[2023-03-20T16:12:40Z DEBUG collector::execute] cd "/tmp/.tmp1Fimet" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1Fimet#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-03-20T16:12:41Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:12:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:12:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:12:41Z DEBUG collector::execute] cd "/tmp/.tmpIcLgO7" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIcLgO7#serde@1.0.136" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:12:43Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:12:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:12:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:12:43Z DEBUG collector::execute] cd "/tmp/.tmpgNaBex" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgNaBex#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:12:46Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:12:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:12:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:12:46Z DEBUG collector::execute] cd "/tmp/.tmpdKMy6f" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdKMy6f#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (8/8)
Preparing syn-1.0.89
[2023-03-20T16:12:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-20T16:12:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:12:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-20T16:12:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-20T16:12:49Z DEBUG collector::execute] cd "/tmp/.tmpFrJGHS" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFrJGHS#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-03-20T16:12:49Z DEBUG collector::execute] cd "/tmp/.tmpnhuEpF" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnhuEpF#syn@1.0.89" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-20T16:12:49Z DEBUG collector::execute] cd "/tmp/.tmp5tESmq" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5tESmq#syn@1.0.89" "--" "--skip-this-rustc"
[2023-03-20T16:12:54Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:12:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:12:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-20T16:12:54Z DEBUG collector::execute] cd "/tmp/.tmpUQRsfM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUQRsfM#syn@1.0.89" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:12:55Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:12:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:12:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-20T16:12:55Z DEBUG collector::execute] cd "/tmp/.tmpbJ2FzH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbJ2FzH#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-20T16:12:58Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-03-20T16:12:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-20T16:12:58Z DEBUG collector::execute] cd "/tmp/.tmpPP57Dh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPP57Dh#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (8/8)
---
[RUSTC-TIMING] rustc_incremental test:false 9.503
[RUSTC-TIMING] rustc_query_impl test:false 91.746
[RUSTC-TIMING] rustc_monomorphize test:false 7.710
[RUSTC-TIMING] rustc_ast_lowering test:false 20.973
warning: rustc_smir.3c19d3f1-cgu.9: no profile data available for function _RNvXNtNtCsjIMh3oThLKZ_4core5slice3cmpShNtNtB6_3cmp9PartialEq2eqCsj1lHTfP6edN_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvMs5_NtCsbE5LYh4I2LC_9hashbrown3rawINtB6_8RawTableTNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE0ECsj1lHTfP6edN_10rustc_smir Hash = 1126541921141000735 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvMs5_NtCsbE5LYh4I2LC_9hashbrown3rawINtB6_8RawTableTNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE0ECsj1lHTfP6edN_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvMs5_NtCsbE5LYh4I2LC_9hashbrown3rawINtB6_8RawTableTNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE0ECsj1lHTfP6edN_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCsbE5LYh4I2LC_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCshjKJD5yuCTO_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECsj1lHTfP6edN_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.2: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCsbE5LYh4I2LC_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCshjKJD5yuCTO_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECsj1lHTfP6edN_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RINvNtNtCsjIMh3oThLKZ_4core5alloc6layout10size_alignTNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RINvYINtNtNtCsjIMh3oThLKZ_4core5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexENtNtNtNtBa_4iter6traits8iterator8Iterator6copiedBJ_ECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvMs0_CsktCRv2KoZRe_8indexmapINtB5_6BucketNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIduE7key_refCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvMs_NtNtNtNtNtCs1JDRsBSSWmt_3std3sys6common12thread_local10fast_local4fastINtB4_3KeyINtNtCsjIMh3oThLKZ_4core4cell4CellPuEE13register_dtorCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertINtNtNtB7_3ptr6unique6UniquehEINtB5_4IntoINtNtBD_8non_null7NonNullhEE4intoCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumINtB5_4IntojE4intoCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertjINtB5_4IntoNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumE4intoCsj1lHTfP6edN_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXs5_NtCsjIMh3oThLKZ_4core7convertjINtB5_7TryFromjE8try_fromCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXsV_NtCsjIMh3oThLKZ_4core3fmtRNtNtCs6OU47sCiABw_10rustc_span6def_id5DefIdNtB5_5Debug3fmtCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.14: no profile data available for function _RNvXsV_NtCsjIMh3oThLKZ_4core3fmtRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVechEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeRNtNtCs6OU47sCiABw_10rustc_span6def_id5DefIdECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeRbECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placejECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h6919db7d9fe242a8E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17hb0500e3b4c9eec0eE Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10stable_mir11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10stable_mir10find_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10stable_mir15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10stable_mir15all_local_items Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvXs3_NtCsj1lHTfP6edN_10rustc_smir10stable_mirNtB5_5CrateNtNtCsjIMh3oThLKZ_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.11: no profile data available for function _RNvXs9_NtCsj1lHTfP6edN_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCsjIMh3oThLKZ_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVechEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNvNtNtNtNtCskIPaQApQrQC_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvXs2_NtNtNtCsjIMh3oThLKZ_4core3ops8function5implsQNCNCNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15all_local_items00INtB7_6FnOnceTRNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdEE9call_onceBW_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir11local_crate Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15external_crates Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir10find_crate Hash = 578412082070520035 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15all_local_items Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.4: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir10smir_crate Hash = 737863019117533973 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RINvNtCshjKJD5yuCTO_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsj1lHTfP6edN_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RINvNvMs_NtCshjKJD5yuCTO_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RINvNvMs_NtCshjKJD5yuCTO_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvMs0_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvMs0_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvMs_NtCshjKJD5yuCTO_5alloc7raw_vecINtB4_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvMs_NtCshjKJD5yuCTO_5alloc7raw_vecINtB4_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvXs1_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateENtNtNtCsjIMh3oThLKZ_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvXs1_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemENtNtNtCsjIMh3oThLKZ_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.0: no profile data available for function _RNvXs1_NtCshjKJD5yuCTO_5alloc7raw_vecINtB5_6RawVechENtNtNtCsjIMh3oThLKZ_4core3ops4drop4Drop4dropCsj1lHTfP6edN_10rustc_smir Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.15: no profile data available for function _RINvNtNtNtCsbE5LYh4I2LC_9hashbrown3raw5alloc5inner8do_allocNtNtCshjKJD5yuCTO_5alloc5alloc6GlobalECsj1lHTfP6edN_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.15: no profile data available for function _RINvXs8_NtCsbE5LYh4I2LC_9hashbrown3setINtB6_7HashSetNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtNtNtB21_4iter6traits7collect6ExtendBO_E6extendINtNtNtB3t_8adapters6copied6CopiedINtNtNtB21_5slice4iter4IterBO_EEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.15: no profile data available for function _RNvMs2_NtCsbE5LYh4I2LC_9hashbrown3setINtB5_7HashSetNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE6insertCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvMs5_NtCsjIMh3oThLKZ_4core3anyNtB6_6TypeId2ofNtNtCs8EKSgVI6S3c_12tracing_core8callsite15DefaultCallsiteECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCskIPaQApQrQC_12rustc_middle9dep_graph8dep_node7DepKindEEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvXNtCskIPaQApQrQC_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECsj1lHTfP6edN_10rustc_smir Hash = 269792486063750595 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RINvXsv_NtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCsjIMh3oThLKZ_4core4hash4Hash4hashNtCseuWXvfe3zED_10rustc_hash8FxHasherECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCsj1lHTfP6edN_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RNvNvNtNtNtNtCskIPaQApQrQC_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RNvXNtCsjIMh3oThLKZ_4core6borrowNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_6BorrowBu_E6borrowCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.12: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB5_4IntoNtNtCs72tAHySJL1F_21rustc_data_structures9profiling17QueryInvocationIdE4intoCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVechEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RINvXs_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateNCNCNvNtB3x_10rustc_smir10find_crate00E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 23700016549702699 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.6: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters5chainINtB2_5ChainINtNtNtB8_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumEBX_E3newCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.1: no profile data available for function _RINvCscdmc6WHFWhN_8smallvec10infallibleuECsj1lHTfP6edN_10rustc_smir Hash = 1063705160175073211 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.1: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtCscdmc6WHFWhN_8smallvec18CollectionAllocErrECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.1: no profile data available for function _RNvMsc_Cscdmc6WHFWhN_8smallvecINtB5_8SmallVecANtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexj8_E11try_reserveCsj1lHTfP6edN_10rustc_smir Hash = 832846567024380499 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateuNCNCNvNtB2a_10rustc_smir15external_crates00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCshjKJD5yuCTO_5alloc3vecINtB4G_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RINvXs0_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsbE5LYh4I2LC_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RINvXs0_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumENCNCNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15external_crates00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3c_8for_each4callNtNtB2g_10stable_mir5CrateNCINvMsi_NtCshjKJD5yuCTO_5alloc3vecINtB4O_3VecB4f_E14extend_trustedBN_E0E0EB2g_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB2_3MapINtNtB4_6copied6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsbE5LYh4I2LC_9hashbrown3setINtB2Y_7HashSetB1F_INtNtB8_4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtNtB6_6traits7collect6ExtendB1F_E6extendBT_E0E3newCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB2_3MapINtNtCsktCRv2KoZRe_8indexmap3set4IterNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdENCNCNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15all_local_items00E3newB2q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.7: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3mapINtB2_3MapINtNtNtB8_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumENCNCNvNtCsj1lHTfP6edN_10rustc_smir10rustc_smir15external_crates00E3newB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvMsH_NtCs8EKSgVI6S3c_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsjIMh3oThLKZ_4core6option6OptionRDNtB6_5ValueEL_EEj3_ECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvNtCs8EKSgVI6S3c_12tracing_core5field5debugRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvNtCs8EKSgVI6S3c_12tracing_core5field5debugRNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeRNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RINvXs_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCsbE5LYh4I2LC_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECsj1lHTfP6edN_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RNvMNtNtNtCsjIMh3oThLKZ_4core4iter8adapters6copiedINtB2_6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEE3newCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RNvXs_NtNtNtCsjIMh3oThLKZ_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RNvXsu_NtCs8EKSgVI6S3c_12tracing_core5fieldINtB5_10DebugValueRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumENtB5_5Value6recordCsj1lHTfP6edN_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.8: no profile data available for function _RNvXsu_NtCs8EKSgVI6S3c_12tracing_core5fieldINtB5_10DebugValueRNtNtCshjKJD5yuCTO_5alloc6string6StringENtB5_5Value6recordCsj1lHTfP6edN_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc3vec3VecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc3vec3VecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RNvXNtNtCshjKJD5yuCTO_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumENCNCNvNtBY_10rustc_smir15external_crates00EE9from_iterBY_ Hash = 1096621587988718700 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RNvXNtNtCshjKJD5yuCTO_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCsjIMh3oThLKZ_4core4iter8adapters3map3MapINtNtCsktCRv2KoZRe_8indexmap3set4IterNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdENCNCNvNtBY_10rustc_smir15all_local_items00EE9from_iterBY_ Hash = 421769329463800046 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.3: no profile data available for function _RNvXsn_NtCshjKJD5yuCTO_5alloc3vecINtB5_3VechENtNtNtCsjIMh3oThLKZ_4core3ops4drop4Drop4dropCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.13: no profile data available for function _RNvMs2_NtCsktCRv2KoZRe_8indexmap3setINtB5_8IndexSetNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE4iterCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.13: no profile data available for function _RNvXsb_NtCsktCRv2KoZRe_8indexmap3setINtB5_4IterNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdENtNtNtNtCsjIMh3oThLKZ_4core4iter6traits8iterator8Iterator4nextCsj1lHTfP6edN_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.13: no profile data available for function _RNvXsb_NtCsktCRv2KoZRe_8indexmap3setINtB5_4IterNtNtCs6OU47sCiABw_10rustc_span6def_id10LocalDefIdENtNtNtNtCsjIMh3oThLKZ_4core4iter6traits8iterator8Iterator9size_hintCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.5: no profile data available for function _RINvNtCsbE5LYh4I2LC_9hashbrown3map9make_hashNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexBG_INtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.5: no profile data available for function _RINvXs1x_NtCsbE5LYh4I2LC_9hashbrown3mapINtB7_7HashMapNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECsj1lHTfP6edN_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.5: no profile data available for function _RNvMs1_NtCsbE5LYh4I2LC_9hashbrown3mapINtB5_7HashMapNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherEE6insertCsj1lHTfP6edN_10rustc_smir Hash = 11922953328495428 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvMs2_NtNtCs1JDRsBSSWmt_3std6thread5localINtB6_8LocalKeyINtNtCsjIMh3oThLKZ_4core4cell4CellPuEE4withNCNvNtNtNtNtCskIPaQApQrQC_12rustc_middle2ty7context3tls3tlv7get_tlv0B1s_ECsj1lHTfP6edN_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCs72tAHySJL1F_21rustc_data_structures4sync11assert_syncNtNtNtNtCskIPaQApQrQC_12rustc_middle2ty7context3tls12ImplicitCtxtECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeINtNtCshjKJD5yuCTO_5alloc7raw_vec6RawVechEECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtCshjKJD5yuCTO_5alloc6string6StringECsj1lHTfP6edN_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvNtCsjIMh3oThLKZ_4core3ptr13drop_in_placeNtNtNtCs1JDRsBSSWmt_3std6thread5local11AccessErrorECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RINvYINtNtCsjIMh3oThLKZ_4core4hash18BuildHasherDefaultNtCseuWXvfe3zED_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCslkIXeHqJMIa_18rustc_query_system9dep_graph5graph12DepNodeIndexECsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvXs1_NtCsjIMh3oThLKZ_4core7convertNtNtCshjKJD5yuCTO_5alloc11collections19TryReserveErrorKindINtB5_4IntoNtBA_15TryReserveErrorE4intoCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvXs1_NtNtNtCsjIMh3oThLKZ_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCs6OU47sCiABw_10rustc_span6def_id8CrateNumNtNtCsj1lHTfP6edN_10rustc_smir10stable_mir5CrateNCNCNvNtB2E_10rustc_smir10find_crate00E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 1147234025963109369 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvXsV_NtCsjIMh3oThLKZ_4core3fmtRNtNtCshjKJD5yuCTO_5alloc6string6StringNtB5_5Debug3fmtCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvXsV_NtCsjIMh3oThLKZ_4core3fmtRbNtB5_5Debug3fmtCsj1lHTfP6edN_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir14rustc_internal11item_def_id Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.3c19d3f1-cgu.10: no profile data available for function _RNvNtCsj1lHTfP6edN_10rustc_smir14rustc_internal9crate_num Hash = 742261418966908927 up to 0 count discarded
[RUSTC-TIMING] rustc_smir test:false 2.878
warning: `rustc_smir` (lib) generated 111 warnings
[RUSTC-TIMING] rustc_transmute test:false 7.407
[RUSTC-TIMING] rustc_metadata test:false 36.504
---
   3:     0x7f94fad73c2a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4a605a2ee92d2fe0
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f94fadd724e - core::fmt::write::h91f245f7b204700b
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/core/src/fmt/mod.rs:1254:17
   5:     0x7f94fad669b5 - std::io::Write::write_fmt::h0beff5754ce52d5f
   6:     0x7f94fad739f5 - std::sys_common::backtrace::_print::h053e5eb7d9e406de
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f94fad739f5 - std::sys_common::backtrace::print::h7e97a35767dabc66
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/sys_common/backtrace.rs:34:9
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f94fad7676f - std::panicking::default_hook::{{closure}}::hc62a5029cca001d4
   9:     0x7f94fad764ab - std::panicking::default_hook::hf543ded32e2d91bb
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/panicking.rs:290:9
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/panicking.rs:290:9
  10:     0x7f94fdfbb875 - <rustc_driver_impl[ee9c894de86c31cc]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[e5b79299eea94447]::ops::function::FnOnce<(&core[e5b79299eea94447]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f94fad76fad - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h93da42638d19d817
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/alloc/src/boxed.rs:2002:9
  12:     0x7f94fad76fad - std::panicking::rust_panic_with_hook::heb4aa8f5547f3692
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/panicking.rs:696:13
  13:     0x7f94fad76d29 - std::panicking::begin_panic_handler::{{closure}}::h26bd0e7d8631a3e7
  14:     0x7f94fad74096 - std::sys_common::backtrace::__rust_end_short_backtrace::h9c671ed32bc68220
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f94fad76a32 - rust_begin_unwind
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/panicking.rs:579:5
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/panicking.rs:579:5
  16:     0x7f94fadd35a3 - core::panicking::panic_fmt::ha88a76726ac4991a
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/core/src/panicking.rs:64:14
  17:     0x7f94fadd3b33 - core::result::unwrap_failed::h508e2cbe657f262d
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/core/src/result.rs:1750:5
  18:     0x555b48dc5d16 - rustdoc[78e7dacd13187d29]::html::render::assoc_method
  19:     0x555b48dc6851 - rustdoc[78e7dacd13187d29]::html::render::render_assoc_item
  20:     0x555b48cef435 - rustdoc[78e7dacd13187d29]::html::render::print_item::item_trait
  21:     0x555b48ce69c6 - rustdoc[78e7dacd13187d29]::html::render::print_item::print_item
  22:     0x555b48e63a7b - <rustdoc[78e7dacd13187d29]::html::render::context::Context>::render_item
  23:     0x555b48e6be2c - <rustdoc[78e7dacd13187d29]::html::render::context::Context as rustdoc[78e7dacd13187d29]::formats::renderer::FormatRenderer>::item
  24:     0x555b48db9964 - rustdoc[78e7dacd13187d29]::formats::renderer::run_format::<rustdoc[78e7dacd13187d29]::html::render::context::Context>
  25:     0x555b48bfca6c - rustdoc[78e7dacd13187d29]::run_renderer::<rustdoc[78e7dacd13187d29]::html::render::context::Context>
  26:     0x555b48d9ba9a - <rustc_session[574b992fd3096d29]::session::Session>::time::<core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>, rustdoc[78e7dacd13187d29]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#1}>
  27:     0x555b48d2ddb1 - <rustc_middle[f15f9cc93451149c]::ty::context::GlobalCtxt>::enter::<rustdoc[78e7dacd13187d29]::main_args::{closure#1}::{closure#0}::{closure#0}, core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>>
  28:     0x555b48c73590 - <rustc_interface[73d1f314352735e6]::interface::Compiler>::enter::<rustdoc[78e7dacd13187d29]::main_args::{closure#1}::{closure#0}, core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>>
  29:     0x555b48d9a79b - rustc_span[4f72e703892d75b8]::with_source_map::<core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>, rustc_interface[73d1f314352735e6]::interface::run_compiler<core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>, rustdoc[78e7dacd13187d29]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  30:     0x555b48c75324 - <scoped_tls[4fc51043ffcee35c]::ScopedKey<rustc_span[4f72e703892d75b8]::SessionGlobals>>::set::<rustc_interface[73d1f314352735e6]::interface::run_compiler<core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>, rustdoc[78e7dacd13187d29]::main_args::{closure#1}>::{closure#0}, core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>>
  31:     0x555b48da8ca0 - std[1438ea0700fdeceb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73d1f314352735e6]::util::run_in_thread_pool_with_globals<rustc_interface[73d1f314352735e6]::interface::run_compiler<core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>, rustdoc[78e7dacd13187d29]::main_args::{closure#1}>::{closure#0}, core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>>
  32:     0x555b48e5637d - <<std[1438ea0700fdeceb]::thread::Builder>::spawn_unchecked_<rustc_interface[73d1f314352735e6]::util::run_in_thread_pool_with_globals<rustc_interface[73d1f314352735e6]::interface::run_compiler<core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>, rustdoc[78e7dacd13187d29]::main_args::{closure#1}>::{closure#0}, core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e5b79299eea94447]::result::Result<(), rustc_span[4f72e703892d75b8]::ErrorGuaranteed>>::{closure#1} as core[e5b79299eea94447]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7f94fad80e83 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha3dc295b5e6b3f90
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/alloc/src/boxed.rs:1988:9
  34:     0x7f94fad80e83 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h5acbc9bfe1613db4
  35:     0x7f94fad80e83 - std::sys::unix::thread::Thread::new::thread_start::h2c90da8f2d6aaf80
                               at /rustc/56fa9753c5bb8be807bdabf9018e03618dd75fbf/library/std/src/sys/unix/thread.rs:108:17
  36:     0x7f94fa43fea5 - start_thread
  37:     0x7f94f9e66b0d - clone
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (56fa9753c 2023-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z unstable-options -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -C link-arg=-fuse-ld=lld -C link-arg=-Wl,--threads=1 -Z unstable-options -Z normalize-docs -Z force-unstable-if-unmarked -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not document `rustc_trait_selection`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_trait_selection compiler/rustc_trait_selection/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=86c31d7526fc42f9 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libitertools-497ff39347c45d16.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-30feeaed54033d04.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-a0f3834143d3b5b7.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-26c221fe9d0708c7.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-3b971be277eda037.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-1900df2da8ef9e8a.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-4f0547a38ac18bfc.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-9a5cb286a6431107.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d8e116e69ab94a70.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-80c8b4338c8cd3e7.rmeta --extern rustc_parse_format=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_parse_format-45c2cc0328d2d562.rmeta --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-3043635f109e916d.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-14514d3459725086.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-58f4424cccd10043.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-55824c27bc60d0d6.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-b27b214a4fc63494.rmeta --extern rustc_transmute=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_transmute-07dbff34a2f94f5e.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-2629e91688622a91.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-4fa15be69e783f44.rmeta --extern-html-root-url 'itertools=https://docs.rs/itertools/0.10.5/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.10.0/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--threads=1 -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.70.0-nightly
  (56fa9753c
  2023-03-20)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 101)
[RUSTC-TIMING] rustc_trait_selection test:false 2.958
Build completed unsuccessfully in 0:03:06
stage-build INFO: Section `Stage 4 (final build)` ended: FAIL (186.14s)
stage-build ERROR: The multi-stage build has failed
---
Total duration:                       1h 24m 20s
------------------------------------------------
root INFO: Free disk space: 478.01 GiB out of total 581.32 GiB (17.77% used)
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
