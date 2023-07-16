plain
Executing benchmark cargo-0.60.0 (1/8)
Preparing cargo-0.60.0
[2023-05-19T09:28:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:28:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:28:08Z DEBUG collector::execute] cd "/tmp/.tmpcblexr" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcblexr#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-05-19T09:28:08Z DEBUG collector::execute] cd "/tmp/.tmpbjK97T" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbjK97T#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-05-19T09:29:02Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:29:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:29:03Z DEBUG collector::execute] cd "/tmp/.tmphO20bD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphO20bD#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Opt + [Full]
---
[2023-05-19T09:31:12Z DEBUG collector::execute] cd "/tmp/.tmpngLl6s" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpngLl6s#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
[2023-05-19T09:31:16Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:31:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:31:16Z DEBUG collector::execute] cd "/tmp/.tmpzsbkNz" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzsbkNz#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-05-19T09:31:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:31:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:31:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:31:25Z DEBUG collector::execute] cd "/tmp/.tmpZd9Blw" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZd9Blw#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-19T09:31:25Z DEBUG collector::execute] cd "/tmp/.tmp3y8VvF" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3y8VvF#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-19T09:31:50Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:31:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:31:50Z DEBUG collector::execute] cd "/tmp/.tmpNc4uky" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNc4uky#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
---
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-05-19T09:32:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:32:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:32:01Z DEBUG collector::execute] cd "/tmp/.tmpCXhwHe" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCXhwHe#regex@1.5.5" "--" "--skip-this-rustc"
[2023-05-19T09:32:01Z DEBUG collector::execute] cd "/tmp/.tmpgvaG24" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgvaG24#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:32:10Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:32:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:32:10Z DEBUG collector::execute] cd "/tmp/.tmp3Us9aW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3Us9aW#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
Running regex-1.5.5: Opt + [Full]
[2023-05-19T09:32:13Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:32:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:32:13Z DEBUG collector::execute] cd "/tmp/.tmpFQqmPM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFQqmPM#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/8)
Preparing ripgrep-13.0.0
[2023-05-19T09:32:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:32:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:32:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:32:24Z DEBUG collector::execute] cd "/tmp/.tmpuGkezn" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuGkezn#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-05-19T09:32:24Z DEBUG collector::execute] cd "/tmp/.tmpoW14v3" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoW14v3#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:32:53Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:32:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:32:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:32:54Z DEBUG collector::execute] cd "/tmp/.tmpCDI8sf" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCDI8sf#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:32:58Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:32:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:32:58Z DEBUG collector::execute] cd "/tmp/.tmpwSY6TQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwSY6TQ#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark ripgrep-13.0.0 (5/8)
Finished benchmark ripgrep-13.0.0 (5/8)
Executing benchmark ripgrep-13.0.0-tiny (6/8)
Preparing ripgrep-13.0.0-tiny
[2023-05-19T09:33:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:33:14Z DEBUG collector::execute] cd "/tmp/.tmpzC1XfA" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzC1XfA#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:33:26Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:33:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:33:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:33:26Z DEBUG collector::execute] cd "/tmp/.tmpu2RrLW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu2RrLW#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (7/8)
Preparing serde-1.0.136
[2023-05-19T09:34:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:34:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:34:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:34:08Z DEBUG collector::execute] cd "/tmp/.tmpUPr1LE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUPr1LE#serde@1.0.136" "--" "--skip-this-rustc"
[2023-05-19T09:34:08Z DEBUG collector::execute] cd "/tmp/.tmpoftMfm" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoftMfm#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:34:08Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:34:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:34:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:34:08Z DEBUG collector::execute] cd "/tmp/.tmpOHaAfs" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOHaAfs#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:34:11Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:34:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:34:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:34:11Z DEBUG collector::execute] cd "/tmp/.tmpBAldHE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBAldHE#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (8/8)
Preparing syn-1.0.89
[2023-05-19T09:34:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:34:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:34:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:34:14Z DEBUG collector::execute] cd "/tmp/.tmp2swsrA" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2swsrA#syn@1.0.89" "--" "--skip-this-rustc"
[2023-05-19T09:34:14Z DEBUG collector::execute] cd "/tmp/.tmpYkXfqh" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYkXfqh#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:34:18Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:34:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:34:18Z DEBUG collector::execute] cd "/tmp/.tmpN96jRn" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN96jRn#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
---
Preparing bitmaps-3.1.0
[2023-05-19T09:48:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:48:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T09:48:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:48:03Z DEBUG collector::execute] cd "/tmp/.tmpKSvSmh" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKSvSmh#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-19T09:48:03Z DEBUG collector::execute] cd "/tmp/.tmpE155lm" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE155lm#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:48:03Z DEBUG collector::execute] cd "/tmp/.tmpfYUAc2" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfYUAc2#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2023-05-19T09:48:03Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:48:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T09:48:03Z DEBUG collector::execute] cd "/tmp/.tmpqxBie2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqxBie2#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:48:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:48:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:48:05Z DEBUG collector::execute] cd "/tmp/.tmpqxBie2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqxBie2#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqxBie2/incremental-state"
[2023-05-19T09:48:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:48:07Z DEBUG collector::execute] cd "/tmp/.tmpqxBie2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqxBie2#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqxBie2/incremental-state"
[2023-05-19T09:48:07Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpqxBie2"
[2023-05-19T09:48:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:48:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:48:07Z DEBUG collector::execute] cd "/tmp/.tmpqxBie2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqxBie2#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqxBie2/incremental-state"
[2023-05-19T09:48:08Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:48:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:48:08Z DEBUG collector::execute] cd "/tmp/.tmpEzCEh3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEzCEh3#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:48:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:48:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:48:09Z DEBUG collector::execute] cd "/tmp/.tmpEzCEh3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEzCEh3#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEzCEh3/incremental-state"
[2023-05-19T09:48:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:48:11Z DEBUG collector::execute] cd "/tmp/.tmpEzCEh3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEzCEh3#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEzCEh3/incremental-state"
[2023-05-19T09:48:12Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpEzCEh3"
[2023-05-19T09:48:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:48:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:48:12Z DEBUG collector::execute] cd "/tmp/.tmpEzCEh3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEzCEh3#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEzCEh3/incremental-state"
[2023-05-19T09:48:13Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:48:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:48:13Z DEBUG collector::execute] cd "/tmp/.tmpn5SXzf" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn5SXzf#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:48:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-05-19T09:51:18Z DEBUG collector::execute] cd "/tmp/.tmpbPP384" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbPP384#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpbPP384/incremental-state"
Running cargo-0.60.0: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-05-19T09:51:25Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:51:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:51:26Z DEBUG collector::execute] cd "/tmp/.tmp7AjCAJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7AjCAJ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:52:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-19T09:52:12Z DEBUG collector::execute] cd "/tmp/.tmp7AjCAJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7AjCAJ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7AjCAJ/incremental-state"
[2023-05-19T09:52:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:52:57Z DEBUG collector::execute] cd "/tmp/.tmp7AjCAJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7AjCAJ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7AjCAJ/incremental-state"
[2023-05-19T09:53:04Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmp7AjCAJ"
[2023-05-19T09:53:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:53:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:53:04Z DEBUG collector::execute] cd "/tmp/.tmp7AjCAJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7AjCAJ#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7AjCAJ/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2023-05-19T09:53:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:53:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T09:53:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T09:53:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:53:17Z DEBUG collector::execute] cd "/tmp/.tmp27PEqk" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp27PEqk#ctfe-stress-5@0.1.0" "--" "--skip-this-rustc"
[2023-05-19T09:53:17Z DEBUG collector::execute] cd "/tmp/.tmpGBjgGD" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGBjgGD#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-19T09:53:17Z DEBUG collector::execute] cd "/tmp/.tmpGHFAwr" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGHFAwr#ctfe-stress-5@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:53:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:53:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T09:53:17Z DEBUG collector::execute] cd "/tmp/.tmpW7nZiN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW7nZiN#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:53:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:53:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:53:22Z DEBUG collector::execute] cd "/tmp/.tmpW7nZiN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW7nZiN#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpW7nZiN/incremental-state"
[2023-05-19T09:53:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:53:29Z DEBUG collector::execute] cd "/tmp/.tmpW7nZiN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW7nZiN#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpW7nZiN/incremental-state"
Running ctfe-stress-5: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-05-19T09:53:29Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:53:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:53:29Z DEBUG collector::execute] cd "/tmp/.tmpscbjMa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpscbjMa#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:53:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:53:35Z DEBUG collector::execute] cd "/tmp/.tmpscbjMa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpscbjMa#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpscbjMa/incremental-state"
[2023-05-19T09:53:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:53:42Z DEBUG collector::execute] cd "/tmp/.tmpscbjMa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpscbjMa#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpscbjMa/incremental-state"
[2023-05-19T09:53:43Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:53:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:53:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:53:43Z DEBUG collector::execute] cd "/tmp/.tmpvGnaBX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvGnaBX#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:53:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-19T09:53:48Z DEBUG collector::execute] cd "/tmp/.tmpvGnaBX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvGnaBX#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvGnaBX/incremental-state"
[2023-05-19T09:53:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:53:55Z DEBUG collector::execute] cd "/tmp/.tmpvGnaBX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvGnaBX#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpvGnaBX/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2023-05-19T09:53:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T09:53:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:53:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:53:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:53:55Z DEBUG collector::execute] cd "/tmp/.tmpFqPEKf" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFqPEKf#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-19T09:53:55Z DEBUG collector::execute] cd "/tmp/.tmpd2bWNE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd2bWNE#diesel@1.4.8" "--" "--skip-this-rustc"
[2023-05-19T09:53:55Z DEBUG collector::execute] cd "/tmp/.tmpobd0SU" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpobd0SU#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:54:12Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:54:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T09:54:12Z DEBUG collector::execute] cd "/tmp/.tmpqP6oMk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqP6oMk#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:54:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:54:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:54:22Z DEBUG collector::execute] cd "/tmp/.tmpqP6oMk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqP6oMk#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqP6oMk/incremental-state"
[2023-05-19T09:54:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:54:34Z DEBUG collector::execute] cd "/tmp/.tmpqP6oMk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqP6oMk#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqP6oMk/incremental-state"
[2023-05-19T09:54:37Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpqP6oMk"
[2023-05-19T09:54:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:54:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:54:37Z DEBUG collector::execute] cd "/tmp/.tmpqP6oMk" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqP6oMk#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpqP6oMk/incremental-state"
[2023-05-19T09:54:39Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:54:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:54:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:54:39Z DEBUG collector::execute] cd "/tmp/.tmpDJNk0f" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDJNk0f#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:54:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:54:51Z DEBUG collector::execute] cd "/tmp/.tmpDJNk0f" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDJNk0f#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDJNk0f/incremental-state"
[2023-05-19T09:55:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:55:05Z DEBUG collector::execute] cd "/tmp/.tmpDJNk0f" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDJNk0f#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDJNk0f/incremental-state"
[2023-05-19T09:55:08Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpDJNk0f"
[2023-05-19T09:55:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:55:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:55:08Z DEBUG collector::execute] cd "/tmp/.tmpDJNk0f" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDJNk0f#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDJNk0f/incremental-state"
[2023-05-19T09:55:11Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:55:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:55:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:55:11Z DEBUG collector::execute] cd "/tmp/.tmpemaesd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpemaesd#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:55:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-19T09:55:24Z DEBUG collector::execute] cd "/tmp/.tmpemaesd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpemaesd#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpemaesd/incremental-state"
[2023-05-19T09:55:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:55:38Z DEBUG collector::execute] cd "/tmp/.tmpemaesd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpemaesd#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpemaesd/incremental-state"
[2023-05-19T09:55:41Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpemaesd"
[2023-05-19T09:55:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:55:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-19T09:55:41Z DEBUG collector::execute] cd "/tmp/.tmpemaesd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpemaesd#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpemaesd/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-05-19T09:55:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T09:55:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:55:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:55:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:55:44Z DEBUG collector::execute] cd "/tmp/.tmproGAWE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmproGAWE#externs@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:55:44Z DEBUG collector::execute] cd "/tmp/.tmpcT2mHq" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcT2mHq#externs@0.1.0" "--" "--skip-this-rustc"
[2023-05-19T09:55:44Z DEBUG collector::execute] cd "/tmp/.tmps6DscG" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps6DscG#externs@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-19T09:55:44Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:55:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T09:55:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T09:55:44Z DEBUG collector::execute] cd "/tmp/.tmpc2CLpM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc2CLpM#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:55:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:55:44Z DEBUG collector::execute] cd "/tmp/.tmpc2CLpM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc2CLpM#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpc2CLpM/incremental-state"
[2023-05-19T09:55:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:55:45Z DEBUG collector::execute] cd "/tmp/.tmpc2CLpM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc2CLpM#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpc2CLpM/incremental-state"
[2023-05-19T09:55:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:55:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:55:45Z DEBUG collector::execute] cd "/tmp/.tmprsbKzN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprsbKzN#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:55:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:55:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:55:46Z DEBUG collector::execute] cd "/tmp/.tmprsbKzN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprsbKzN#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprsbKzN/incremental-state"
[2023-05-19T09:55:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:55:46Z DEBUG collector::execute] cd "/tmp/.tmprsbKzN" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprsbKzN#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmprsbKzN/incremental-state"
[2023-05-19T09:55:47Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:55:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:55:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:55:47Z DEBUG collector::execute] cd "/tmp/.tmpOu0hOl" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOu0hOl#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:55:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-19T09:55:47Z DEBUG collector::execute] cd "/tmp/.tmpOu0hOl" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOu0hOl#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOu0hOl/incremental-state"
[2023-05-19T09:55:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:55:48Z DEBUG collector::execute] cd "/tmp/.tmpOu0hOl" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOu0hOl#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpOu0hOl/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-05-19T09:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T09:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:55:48Z DEBUG collector::execute] cd "/tmp/.tmpoNe0X4" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoNe0X4#match-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-19T09:55:48Z DEBUG collector::execute] cd "/tmp/.tmpWsBtZ2" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWsBtZ2#match-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-19T09:55:48Z DEBUG collector::execute] cd "/tmp/.tmpDmsWA3" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDmsWA3#match-stress@0.1.0" "--" "--skip-this-rustc"
Running match-stress: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-05-19T09:55:48Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T09:55:48Z DEBUG collector::execute] cd "/tmp/.tmph5ZMPg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph5ZMPg#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:55:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:55:50Z DEBUG collector::execute] cd "/tmp/.tmph5ZMPg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph5ZMPg#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmph5ZMPg/incremental-state"
[2023-05-19T09:55:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:55:52Z DEBUG collector::execute] cd "/tmp/.tmph5ZMPg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmph5ZMPg#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmph5ZMPg/incremental-state"
[2023-05-19T09:55:53Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:55:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:55:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:55:53Z DEBUG collector::execute] cd "/tmp/.tmpCajMAT" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCajMAT#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:55:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:55:55Z DEBUG collector::execute] cd "/tmp/.tmpCajMAT" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCajMAT#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCajMAT/incremental-state"
[2023-05-19T09:55:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:55:57Z DEBUG collector::execute] cd "/tmp/.tmpCajMAT" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCajMAT#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCajMAT/incremental-state"
[2023-05-19T09:55:58Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:55:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:55:58Z DEBUG collector::execute] cd "/tmp/.tmpYTcFbA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYTcFbA#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:56:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-19T09:56:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-19T09:56:00Z DEBUG collector::execute] cd "/tmp/.tmpYTcFbA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYTcFbA#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYTcFbA/incremental-state"
[2023-05-19T09:56:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:56:02Z DEBUG collector::execute] cd "/tmp/.tmpYTcFbA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYTcFbA#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYTcFbA/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-05-19T09:56:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T09:56:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:56:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T09:56:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T09:56:03Z DEBUG collector::execute] cd "/tmp/.tmpwReD33" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwReD33#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-05-19T09:56:03Z DEBUG collector::execute] cd "/tmp/.tmp0CbCYw" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0CbCYw#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-05-19T09:56:03Z DEBUG collector::execute] cd "/tmp/.tmphq8iRk" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphq8iRk#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-05-19T09:56:03Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:56:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T09:56:03Z DEBUG collector::execute] cd "/tmp/.tmppJ4UbD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppJ4UbD#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:56:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:56:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-19T09:56:04Z DEBUG collector::execute] cd "/tmp/.tmppJ4UbD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppJ4UbD#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppJ4UbD/incremental-state"
[2023-05-19T09:56:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:56:04Z DEBUG collector::execute] cd "/tmp/.tmppJ4UbD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppJ4UbD#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppJ4UbD/incremental-state"
[2023-05-19T09:56:04Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:56:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:56:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:56:04Z DEBUG collector::execute] cd "/tmp/.tmpYeAlFL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYeAlFL#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:56:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:56:04Z DEBUG collector::execute] cd "/tmp/.tmpYeAlFL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYeAlFL#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYeAlFL/incremental-state"
[2023-05-19T09:56:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:56:04Z DEBUG collector::execute] cd "/tmp/.tmpYeAlFL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYeAlFL#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYeAlFL/incremental-state"
[2023-05-19T09:56:05Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:56:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:56:05Z DEBUG collector::execute] cd "/tmp/.tmpCS5CXv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCS5CXv#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:56:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-05-19T09:56:18Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:56:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T09:56:18Z DEBUG collector::execute] cd "/tmp/.tmpb1zVlX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb1zVlX#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:56:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-19T09:56:22Z DEBUG collector::execute] cd "/tmp/.tmpb1zVlX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb1zVlX#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpb1zVlX/incremental-state"
[2023-05-19T09:56:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-19T09:56:26Z DEBUG collector::execute] cd "/tmp/.tmpb1zVlX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb1zVlX#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpb1zVlX/incremental-state"
[2023-05-19T09:56:27Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmpb1zVlX"
[2023-05-19T09:56:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-05-19T09:56:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-05-19T09:56:27Z DEBUG collector::execute] cd "/tmp/.tmpb1zVlX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb1zVlX#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpb1zVlX/incremental-state"
[2023-05-19T09:56:31Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T09:56:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T09:56:31Z DEBUG collector::execute] cd "/tmp/.tmpYVw3f2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYVw3f2#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T09:56:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[RUSTC-TIMING] rustc_error_codes test:false 2.004
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
[RUSTC-TIMING] build_script_build test:false 2.880
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtNtCs2Pz8q0IAOwP_3std11collections4hash3map7HashMapNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdTINtNtNtCs9StLUOJxN7N_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtB4_4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtB4_3fmt5ErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtNtCs9StLUOJxN7N_12rustc_middle5query8plumbing12query_get_atINtNtNtCsfdl0L7kMnbW_18rustc_query_system5query6caches11SingleCacheINtNtB4_5erase6ErasedAhj10_EEECslAOZOuzpXl3_10rustc_smir Hash = 903233818342430900 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXs2_NtNtNtCsaHM24QwvQmi_4core3ops8function5implsQNCNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir30rustc_terminator_to_terminator0INtB7_6FnOnceTToNtNtCs9StLUOJxN7N_12rustc_middle3mir10BasicBlockEEE9call_onceBU_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXs2_NtNtNtCsaHM24QwvQmi_4core3ops8function5implsQNCNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtBT_6TablesNtNtBV_10stable_mir7Context15all_local_items0INtB7_6FnOnceTRNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdEE9call_onceBV_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h1df4ca5c02878462E Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h306b4600b738f0b9E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvMNtCslAOZOuzpXl3_10rustc_smir14rustc_internalNtNtB4_10rustc_smir6Tables11item_def_id Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvMNtCslAOZOuzpXl3_10rustc_smir14rustc_internalNtNtB4_10rustc_smir6Tables10crate_item Hash = 11922954461437676 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context10find_crate Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context15all_local_items Hash = 995382920162496885 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context8entry_fn Hash = 18663487788228108 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context8mir_body Hash = 668285657691964329 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context12rustc_tables Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context7ty_kind Hash = 1117982120138886448 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvMs_NtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB4_6Tables9intern_ty Hash = 287486625014882487 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir10smir_crate Hash = 828027342165498051 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir28rustc_statement_to_statement Hash = 90653978435007901 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir14rustc_op_to_op Hash = 650973719845048549 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir22rustc_bin_op_to_bin_op Hash = 393568427803840120 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir30rustc_terminator_to_terminator Hash = 471086160746802464 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs4tPnfut6gBO_10rustc_span6symbol6SymbolEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtCsg1BfsErRJAJ_9rustc_abi8FieldIdxECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtCsg1BfsErRJAJ_9rustc_abi10VariantIdxECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCs9StLUOJxN7N_12rustc_middle3mir5LocalECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRbECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeyECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRINtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 974670608791895679 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RNvXsm_NtCsaHM24QwvQmi_4core3fmtSINtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17hf357e5b001937c76E Hash = 1124680650125156080 up to 0 count discarded


warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNtCsib7qy6ttlXi_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECslAOZOuzpXl3_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyNtNtB9_5alloc6GlobalEB1s_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handlehNtNtB9_5alloc6GlobalECslAOZOuzpXl3_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs0_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVechE14grow_amortizedCslAOZOuzpXl3_10rustc_smir Hash = 515675264369571770 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdE16reserve_for_pushCslAOZOuzpXl3_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyE16reserve_for_pushCslAOZOuzpXl3_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyE11allocate_inBR_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVechE11allocate_inCslAOZOuzpXl3_10rustc_smir Hash = 1096621587513427694 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVechE16reserve_for_pushCslAOZOuzpXl3_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBQ_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBQ_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBS_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVechENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropCslAOZOuzpXl3_10rustc_smir Hash = 134732430909126014 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.14: no profile data available for function _RNvMs2_NtCshYLgFxPT9uR_8indexmap3setINtB5_8IndexSetNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE4iterCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.14: no profile data available for function _RNvXsb_NtCshYLgFxPT9uR_8indexmap3setINtB5_4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdENtNtNtNtCsaHM24QwvQmi_4core4iter6traits8iterator8Iterator4nextCslAOZOuzpXl3_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.14: no profile data available for function _RNvXsb_NtCshYLgFxPT9uR_8indexmap3setINtB5_4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdENtNtNtNtCsaHM24QwvQmi_4core4iter6traits8iterator8Iterator9size_hintCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvMs5_NtCsEZL2HUuqMG_9hashbrown3rawINtB6_8RawTableTNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBP_uINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE0ECslAOZOuzpXl3_10rustc_smir Hash = 654994090400369401 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvMs5_NtCsEZL2HUuqMG_9hashbrown3rawINtB6_8RawTableTNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBP_uINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE0ECslAOZOuzpXl3_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvMs5_NtCsEZL2HUuqMG_9hashbrown3rawINtB6_8RawTableTNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBP_uINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE0ECslAOZOuzpXl3_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsEZL2HUuqMG_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCsib7qy6ttlXi_5alloc5alloc6GlobalENCNvMs9_B1z_B1w_14prepare_resize0EECslAOZOuzpXl3_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsEZL2HUuqMG_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCsib7qy6ttlXi_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_15rehash_in_place0EECslAOZOuzpXl3_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvMs2_NtNtCs2Pz8q0IAOwP_3std6thread5localINtB6_8LocalKeyINtNtCsaHM24QwvQmi_4core4cell4CellOuEE15initialize_withNCNvMs3_B6_BF_3set0uECslAOZOuzpXl3_10rustc_smir Hash = 811594942473831488 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtCs2Pz8q0IAOwP_3std6thread5local11AccessErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvNtNtNtCsEZL2HUuqMG_9hashbrown3raw5alloc5inner8do_allocNtNtCsib7qy6ttlXi_5alloc5alloc6GlobalECslAOZOuzpXl3_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvXs8_NtCs4tPnfut6gBO_10rustc_span6def_idNtB6_5DefIdNtNtCsaHM24QwvQmi_4core4hash4Hash4hashNtCs8V3J8TV8NKA_10rustc_hash8FxHasherECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvXs8_NtCsEZL2HUuqMG_9hashbrown3setINtB6_7HashSetNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEINtNtNtNtB20_4iter6traits7collect6ExtendBN_E6extendINtNtNtB3s_8adapters6copied6CopiedINtNtNtB20_5slice4iter4IterBN_EEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvMs2_NtCsEZL2HUuqMG_9hashbrown3setINtB5_7HashSetNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE6insertCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvMs3_NtNtCs2Pz8q0IAOwP_3std6thread5localINtB5_8LocalKeyINtNtCsaHM24QwvQmi_4core4cell4CellOuEE7replaceCslAOZOuzpXl3_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvXCsEZL2HUuqMG_9hashbrownNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdINtB2_10EquivalentBp_E10equivalentCslAOZOuzpXl3_10rustc_smir Hash = 382993475055910911 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvXs3_NtNtCsaHM24QwvQmi_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslAOZOuzpXl3_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEEB1j_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEBO_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 567991000227110385 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body13AssertMessageEBO_ Hash = 647472252099359144 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body6RvalueEBO_ Hash = 1096621588281264899 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementEBO_ Hash = 844982797524240697 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvMs0_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VechE17extend_from_sliceCslAOZOuzpXl3_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumENCNvXNtBY_10rustc_smirNtB49_6TablesNtBW_7Context15external_crates0EE9from_iterBY_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtCshYLgFxPT9uR_8indexmap3set4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdENCNvXNtBY_10rustc_smirNtB4q_6TablesNtBW_7Context15all_local_items0EE9from_iterBY_ Hash = 303685787496864384 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtB2b_6copied6CopiedINtNtNtB2f_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyEENCNvMs_NtB10_10rustc_smirNtB4t_6Tables14rustc_ty_to_ty0EE9from_iterB10_ Hash = 414921810136842226 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2f_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir9LocalDeclENCNvXNtB10_10rustc_smirNtB4b_6TablesNtBY_7Context8mir_bodys_0EE9from_iterB10_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2w_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir14BasicBlockDataENCNvXNtB12_10rustc_smirNtB4y_6TablesNtB10_7Context8mir_body0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapNtNtNtCs9StLUOJxN7N_12rustc_middle3mir10terminator17SwitchTargetsIterNCNvNtB12_10rustc_smir30rustc_terminator_to_terminator0EE9from_iterB12_ Hash = 141929913395094594 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2s_5slice4iter4IterNtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax7OperandENCNvNtB12_10rustc_smir30rustc_terminator_to_terminators_0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2u_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir9StatementENvNtB12_10rustc_smir28rustc_statement_to_statementEE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyENtB5_5Debug3fmtB18_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBJ_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBJ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBL_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VechENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsp_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtCsaHM24QwvQmi_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsp_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtCsaHM24QwvQmi_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsp_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandENtNtCsaHM24QwvQmi_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsp_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementENtNtCsaHM24QwvQmi_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeQNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCs4tPnfut6gBO_10rustc_span6symbol6SymbolECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRjECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvMNtNtNtCsaHM24QwvQmi_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCslAOZOuzpXl3_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXNtCsaHM24QwvQmi_4core3fmtQNtNtCsib7qy6ttlXi_5alloc6string6StringNtB2_5Write10write_charCslAOZOuzpXl3_10rustc_smir Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXNtCsaHM24QwvQmi_4core3fmtQNtNtCsib7qy6ttlXi_5alloc6string6StringNtB2_5Write9write_fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXNtCsaHM24QwvQmi_4core3fmtQNtNtCsib7qy6ttlXi_5alloc6string6StringNtB2_5Write9write_strCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsM_NtCsaHM24QwvQmi_4core6optionINtB5_6OptionNtNtCs4tPnfut6gBO_10rustc_span6symbol6SymbolENtNtB7_3fmt5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsM_NtCsaHM24QwvQmi_4core6optionINtB5_6OptionjENtNtB7_3fmt5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCsib7qy6ttlXi_5alloc6string6StringNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyNtB5_5Debug3fmtBB_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsm_NtCsaHM24QwvQmi_4core3fmtSNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyNtB5_5Debug3fmtBB_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvYNtNtCsib7qy6ttlXi_5alloc6string6StringNtNtCsaHM24QwvQmi_4core3fmt5Write9write_fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvMNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2tyNtB2_2Ty4kind Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXs1_NtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2tyNtB5_2TyNtNtCsaHM24QwvQmi_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCs9StLUOJxN7N_12rustc_middle9dep_graph8dep_node7DepKindEEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvXNtCs9StLUOJxN7N_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECslAOZOuzpXl3_10rustc_smir Hash = 816997251081395164 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateNCNvXNtB3x_10rustc_smirNtB4k_6TablesNtB3v_7Context10find_crate0E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 629592967359852085 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvYINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvMs0_CshYLgFxPT9uR_8indexmapINtB5_6BucketNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIduE7key_refCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRINtNtNtCs9StLUOJxN7N_12rustc_middle2ty4list4ListINtNtNtBC_3mir6syntax14ProjectionElemNtB1m_5LocalNtBA_2TyEENtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtCsg1BfsErRJAJ_9rustc_abi10VariantIdxNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCs9StLUOJxN7N_12rustc_middle3mir5LocalNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsx_NtCsib7qy6ttlXi_5alloc5boxedINtB5_3BoxNtNtCs9StLUOJxN7N_12rustc_middle3mir8ConstantENtNtCsaHM24QwvQmi_4core3fmt7Display3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.9: no profile data available for function _RINvMsz_NtCsEZL2HUuqMG_9hashbrown3mapINtB6_15RawEntryBuilderNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdTINtNtNtCs9StLUOJxN7N_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE6searchNCINvB6_10equivalentBW_BW_E0ECslAOZOuzpXl3_10rustc_smir Hash = 910108394528351925 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.9: no profile data available for function _RINvXs1x_NtCsEZL2HUuqMG_9hashbrown3mapINtB7_7HashMapNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEINtNtNtNtB22_4iter6traits7collect6ExtendTBO_uEE6extendINtNtNtB3u_8adapters3map3MapINtNtB4k_6copied6CopiedINtNtNtB22_5slice4iter4IterBO_EENCINvXs8_NtB9_3setINtB5J_7HashSetBO_B1X_EIB3o_BO_E6extendB4H_E0EECslAOZOuzpXl3_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.9: no profile data available for function _RNvMs1_NtCsEZL2HUuqMG_9hashbrown3mapINtB5_7HashMapNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE6insertCslAOZOuzpXl3_10rustc_smir Hash = 11922956408974369 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters6copied9copy_foldNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyuNCINvNtBN_3map8map_foldB1p_NtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyuNCNvMs_NtB2z_10rustc_smirNtB3p_6Tables14rustc_ty_to_ty0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2t_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5h_3VecB2t_E14extend_trustedINtB27_3MapINtBL_6CopiedINtNtNtB4_5slice4iter4IterB1p_EEB3i_EE0E0E0E0EB2z_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_NtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyuNCNvMs_NtB3h_10rustc_smirNtB47_6Tables14rustc_ty_to_ty0NCINvNvB26_8for_each4callB3b_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5v_3VecB3b_E14extend_trustedINtB2P_3MapBP_B40_EE0E0E0EB3h_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCsEZL2HUuqMG_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7c_7HashMapB1s_uB4K_EIB5V_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECslAOZOuzpXl3_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvXsv_NtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCsaHM24QwvQmi_4core4hash4Hash4hashNtCs8V3J8TV8NKA_10rustc_hash8FxHasherECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXCsEZL2HUuqMG_9hashbrownNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_10EquivalentBp_E10equivalentCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXs1_NtNtNtCsaHM24QwvQmi_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateNCNvXNtB2E_10rustc_smirNtB3r_6TablesNtB2C_7Context10find_crate0E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 505312707234972298 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCs4tPnfut6gBO_10rustc_span6symbol6SymbolNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRbNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRjNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyEENtNtNtB8_6traits8iterator8Iterator9size_hintCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsu_NtCs2ZKLbr2TgcK_12tracing_core5fieldINtB5_10DebugValueRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumENtB5_5Value6recordCslAOZOuzpXl3_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsu_NtCs2ZKLbr2TgcK_12tracing_core5fieldINtB5_10DebugValueRNtNtCsib7qy6ttlXi_5alloc6string6StringENtB5_5Value6recordCslAOZOuzpXl3_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11item_def_id Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal10crate_item Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal9crate_num Hash = 784007058953177093 up to 0 count discarded
[RUSTC-TIMING] rustc_traits test:false 84.550
[RUSTC-TIMING] rustc_traits test:false 84.550
warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB16_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEEB1j_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEEB1q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtBP_10stable_mir9CrateItemNCNvBN_10crate_item0E00EBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdNCNvBN_11item_def_id0E00EBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty6TyKindEBM_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body4BodyEBO_ Hash = 1096621589765811986 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRbECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRjECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placejECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNSNvYNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtBd_10stable_mir9CrateItemNCNvBb_10crate_item0E00INtNtNtCsaHM24QwvQmi_4core3ops8function6FnOnceTQNtNtBd_10rustc_smir6TablesEE9call_once6vtableBd_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNSNvYNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdNCNvBb_11item_def_id0E00INtNtNtCsaHM24QwvQmi_4core3ops8function6FnOnceTQNtNtBd_10rustc_smir6TablesEE9call_once6vtableBd_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h50f1ae1f583ddd66E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h99dc71a8495f20f8E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtB8_10stable_mir9CrateItemNCNvB6_10crate_item0E00B8_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdNCNvB6_11item_def_id0E00B8_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvMNtCslAOZOuzpXl3_10rustc_smir10stable_mirNtB2_9CrateItem4body Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir8entry_fn Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir11local_crate Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir10find_crate Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir15external_crates Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir15all_local_items Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCslAOZOuzpXl3_10rustc_smir10stable_mir4withNtNtB2_2ty6TyKindNCNvMBN_NtBN_2Ty4kind0EB4_ Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCslAOZOuzpXl3_10rustc_smir10stable_mir4withuNCINvNtB4_14rustc_internal11with_tablesNtB2_9CrateItemNCNvBR_10crate_item0E0EB4_ Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCslAOZOuzpXl3_10rustc_smir10stable_mir4withuNCINvNtB4_14rustc_internal11with_tablesNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdNCNvBR_11item_def_id0E0EB4_ Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvXs4_NtCslAOZOuzpXl3_10rustc_smir10stable_mirNtB5_5CrateNtNtCsaHM24QwvQmi_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvXsa_NtCslAOZOuzpXl3_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCsaHM24QwvQmi_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNvNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir3TLV7___getit7destroy Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateuNCNvXNtB2a_10rustc_smirNtB2Y_6TablesNtB28_7Context15external_crates0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB55_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs9StLUOJxN7N_12rustc_middle3mir14BasicBlockDataNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockuNCNvXNtB2k_10rustc_smirNtB3n_6TablesNtB2i_7Context8mir_body0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2c_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5m_3VecB2c_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3i_EE0E0E0EB2k_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs9StLUOJxN7N_12rustc_middle3mir9LocalDeclNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyuNCNvXNtB2c_10rustc_smirNtB30_6TablesNtB2a_7Context8mir_bodys_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB51_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2V_EE0E0E0EB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs9StLUOJxN7N_12rustc_middle3mir9StatementNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementuNvNtB2e_10rustc_smir28rustc_statement_to_statementNCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB54_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3a_EE0E0E0EB2e_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax7OperandNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperanduNCNvNtB2l_10rustc_smir30rustc_terminator_to_terminators_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2d_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5g_3VecB2d_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3f_EE0E0E0EB2l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 567991000227110385 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body13AssertMessageEBO_ Hash = 647472252099359144 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyEENCNvMs_NtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2u_6Tables14rustc_ty_to_ty0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3D_8for_each4callNtNtNtB2w_10stable_mir2ty2TyNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5h_3VecB4G_E14extend_trustedBN_E0E0EB2w_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsEZL2HUuqMG_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB53_8iterator8Iterator4folduNCINvNvB5M_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Y_7HashMapB1J_uB3O_EIB4Z_B6H_E6extendBN_E0E0ECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumENCNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2d_6TablesNtNtB2f_10stable_mir7Context15external_crates0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3P_8for_each4callNtB36_5CrateNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5d_3VecB4S_E14extend_trustedBN_E0E0EB2f_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir14BasicBlockDataENCNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2j_6TablesNtNtB2l_10stable_mir7Context8mir_body0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3N_8for_each4callNtNtNtB3c_3mir4body10BasicBlockNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5u_3VecB4Q_E14extend_trustedBN_E0E0EB2l_ Hash = 650973720849546769 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir9LocalDeclENCNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2d_6TablesNtNtB2f_10stable_mir7Context8mir_bodys_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3J_8for_each4callNtNtB36_2ty2TyNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB59_3VecB4M_E14extend_trustedBN_E0E0EB2f_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir9StatementENvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir28rustc_statement_to_statementENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3j_8for_each4callNtNtNtNtB2c_10stable_mir3mir4body9StatementNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5c_3VecB4m_E14extend_trustedBN_E0E0EB2c_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax7OperandENCNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir30rustc_terminator_to_terminators_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3x_8for_each4callNtNtNtNtB2l_10stable_mir3mir4body7OperandNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5o_3VecB4A_E14extend_trustedBN_E0E0EB2l_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRINtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax14ProjectionElemNtB19_5LocalNtNtB1b_2ty2TyEINtNtNtBa_5slice4iter4IterB14_EECslAOZOuzpXl3_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyINtNtNtBa_5slice4iter4IterB14_EEB1a_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded
---
Preparing cargo-0.60.0
[2023-05-19T10:34:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T10:34:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T10:34:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T10:34:17Z DEBUG collector::execute] cd "/tmp/.tmpzPpOar" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzPpOar#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-05-19T10:34:17Z DEBUG collector::execute] cd "/tmp/.tmpfcxgFW" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfcxgFW#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-05-19T10:34:17Z DEBUG collector::execute] cd "/tmp/.tmpHuAx3r" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHuAx3r#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-05-19T10:36:27Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:36:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T10:36:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T10:36:28Z DEBUG collector::execute] cd "/tmp/.tmpBsyhmi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBsyhmi#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T10:36:34Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:36:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T10:36:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T10:36:34Z DEBUG collector::execute] cd "/tmp/.tmpwr7hQB" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwr7hQB#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T10:36:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:36:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T10:36:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T10:36:59Z DEBUG collector::execute] cd "/tmp/.tmpzESYqu" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzESYqu#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/8)
Preparing clap-3.1.6
[2023-05-19T10:38:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T10:38:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T10:38:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T10:38:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T10:38:05Z DEBUG collector::execute] cd "/tmp/.tmpHhY1Yb" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHhY1Yb#clap@3.1.6" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-19T10:38:05Z DEBUG collector::execute] cd "/tmp/.tmpR3DO7X" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR3DO7X#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-05-19T10:38:05Z DEBUG collector::execute] cd "/tmp/.tmpjPHJ21" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjPHJ21#clap@3.1.6" "--" "--skip-this-rustc"
Running clap-3.1.6: Check + [Full]
[2023-05-19T10:38:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:38:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T10:38:17Z DEBUG collector::execute] cd "/tmp/.tmpcaF5SL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcaF5SL#clap@3.1.6" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T10:38:19Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:38:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T10:38:19Z DEBUG collector::execute] cd "/tmp/.tmpdE1xQL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdE1xQL#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
---
Preparing hyper-0.14.18
[2023-05-19T10:38:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T10:38:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T10:38:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T10:38:35Z DEBUG collector::execute] cd "/tmp/.tmpb2i0GR" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb2i0GR#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-19T10:38:35Z DEBUG collector::execute] cd "/tmp/.tmpPjFw25" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPjFw25#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-19T10:38:35Z DEBUG collector::execute] cd "/tmp/.tmpAXqkdN" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAXqkdN#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-19T10:39:24Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:39:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T10:39:24Z DEBUG collector::execute] cd "/tmp/.tmpJsctr0" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJsctr0#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Debug + [Full]
Running hyper-0.14.18: Debug + [Full]
[2023-05-19T10:39:26Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:39:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T10:39:26Z DEBUG collector::execute] cd "/tmp/.tmpCrE3mo" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCrE3mo#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
[2023-05-19T10:39:31Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:39:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T10:39:31Z DEBUG collector::execute] cd "/tmp/.tmpNwqIad" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNwqIad#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-05-19T10:39:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T10:39:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-05-19T10:39:58Z DEBUG collector::execute] cd "/tmp/.tmpn6c0RJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpn6c0RJ#regex@1.5.5" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Debug + [Full]
[2023-05-19T10:39:58Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:39:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T10:39:59Z DEBUG collector::execute] cd "/tmp/.tmpiwmEzW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiwmEzW#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T10:40:02Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:40:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T10:40:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T10:40:02Z DEBUG collector::execute] cd "/tmp/.tmpGAQEP0" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGAQEP0#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/8)
Preparing ripgrep-13.0.0
[2023-05-19T10:40:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T10:40:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T10:40:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T10:40:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T10:40:16Z DEBUG collector::execute] cd "/tmp/.tmpKfwM7G" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKfwM7G#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-05-19T10:40:16Z DEBUG collector::execute] cd "/tmp/.tmpa9AWTQ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpa9AWTQ#ripgrep@13.0.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-19T10:40:16Z DEBUG collector::execute] cd "/tmp/.tmpgttn7K" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgttn7K#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-05-19T10:41:23Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:41:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T10:41:23Z DEBUG collector::execute] cd "/tmp/.tmp3W5vHw" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3W5vHw#ripgrep@13.0.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Debug + [Full]
---
[2023-05-19T10:43:09Z DEBUG collector::execute] cd "/tmp/.tmpfPHcc8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfPHcc8#serde@1.0.136" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Debug + [Full]
[2023-05-19T10:43:11Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:43:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T10:43:11Z DEBUG collector::execute] cd "/tmp/.tmpxofYFS" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxofYFS#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T10:43:15Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:43:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T10:43:15Z DEBUG collector::execute] cd "/tmp/.tmppddMQE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppddMQE#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark serde-1.0.136 (7/8)
Finished benchmark serde-1.0.136 (7/8)
Executing benchmark syn-1.0.89 (8/8)
Preparing syn-1.0.89
[2023-05-19T10:43:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-19T10:43:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-19T10:43:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-19T10:43:21Z DEBUG collector::execute] cd "/tmp/.tmpnOXP76" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnOXP76#syn@1.0.89" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-19T10:43:21Z DEBUG collector::execute] cd "/tmp/.tmptGVakH" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptGVakH#syn@1.0.89" "--" "--skip-this-rustc"
[2023-05-19T10:43:21Z DEBUG collector::execute] cd "/tmp/.tmpczWdVy" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpczWdVy#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-05-19T10:43:31Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:43:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T10:43:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-19T10:43:31Z DEBUG collector::execute] cd "/tmp/.tmpXSXwNE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXSXwNE#syn@1.0.89" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T10:43:32Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:43:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T10:43:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-19T10:43:32Z DEBUG collector::execute] cd "/tmp/.tmp1NbyBS" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1NbyBS#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-19T10:43:36Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-19T10:43:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-19T10:43:36Z DEBUG collector::execute] cd "/tmp/.tmpDC0lSd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDC0lSd#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (8/8)
---
[RUSTC-TIMING] rustc_symbol_mangling test:false 20.647
[RUSTC-TIMING] rustc_ast_lowering test:false 62.589
[RUSTC-TIMING] rustc_query_impl test:false 179.890
[RUSTC-TIMING] rustc_monomorphize test:false 28.304
warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtNtCs2Pz8q0IAOwP_3std11collections4hash3map7HashMapNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdTINtNtNtCs9StLUOJxN7N_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtB4_4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtB4_3fmt5ErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RINvNtNtCs9StLUOJxN7N_12rustc_middle5query8plumbing12query_get_atINtNtNtCsfdl0L7kMnbW_18rustc_query_system5query6caches11SingleCacheINtNtB4_5erase6ErasedAhj10_EEECslAOZOuzpXl3_10rustc_smir Hash = 903233818342430900 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXs2_NtNtNtCsaHM24QwvQmi_4core3ops8function5implsQNCNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir30rustc_terminator_to_terminator0INtB7_6FnOnceTToNtNtCs9StLUOJxN7N_12rustc_middle3mir10BasicBlockEEE9call_onceBU_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXs2_NtNtNtCsaHM24QwvQmi_4core3ops8function5implsQNCNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtBT_6TablesNtNtBV_10stable_mir7Context15all_local_items0INtB7_6FnOnceTRNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdEE9call_onceBV_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h1df4ca5c02878462E Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h306b4600b738f0b9E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvMNtCslAOZOuzpXl3_10rustc_smir14rustc_internalNtNtB4_10rustc_smir6Tables11item_def_id Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvMNtCslAOZOuzpXl3_10rustc_smir14rustc_internalNtNtB4_10rustc_smir6Tables10crate_item Hash = 11922954461437676 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context10find_crate Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context15all_local_items Hash = 995382920162496885 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context8entry_fn Hash = 18663487788228108 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context8mir_body Hash = 668285657691964329 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context12rustc_tables Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context7ty_kind Hash = 1117982120138886448 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvMs_NtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB4_6Tables9intern_ty Hash = 287486625014882487 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir10smir_crate Hash = 828027342165498051 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir28rustc_statement_to_statement Hash = 90653978435007901 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir14rustc_op_to_op Hash = 650973719845048549 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir22rustc_bin_op_to_bin_op Hash = 393568427803840120 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.2: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir30rustc_terminator_to_terminator Hash = 471086160746802464 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs4tPnfut6gBO_10rustc_span6symbol6SymbolEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtCsg1BfsErRJAJ_9rustc_abi8FieldIdxECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtCsg1BfsErRJAJ_9rustc_abi10VariantIdxECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCs9StLUOJxN7N_12rustc_middle3mir5LocalECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRbECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeyECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRINtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 974670608791895679 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _RNvXsm_NtCsaHM24QwvQmi_4core3fmtSINtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.fb84848127419faf-cgu.10: no profile data available for function _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17hf357e5b001937c76E Hash = 1124680650125156080 up to 0 count discarded


warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNtCsib7qy6ttlXi_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECslAOZOuzpXl3_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyNtNtB9_5alloc6GlobalEB1s_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RINvNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handlehNtNtB9_5alloc6GlobalECslAOZOuzpXl3_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs0_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVechE14grow_amortizedCslAOZOuzpXl3_10rustc_smir Hash = 515675264369571770 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdE16reserve_for_pushCslAOZOuzpXl3_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyE16reserve_for_pushCslAOZOuzpXl3_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyE11allocate_inBR_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVechE11allocate_inCslAOZOuzpXl3_10rustc_smir Hash = 1096621587513427694 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvMs_NtCsib7qy6ttlXi_5alloc7raw_vecINtB4_6RawVechE16reserve_for_pushCslAOZOuzpXl3_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBQ_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBQ_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBS_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.0: no profile data available for function _RNvXs1_NtCsib7qy6ttlXi_5alloc7raw_vecINtB5_6RawVechENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropCslAOZOuzpXl3_10rustc_smir Hash = 134732430909126014 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.14: no profile data available for function _RNvMs2_NtCshYLgFxPT9uR_8indexmap3setINtB5_8IndexSetNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE4iterCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.14: no profile data available for function _RNvXsb_NtCshYLgFxPT9uR_8indexmap3setINtB5_4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdENtNtNtNtCsaHM24QwvQmi_4core4iter6traits8iterator8Iterator4nextCslAOZOuzpXl3_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.14: no profile data available for function _RNvXsb_NtCshYLgFxPT9uR_8indexmap3setINtB5_4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdENtNtNtNtCsaHM24QwvQmi_4core4iter6traits8iterator8Iterator9size_hintCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvMs5_NtCsEZL2HUuqMG_9hashbrown3rawINtB6_8RawTableTNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBP_uINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE0ECslAOZOuzpXl3_10rustc_smir Hash = 654994090400369401 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvMs5_NtCsEZL2HUuqMG_9hashbrown3rawINtB6_8RawTableTNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBP_uINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE0ECslAOZOuzpXl3_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvMs5_NtCsEZL2HUuqMG_9hashbrown3rawINtB6_8RawTableTNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBP_uINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE0ECslAOZOuzpXl3_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsEZL2HUuqMG_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCsib7qy6ttlXi_5alloc5alloc6GlobalENCNvMs9_B1z_B1w_14prepare_resize0EECslAOZOuzpXl3_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.5: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsEZL2HUuqMG_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCsib7qy6ttlXi_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_15rehash_in_place0EECslAOZOuzpXl3_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvMs2_NtNtCs2Pz8q0IAOwP_3std6thread5localINtB6_8LocalKeyINtNtCsaHM24QwvQmi_4core4cell4CellOuEE15initialize_withNCNvMs3_B6_BF_3set0uECslAOZOuzpXl3_10rustc_smir Hash = 811594942473831488 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtCs2Pz8q0IAOwP_3std6thread5local11AccessErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvNtNtNtCsEZL2HUuqMG_9hashbrown3raw5alloc5inner8do_allocNtNtCsib7qy6ttlXi_5alloc5alloc6GlobalECslAOZOuzpXl3_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvXs8_NtCs4tPnfut6gBO_10rustc_span6def_idNtB6_5DefIdNtNtCsaHM24QwvQmi_4core4hash4Hash4hashNtCs8V3J8TV8NKA_10rustc_hash8FxHasherECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RINvXs8_NtCsEZL2HUuqMG_9hashbrown3setINtB6_7HashSetNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEINtNtNtNtB20_4iter6traits7collect6ExtendBN_E6extendINtNtNtB3s_8adapters6copied6CopiedINtNtNtB20_5slice4iter4IterBN_EEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvMs2_NtCsEZL2HUuqMG_9hashbrown3setINtB5_7HashSetNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE6insertCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvMs3_NtNtCs2Pz8q0IAOwP_3std6thread5localINtB5_8LocalKeyINtNtCsaHM24QwvQmi_4core4cell4CellOuEE7replaceCslAOZOuzpXl3_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvXCsEZL2HUuqMG_9hashbrownNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdINtB2_10EquivalentBp_E10equivalentCslAOZOuzpXl3_10rustc_smir Hash = 382993475055910911 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvXs3_NtNtCsaHM24QwvQmi_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslAOZOuzpXl3_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.15: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEEB1j_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEBO_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 567991000227110385 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body13AssertMessageEBO_ Hash = 647472252099359144 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body6RvalueEBO_ Hash = 1096621588281264899 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementEBO_ Hash = 844982797524240697 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvMs0_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VechE17extend_from_sliceCslAOZOuzpXl3_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumENCNvXNtBY_10rustc_smirNtB49_6TablesNtBW_7Context15external_crates0EE9from_iterBY_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtCshYLgFxPT9uR_8indexmap3set4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIdENCNvXNtBY_10rustc_smirNtB4q_6TablesNtBW_7Context15all_local_items0EE9from_iterBY_ Hash = 303685787496864384 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtB2b_6copied6CopiedINtNtNtB2f_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyEENCNvMs_NtB10_10rustc_smirNtB4t_6Tables14rustc_ty_to_ty0EE9from_iterB10_ Hash = 414921810136842226 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2f_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir9LocalDeclENCNvXNtB10_10rustc_smirNtB4b_6TablesNtBY_7Context8mir_bodys_0EE9from_iterB10_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2w_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir14BasicBlockDataENCNvXNtB12_10rustc_smirNtB4y_6TablesNtB10_7Context8mir_body0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapNtNtNtCs9StLUOJxN7N_12rustc_middle3mir10terminator17SwitchTargetsIterNCNvNtB12_10rustc_smir30rustc_terminator_to_terminator0EE9from_iterB12_ Hash = 141929913395094594 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2s_5slice4iter4IterNtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax7OperandENCNvNtB12_10rustc_smir30rustc_terminator_to_terminators_0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXNtNtCsib7qy6ttlXi_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementEINtB2_12SpecFromIterBU_INtNtNtNtCsaHM24QwvQmi_4core4iter8adapters3map3MapINtNtNtB2u_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir9StatementENvNtB12_10rustc_smir28rustc_statement_to_statementEE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyENtB5_5Debug3fmtB18_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBJ_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBJ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBL_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsn_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VechENtNtNtCsaHM24QwvQmi_4core3ops4drop4Drop4dropCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsp_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtCsaHM24QwvQmi_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsp_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtCsaHM24QwvQmi_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsp_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandENtNtCsaHM24QwvQmi_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.1: no profile data available for function _RNvXsp_NtCsib7qy6ttlXi_5alloc3vecINtB5_3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementENtNtCsaHM24QwvQmi_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeQNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCs4tPnfut6gBO_10rustc_span6symbol6SymbolECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRjECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvMNtNtNtCsaHM24QwvQmi_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCslAOZOuzpXl3_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXNtCsaHM24QwvQmi_4core3fmtQNtNtCsib7qy6ttlXi_5alloc6string6StringNtB2_5Write10write_charCslAOZOuzpXl3_10rustc_smir Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXNtCsaHM24QwvQmi_4core3fmtQNtNtCsib7qy6ttlXi_5alloc6string6StringNtB2_5Write9write_fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXNtCsaHM24QwvQmi_4core3fmtQNtNtCsib7qy6ttlXi_5alloc6string6StringNtB2_5Write9write_strCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsM_NtCsaHM24QwvQmi_4core6optionINtB5_6OptionNtNtCs4tPnfut6gBO_10rustc_span6symbol6SymbolENtNtB7_3fmt5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsM_NtCsaHM24QwvQmi_4core6optionINtB5_6OptionjENtNtB7_3fmt5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCsib7qy6ttlXi_5alloc6string6StringNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyNtB5_5Debug3fmtBB_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXsm_NtCsaHM24QwvQmi_4core3fmtSNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyNtB5_5Debug3fmtBB_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvYNtNtCsib7qy6ttlXi_5alloc6string6StringNtNtCsaHM24QwvQmi_4core3fmt5Write9write_fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvMNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2tyNtB2_2Ty4kind Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.13: no profile data available for function _RNvXs1_NtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2tyNtB5_2TyNtNtCsaHM24QwvQmi_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCs9StLUOJxN7N_12rustc_middle9dep_graph8dep_node7DepKindEEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvXNtCs9StLUOJxN7N_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECslAOZOuzpXl3_10rustc_smir Hash = 816997251081395164 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateNCNvXNtB3x_10rustc_smirNtB4k_6TablesNtB3v_7Context10find_crate0E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 629592967359852085 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RINvYINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvMs0_CshYLgFxPT9uR_8indexmapINtB5_6BucketNtNtCs4tPnfut6gBO_10rustc_span6def_id10LocalDefIduE7key_refCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRINtNtNtCs9StLUOJxN7N_12rustc_middle2ty4list4ListINtNtNtBC_3mir6syntax14ProjectionElemNtB1m_5LocalNtBA_2TyEENtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtCsg1BfsErRJAJ_9rustc_abi10VariantIdxNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCs9StLUOJxN7N_12rustc_middle3mir5LocalNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.11: no profile data available for function _RNvXsx_NtCsib7qy6ttlXi_5alloc5boxedINtB5_3BoxNtNtCs9StLUOJxN7N_12rustc_middle3mir8ConstantENtNtCsaHM24QwvQmi_4core3fmt7Display3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.9: no profile data available for function _RINvMsz_NtCsEZL2HUuqMG_9hashbrown3mapINtB6_15RawEntryBuilderNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdTINtNtNtCs9StLUOJxN7N_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE6searchNCINvB6_10equivalentBW_BW_E0ECslAOZOuzpXl3_10rustc_smir Hash = 910108394528351925 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.9: no profile data available for function _RINvXs1x_NtCsEZL2HUuqMG_9hashbrown3mapINtB7_7HashMapNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEINtNtNtNtB22_4iter6traits7collect6ExtendTBO_uEE6extendINtNtNtB3u_8adapters3map3MapINtNtB4k_6copied6CopiedINtNtNtB22_5slice4iter4IterBO_EENCINvXs8_NtB9_3setINtB5J_7HashSetBO_B1X_EIB3o_BO_E6extendB4H_E0EECslAOZOuzpXl3_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.9: no profile data available for function _RNvMs1_NtCsEZL2HUuqMG_9hashbrown3mapINtB5_7HashMapNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsaHM24QwvQmi_4core4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEE6insertCslAOZOuzpXl3_10rustc_smir Hash = 11922956408974369 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters6copied9copy_foldNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyuNCINvNtBN_3map8map_foldB1p_NtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyuNCNvMs_NtB2z_10rustc_smirNtB3p_6Tables14rustc_ty_to_ty0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2t_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5h_3VecB2t_E14extend_trustedINtB27_3MapINtBL_6CopiedINtNtNtB4_5slice4iter4IterB1p_EEB3i_EE0E0E0E0EB2z_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_NtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyuNCNvMs_NtB3h_10rustc_smirNtB47_6Tables14rustc_ty_to_ty0NCINvNvB26_8for_each4callB3b_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5v_3VecB3b_E14extend_trustedINtB2P_3MapBP_B40_EE0E0E0EB3h_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCsEZL2HUuqMG_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7c_7HashMapB1s_uB4K_EIB5V_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECslAOZOuzpXl3_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RINvXsv_NtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCsaHM24QwvQmi_4core4hash4Hash4hashNtCs8V3J8TV8NKA_10rustc_hash8FxHasherECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXCsEZL2HUuqMG_9hashbrownNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_10EquivalentBp_E10equivalentCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXs1_NtNtNtCsaHM24QwvQmi_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateNCNvXNtB2E_10rustc_smirNtB3r_6TablesNtB2C_7Context10find_crate0E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 505312707234972298 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRNtNtCs4tPnfut6gBO_10rustc_span6symbol6SymbolNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRbNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsP_NtCsaHM24QwvQmi_4core3fmtRjNtB5_5Debug3fmtCslAOZOuzpXl3_10rustc_smir Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyEENtNtNtB8_6traits8iterator8Iterator9size_hintCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXs_NtNtNtCsaHM24QwvQmi_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsu_NtCs2ZKLbr2TgcK_12tracing_core5fieldINtB5_10DebugValueRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumENtB5_5Value6recordCslAOZOuzpXl3_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvXsu_NtCs2ZKLbr2TgcK_12tracing_core5fieldINtB5_10DebugValueRNtNtCsib7qy6ttlXi_5alloc6string6StringENtB5_5Value6recordCslAOZOuzpXl3_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11item_def_id Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal10crate_item Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.8: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal9crate_num Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB16_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEEB1j_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir9CrateItemEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyEEB1q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtBP_10stable_mir9CrateItemNCNvBN_10crate_item0E00EBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdNCNvBN_11item_def_id0E00EBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty6TyKindEBM_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body4BodyEBO_ Hash = 1096621589765811986 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRbECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeRjECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placejECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNSNvYNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtBd_10stable_mir9CrateItemNCNvBb_10crate_item0E00INtNtNtCsaHM24QwvQmi_4core3ops8function6FnOnceTQNtNtBd_10rustc_smir6TablesEE9call_once6vtableBd_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNSNvYNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdNCNvBb_11item_def_id0E00INtNtNtCsaHM24QwvQmi_4core3ops8function6FnOnceTQNtNtBd_10rustc_smir6TablesEE9call_once6vtableBd_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h50f1ae1f583ddd66E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h99dc71a8495f20f8E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtB8_10stable_mir9CrateItemNCNvB6_10crate_item0E00B8_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNCNCINvNtCslAOZOuzpXl3_10rustc_smir14rustc_internal11with_tablesNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdNCNvB6_11item_def_id0E00B8_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvMNtCslAOZOuzpXl3_10rustc_smir10stable_mirNtB2_9CrateItem4body Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir8entry_fn Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir11local_crate Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir10find_crate Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir15external_crates Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir15all_local_items Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCslAOZOuzpXl3_10rustc_smir10stable_mir4withNtNtB2_2ty6TyKindNCNvMBN_NtBN_2Ty4kind0EB4_ Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCslAOZOuzpXl3_10rustc_smir10stable_mir4withuNCINvNtB4_14rustc_internal11with_tablesNtB2_9CrateItemNCNvBR_10crate_item0E0EB4_ Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RINvNtCslAOZOuzpXl3_10rustc_smir10stable_mir4withuNCINvNtB4_14rustc_internal11with_tablesNtNtCs4tPnfut6gBO_10rustc_span6def_id5DefIdNCNvBR_11item_def_id0E0EB4_ Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvXs4_NtCslAOZOuzpXl3_10rustc_smir10stable_mirNtB5_5CrateNtNtCsaHM24QwvQmi_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvXsa_NtCslAOZOuzpXl3_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCsaHM24QwvQmi_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.6: no profile data available for function _RNvNvNvNtCslAOZOuzpXl3_10rustc_smir10stable_mir3TLV7___getit7destroy Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc3vec3VecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVecNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeINtNtCsib7qy6ttlXi_5alloc7raw_vec6RawVechEECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir5CrateuNCNvXNtB2a_10rustc_smirNtB2Y_6TablesNtB28_7Context15external_crates0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB55_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs9StLUOJxN7N_12rustc_middle3mir14BasicBlockDataNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockuNCNvXNtB2k_10rustc_smirNtB3n_6TablesNtB2i_7Context8mir_body0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2c_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5m_3VecB2c_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3i_EE0E0E0EB2k_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs9StLUOJxN7N_12rustc_middle3mir9LocalDeclNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyuNCNvXNtB2c_10rustc_smirNtB30_6TablesNtB2a_7Context8mir_bodys_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB51_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2V_EE0E0E0EB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs9StLUOJxN7N_12rustc_middle3mir9StatementNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body9StatementuNvNtB2e_10rustc_smir28rustc_statement_to_statementNCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB54_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3a_EE0E0E0EB2e_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax7OperandNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperanduNCNvNtB2l_10rustc_smir30rustc_terminator_to_terminators_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2d_NCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5g_3VecB2d_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3f_EE0E0E0EB2l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtCsib7qy6ttlXi_5alloc6string6StringECslAOZOuzpXl3_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 567991000227110385 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body13AssertMessageEBO_ Hash = 647472252099359144 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvNtCsaHM24QwvQmi_4core3ptr13drop_in_placeNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle2ty2TyEENCNvMs_NtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2u_6Tables14rustc_ty_to_ty0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3D_8for_each4callNtNtNtB2w_10stable_mir2ty2TyNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5h_3VecB4G_E14extend_trustedBN_E0E0EB2w_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCsfdl0L7kMnbW_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsEZL2HUuqMG_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCs8V3J8TV8NKA_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB53_8iterator8Iterator4folduNCINvNvB5M_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Y_7HashMapB1J_uB3O_EIB4Z_B6H_E6extendBN_E0E0ECslAOZOuzpXl3_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs4tPnfut6gBO_10rustc_span6def_id8CrateNumENCNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2d_6TablesNtNtB2f_10stable_mir7Context15external_crates0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3P_8for_each4callNtB36_5CrateNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5d_3VecB4S_E14extend_trustedBN_E0E0EB2f_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir14BasicBlockDataENCNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2j_6TablesNtNtB2l_10stable_mir7Context8mir_body0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3N_8for_each4callNtNtNtB3c_3mir4body10BasicBlockNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5u_3VecB4Q_E14extend_trustedBN_E0E0EB2l_ Hash = 650973720849546769 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir9LocalDeclENCNvXNtCslAOZOuzpXl3_10rustc_smir10rustc_smirNtB2d_6TablesNtNtB2f_10stable_mir7Context8mir_bodys_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3J_8for_each4callNtNtB36_2ty2TyNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB59_3VecB4M_E14extend_trustedBN_E0E0EB2f_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs9StLUOJxN7N_12rustc_middle3mir9StatementENvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir28rustc_statement_to_statementENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3j_8for_each4callNtNtNtNtB2c_10stable_mir3mir4body9StatementNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5c_3VecB4m_E14extend_trustedBN_E0E0EB2c_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.12: no profile data available for function _RINvXs0_NtNtNtCsaHM24QwvQmi_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax7OperandENCNvNtCslAOZOuzpXl3_10rustc_smir10rustc_smir30rustc_terminator_to_terminators_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3x_8for_each4callNtNtNtNtB2l_10stable_mir3mir4body7OperandNCINvMsi_NtCsib7qy6ttlXi_5alloc3vecINtB5o_3VecB4A_E14extend_trustedBN_E0E0EB2l_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRINtNtNtCs9StLUOJxN7N_12rustc_middle3mir6syntax14ProjectionElemNtB19_5LocalNtNtB1b_2ty2TyEINtNtNtBa_5slice4iter4IterB14_EECslAOZOuzpXl3_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir2ty2TyINtNtNtBa_5slice4iter4IterB14_EEB1a_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body10BasicBlockINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body12SwitchTargetINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.fb84848127419faf-cgu.7: no profile data available for function _RINvMs5_NtNtCsaHM24QwvQmi_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCslAOZOuzpXl3_10rustc_smir10stable_mir3mir4body7OperandINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

---
  --> compiler/rustc_borrowck/src/places_conflict.rs:16:5
   |
16 |   //! 