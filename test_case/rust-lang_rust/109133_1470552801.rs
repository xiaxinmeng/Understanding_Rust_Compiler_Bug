plain
Executing benchmark cargo-0.60.0 (1/7)
Preparing cargo-0.60.0
[2023-03-15T15:55:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T15:55:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T15:55:59Z DEBUG collector::execute] cd "/tmp/.tmpCvW9bM" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCvW9bM#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-15T15:55:59Z DEBUG collector::execute] cd "/tmp/.tmpaaXBnr" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaaXBnr#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-15T15:56:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T15:56:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T15:56:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T15:56:43Z DEBUG collector::execute] cd "/tmp/.tmpsB3EPX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsB3EPX#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T15:57:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T15:57:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T15:57:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T15:57:16Z DEBUG collector::execute] cd "/tmp/.tmpUlBDZA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUlBDZA#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/7)
Preparing clap-3.1.6
[2023-03-15T15:59:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T15:59:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T15:59:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T15:59:13Z DEBUG collector::execute] cd "/tmp/.tmpPPfiKJ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPPfiKJ#clap@3.1.6" "--" "--skip-this-rustc"
[2023-03-15T15:59:13Z DEBUG collector::execute] cd "/tmp/.tmpxrhWRY" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxrhWRY#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-03-15T15:59:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T15:59:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T15:59:16Z DEBUG collector::execute] cd "/tmp/.tmpi8K8Sj" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpi8K8Sj#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
---
[2023-03-15T15:59:32Z DEBUG collector::execute] cd "/tmp/.tmpGcOv40" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGcOv40#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
Running hyper-0.14.18: Debug + [Full]
[2023-03-15T15:59:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T15:59:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T15:59:57Z DEBUG collector::execute] cd "/tmp/.tmpLiOJ3i" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLiOJ3i#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:00:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:00:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:00:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:00:01Z DEBUG collector::execute] cd "/tmp/.tmpLdqivD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLdqivD#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2023-03-15T16:00:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:00:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:00:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:00:10Z DEBUG collector::execute] cd "/tmp/.tmpLIXDqe" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLIXDqe#regex@1.5.5" "--" "--skip-this-rustc"
[2023-03-15T16:00:10Z DEBUG collector::execute] cd "/tmp/.tmpvXoO2j" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvXoO2j#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:00:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:00:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:00:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:00:20Z DEBUG collector::execute] cd "/tmp/.tmpxuUIqW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxuUIqW#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:00:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:00:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:00:23Z DEBUG collector::execute] cd "/tmp/.tmpU9D18B" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU9D18B#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/7)
Finished benchmark regex-1.5.5 (4/7)
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2023-03-15T16:00:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:00:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:00:35Z DEBUG collector::execute] cd "/tmp/.tmpIVd9KK" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIVd9KK#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-03-15T16:00:35Z DEBUG collector::execute] cd "/tmp/.tmpMbu3cB" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMbu3cB#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:01:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:01:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:01:00Z DEBUG collector::execute] cd "/tmp/.tmpOqhdMc" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOqhdMc#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
---
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2023-03-15T16:01:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:01:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:01:23Z DEBUG collector::execute] cd "/tmp/.tmprTwVSK" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprTwVSK#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:01:23Z DEBUG collector::execute] cd "/tmp/.tmpKbMnOo" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKbMnOo#serde@1.0.136" "--" "--skip-this-rustc"
[2023-03-15T16:01:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:01:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:01:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:01:24Z DEBUG collector::execute] cd "/tmp/.tmplLOtjn" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplLOtjn#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:01:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:01:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:01:27Z DEBUG collector::execute] cd "/tmp/.tmpR92OS9" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR92OS9#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark serde-1.0.136 (6/7)
Finished benchmark serde-1.0.136 (6/7)
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2023-03-15T16:01:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:01:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:01:31Z DEBUG collector::execute] cd "/tmp/.tmp2BqWIr" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2BqWIr#syn@1.0.89" "--" "--skip-this-rustc"
[2023-03-15T16:01:31Z DEBUG collector::execute] cd "/tmp/.tmp8a9T0u" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8a9T0u#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:01:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:01:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:01:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:01:35Z DEBUG collector::execute] cd "/tmp/.tmp2MwDxz" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2MwDxz#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:01:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:01:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:01:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:01:38Z DEBUG collector::execute] cd "/tmp/.tmpHrwwhL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHrwwhL#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging LLVM PGO profiles to /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata
stage-build INFO: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata /tmp/tmp-multistage/opt-artifacts/llvm-pgo`
stage-build INFO: LLVM PGO statistics
---
Preparing bitmaps-3.1.0
[2023-03-15T16:12:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:12:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:12:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:12:23Z DEBUG collector::execute] cd "/tmp/.tmp36JDt2" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp36JDt2#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T16:12:23Z DEBUG collector::execute] cd "/tmp/.tmp10iM0Z" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp10iM0Z#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:12:23Z DEBUG collector::execute] cd "/tmp/.tmpdCKShD" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdCKShD#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2023-03-15T16:12:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:12:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:12:23Z DEBUG collector::execute] cd "/tmp/.tmpekycT3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpekycT3#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:12:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:12:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:12:25Z DEBUG collector::execute] cd "/tmp/.tmpekycT3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpekycT3#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpekycT3/incremental-state"
[2023-03-15T16:12:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:12:28Z DEBUG collector::execute] cd "/tmp/.tmpekycT3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpekycT3#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpekycT3/incremental-state"
[2023-03-15T16:12:28Z DEBUG collector::execute] applying println to "/tmp/.tmpekycT3"
[2023-03-15T16:12:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:12:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:12:28Z DEBUG collector::execute] cd "/tmp/.tmpekycT3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpekycT3#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpekycT3/incremental-state"
[2023-03-15T16:12:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:12:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:12:29Z DEBUG collector::execute] cd "/tmp/.tmpM3UM1r" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpM3UM1r#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:12:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-03-15T16:12:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:12:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:12:36Z DEBUG collector::execute] cd "/tmp/.tmpH2cHr1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH2cHr1#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:12:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:12:38Z DEBUG collector::execute] cd "/tmp/.tmpH2cHr1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH2cHr1#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH2cHr1/incremental-state"
[2023-03-15T16:12:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:12:40Z DEBUG collector::execute] cd "/tmp/.tmpH2cHr1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH2cHr1#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH2cHr1/incremental-state"
[2023-03-15T16:12:41Z DEBUG collector::execute] applying println to "/tmp/.tmpH2cHr1"
[2023-03-15T16:12:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:12:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:12:41Z DEBUG collector::execute] cd "/tmp/.tmpH2cHr1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH2cHr1#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH2cHr1/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-03-15T16:12:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:12:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:12:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:12:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:12:42Z DEBUG collector::execute] cd "/tmp/.tmpqUnylm" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqUnylm#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-15T16:12:42Z DEBUG collector::execute] cd "/tmp/.tmpXEVCUD" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXEVCUD#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-15T16:12:42Z DEBUG collector::execute] cd "/tmp/.tmpeRi5gY" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeRi5gY#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-03-15T16:13:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:13:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:13:36Z DEBUG collector::execute] cd "/tmp/.tmpmZC2U6" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmZC2U6#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:13:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2023-03-15T16:14:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:14:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:14:25Z DEBUG collector::execute] cd "/tmp/.tmp0DSROi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0DSROi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:15:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T16:15:11Z DEBUG collector::execute] cd "/tmp/.tmp0DSROi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0DSROi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0DSROi/incremental-state"
[2023-03-15T16:16:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:16:05Z DEBUG collector::execute] cd "/tmp/.tmp0DSROi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0DSROi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0DSROi/incremental-state"
[2023-03-15T16:16:14Z DEBUG collector::execute] applying println to "/tmp/.tmp0DSROi"
[2023-03-15T16:16:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:16:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:16:14Z DEBUG collector::execute] cd "/tmp/.tmp0DSROi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0DSROi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp0DSROi/incremental-state"
[2023-03-15T16:16:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:16:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:16:25Z DEBUG collector::execute] cd "/tmp/.tmp7e2oiM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7e2oiM#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:17:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:17:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:17:18Z DEBUG collector::execute] cd "/tmp/.tmp7e2oiM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7e2oiM#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7e2oiM/incremental-state"
[2023-03-15T16:18:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:18:13Z DEBUG collector::execute] cd "/tmp/.tmp7e2oiM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7e2oiM#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7e2oiM/incremental-state"
[2023-03-15T16:18:22Z DEBUG collector::execute] applying println to "/tmp/.tmp7e2oiM"
[2023-03-15T16:18:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:18:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:18:22Z DEBUG collector::execute] cd "/tmp/.tmp7e2oiM" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7e2oiM#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp7e2oiM/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2023-03-15T16:18:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:18:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:18:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:18:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:18:32Z DEBUG collector::execute] cd "/tmp/.tmppmULAe" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppmULAe#ctfe-stress-5@0.1.0" "--" "--skip-this-rustc"
[2023-03-15T16:18:32Z DEBUG collector::execute] cd "/tmp/.tmpgDPjCV" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgDPjCV#ctfe-stress-5@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:18:32Z DEBUG collector::execute] cd "/tmp/.tmpsdkrxT" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsdkrxT#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T16:18:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:18:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:18:33Z DEBUG collector::execute] cd "/tmp/.tmpnRDFuE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnRDFuE#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:18:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:18:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:18:41Z DEBUG collector::execute] cd "/tmp/.tmpnRDFuE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnRDFuE#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnRDFuE/incremental-state"
[2023-03-15T16:18:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:18:51Z DEBUG collector::execute] cd "/tmp/.tmpnRDFuE" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnRDFuE#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpnRDFuE/incremental-state"
[2023-03-15T16:18:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:18:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:18:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:18:51Z DEBUG collector::execute] cd "/tmp/.tmptjiR2I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptjiR2I#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:19:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T16:19:00Z DEBUG collector::execute] cd "/tmp/.tmptjiR2I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptjiR2I#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptjiR2I/incremental-state"
[2023-03-15T16:19:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:19:10Z DEBUG collector::execute] cd "/tmp/.tmptjiR2I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptjiR2I#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmptjiR2I/incremental-state"
[2023-03-15T16:19:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:19:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:19:10Z DEBUG collector::execute] cd "/tmp/.tmp2GNJGm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2GNJGm#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:19:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:19:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:19:18Z DEBUG collector::execute] cd "/tmp/.tmp2GNJGm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2GNJGm#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2GNJGm/incremental-state"
[2023-03-15T16:19:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:19:28Z DEBUG collector::execute] cd "/tmp/.tmp2GNJGm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2GNJGm#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp2GNJGm/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2023-03-15T16:19:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:19:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:19:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:19:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:19:28Z DEBUG collector::execute] cd "/tmp/.tmpGgkXjj" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGgkXjj#diesel@1.4.8" "--" "--skip-this-rustc"
[2023-03-15T16:19:28Z DEBUG collector::execute] cd "/tmp/.tmpyvspJa" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyvspJa#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T16:19:28Z DEBUG collector::execute] cd "/tmp/.tmpjwIY86" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjwIY86#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:19:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:19:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:19:47Z DEBUG collector::execute] cd "/tmp/.tmpjMj04M" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjMj04M#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:20:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:20:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:20:01Z DEBUG collector::execute] cd "/tmp/.tmpjMj04M" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjMj04M#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjMj04M/incremental-state"
[2023-03-15T16:20:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:20:18Z DEBUG collector::execute] cd "/tmp/.tmpjMj04M" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjMj04M#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjMj04M/incremental-state"
[2023-03-15T16:20:22Z DEBUG collector::execute] applying println to "/tmp/.tmpjMj04M"
[2023-03-15T16:20:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:20:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:20:22Z DEBUG collector::execute] cd "/tmp/.tmpjMj04M" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjMj04M#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjMj04M/incremental-state"
[2023-03-15T16:20:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:20:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:20:25Z DEBUG collector::execute] cd "/tmp/.tmpcgF689" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcgF689#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:20:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-03-15T16:21:05Z DEBUG collector::execute] cd "/tmp/.tmpcgF689" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcgF689#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpcgF689/incremental-state"
Running diesel-1.4.8: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-03-15T16:21:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:21:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:21:09Z DEBUG collector::execute] cd "/tmp/.tmpMfbFKv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMfbFKv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:21:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:21:27Z DEBUG collector::execute] cd "/tmp/.tmpMfbFKv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMfbFKv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMfbFKv/incremental-state"
[2023-03-15T16:21:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:21:47Z DEBUG collector::execute] cd "/tmp/.tmpMfbFKv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMfbFKv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMfbFKv/incremental-state"
[2023-03-15T16:21:50Z DEBUG collector::execute] applying println to "/tmp/.tmpMfbFKv"
[2023-03-15T16:21:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:21:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-15T16:21:50Z DEBUG collector::execute] cd "/tmp/.tmpMfbFKv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMfbFKv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMfbFKv/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-03-15T16:21:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:21:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:21:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:21:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:21:54Z DEBUG collector::execute] cd "/tmp/.tmpSbWp2v" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSbWp2v#externs@0.1.0" "--" "--skip-this-rustc"
[2023-03-15T16:21:54Z DEBUG collector::execute] cd "/tmp/.tmpUu9xRz" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUu9xRz#externs@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T16:21:54Z DEBUG collector::execute] cd "/tmp/.tmpfNW8ge" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfNW8ge#externs@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:21:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:21:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:21:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:21:55Z DEBUG collector::execute] cd "/tmp/.tmpKWbPVI" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKWbPVI#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:21:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:21:55Z DEBUG collector::execute] cd "/tmp/.tmpKWbPVI" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKWbPVI#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKWbPVI/incremental-state"
[2023-03-15T16:21:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:21:56Z DEBUG collector::execute] cd "/tmp/.tmpKWbPVI" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKWbPVI#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKWbPVI/incremental-state"
[2023-03-15T16:21:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:21:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:21:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:21:56Z DEBUG collector::execute] cd "/tmp/.tmpys2ffV" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpys2ffV#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:21:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T16:21:57Z DEBUG collector::execute] cd "/tmp/.tmpys2ffV" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpys2ffV#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpys2ffV/incremental-state"
[2023-03-15T16:21:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:21:58Z DEBUG collector::execute] cd "/tmp/.tmpys2ffV" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpys2ffV#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpys2ffV/incremental-state"
[2023-03-15T16:21:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:21:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:21:58Z DEBUG collector::execute] cd "/tmp/.tmpG1cPo8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG1cPo8#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:21:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:21:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:21:59Z DEBUG collector::execute] cd "/tmp/.tmpG1cPo8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG1cPo8#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpG1cPo8/incremental-state"
[2023-03-15T16:22:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:22:00Z DEBUG collector::execute] cd "/tmp/.tmpG1cPo8" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG1cPo8#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpG1cPo8/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-03-15T16:22:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:22:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:22:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:22:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:22:00Z DEBUG collector::execute] cd "/tmp/.tmpMQcJpW" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMQcJpW#match-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T16:22:00Z DEBUG collector::execute] cd "/tmp/.tmpbCNJvU" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbCNJvU#match-stress@0.1.0" "--" "--skip-this-rustc"
[2023-03-15T16:22:00Z DEBUG collector::execute] cd "/tmp/.tmpaZxgrp" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaZxgrp#match-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:22:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:22:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:22:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:22:01Z DEBUG collector::execute] cd "/tmp/.tmp6duNjf" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6duNjf#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:22:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:03Z DEBUG collector::execute] cd "/tmp/.tmp6duNjf" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6duNjf#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6duNjf/incremental-state"
[2023-03-15T16:22:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:22:05Z DEBUG collector::execute] cd "/tmp/.tmp6duNjf" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6duNjf#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp6duNjf/incremental-state"
[2023-03-15T16:22:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:22:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:22:06Z DEBUG collector::execute] cd "/tmp/.tmpd1J9di" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd1J9di#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:22:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:08Z DEBUG collector::execute] cd "/tmp/.tmpd1J9di" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd1J9di#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpd1J9di/incremental-state"
[2023-03-15T16:22:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:22:11Z DEBUG collector::execute] cd "/tmp/.tmpd1J9di" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpd1J9di#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpd1J9di/incremental-state"
Running match-stress: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-03-15T16:22:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:22:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:22:12Z DEBUG collector::execute] cd "/tmp/.tmpxHKLNR" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxHKLNR#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:22:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:14Z DEBUG collector::execute] cd "/tmp/.tmpxHKLNR" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxHKLNR#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxHKLNR/incremental-state"
[2023-03-15T16:22:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:22:17Z DEBUG collector::execute] cd "/tmp/.tmpxHKLNR" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxHKLNR#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxHKLNR/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-03-15T16:22:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:22:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:22:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:22:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:22:18Z DEBUG collector::execute] cd "/tmp/.tmpuEPHEQ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuEPHEQ#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-15T16:22:18Z DEBUG collector::execute] cd "/tmp/.tmpvjSe1p" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvjSe1p#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-15T16:22:18Z DEBUG collector::execute] cd "/tmp/.tmpO3DQZ3" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpO3DQZ3#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--skip-this-rustc"
[2023-03-15T16:22:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:22:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:22:19Z DEBUG collector::execute] cd "/tmp/.tmpq9HBX3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpq9HBX3#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:22:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2023-03-15T16:22:20Z DEBUG collector::execute] cd "/tmp/.tmp57VjGc" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp57VjGc#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp57VjGc/incremental-state"
Running token-stream-stress: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-03-15T16:22:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:22:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:22:20Z DEBUG collector::execute] cd "/tmp/.tmpxg8Emv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxg8Emv#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:22:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:20Z DEBUG collector::execute] cd "/tmp/.tmpxg8Emv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxg8Emv#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxg8Emv/incremental-state"
[2023-03-15T16:22:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:22:21Z DEBUG collector::execute] cd "/tmp/.tmpxg8Emv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxg8Emv#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxg8Emv/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-03-15T16:22:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:22:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:22:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:22:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:22:21Z DEBUG collector::execute] cd "/tmp/.tmpE28tRf" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpE28tRf#tuple-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T16:22:21Z DEBUG collector::execute] cd "/tmp/.tmpzToyx2" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzToyx2#tuple-stress@0.1.0" "--" "--skip-this-rustc"
[2023-03-15T16:22:21Z DEBUG collector::execute] cd "/tmp/.tmpl92aVD" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpl92aVD#tuple-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:22:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:22:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:22:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:22:21Z DEBUG collector::execute] cd "/tmp/.tmpFqcLWb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFqcLWb#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:22:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:26Z DEBUG collector::execute] cd "/tmp/.tmpFqcLWb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFqcLWb#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFqcLWb/incremental-state"
[2023-03-15T16:22:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:22:31Z DEBUG collector::execute] cd "/tmp/.tmpFqcLWb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFqcLWb#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFqcLWb/incremental-state"
[2023-03-15T16:22:32Z DEBUG collector::execute] applying new row to "/tmp/.tmpFqcLWb"
[2023-03-15T16:22:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T16:22:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T16:22:32Z DEBUG collector::execute] cd "/tmp/.tmpFqcLWb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFqcLWb#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFqcLWb/incremental-state"
[2023-03-15T16:22:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:22:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:22:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:22:37Z DEBUG collector::execute] cd "/tmp/.tmpIrwUjm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIrwUjm#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:22:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:41Z DEBUG collector::execute] cd "/tmp/.tmpIrwUjm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIrwUjm#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIrwUjm/incremental-state"
[2023-03-15T16:22:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:22:46Z DEBUG collector::execute] cd "/tmp/.tmpIrwUjm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIrwUjm#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIrwUjm/incremental-state"
[2023-03-15T16:22:48Z DEBUG collector::execute] applying new row to "/tmp/.tmpIrwUjm"
[2023-03-15T16:22:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T16:22:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T16:22:48Z DEBUG collector::execute] cd "/tmp/.tmpIrwUjm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIrwUjm#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpIrwUjm/incremental-state"
[2023-03-15T16:22:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:22:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:22:53Z DEBUG collector::execute] cd "/tmp/.tmp9FyIPi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9FyIPi#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:22:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-15T16:22:57Z DEBUG collector::execute] cd "/tmp/.tmp9FyIPi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9FyIPi#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9FyIPi/incremental-state"
[2023-03-15T16:23:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-15T16:23:03Z DEBUG collector::execute] cd "/tmp/.tmp9FyIPi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9FyIPi#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9FyIPi/incremental-state"
[2023-03-15T16:23:04Z DEBUG collector::execute] applying new row to "/tmp/.tmp9FyIPi"
[2023-03-15T16:23:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T16:23:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-15T16:23:04Z DEBUG collector::execute] cd "/tmp/.tmp9FyIPi" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9FyIPi#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp9FyIPi/incremental-state"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging Rustc PGO profiles to /tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata
stage-build INFO: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata /tmp/tmp-multistage/opt-artifacts/rustc-pgo`
stage-build INFO: Rustc PGO statistics
---
[RUSTC-TIMING] rustc_error_codes test:false 1.923
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
[RUSTC-TIMING] build_script_build test:false 4.052
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvMs5_NtCsdBlHRorbRmI_9hashbrown3rawINtB6_8RawTableTNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE0ECsaVf5JQ9vlaw_10rustc_smir Hash = 1126541921141000735 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvMs5_NtCsdBlHRorbRmI_9hashbrown3rawINtB6_8RawTableTNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE0ECsaVf5JQ9vlaw_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvMs5_NtCsdBlHRorbRmI_9hashbrown3rawINtB6_8RawTableTNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE0ECsaVf5JQ9vlaw_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCsdBlHRorbRmI_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCs9rp5xvMaM5Y_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECsaVf5JQ9vlaw_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCsdBlHRorbRmI_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCs9rp5xvMaM5Y_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECsaVf5JQ9vlaw_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.13: no profile data available for function _RINvXs8_NtCsdBlHRorbRmI_9hashbrown3setINtB6_7HashSetNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtNtNtB21_4iter6traits7collect6ExtendBO_E6extendINtNtNtB3t_8adapters6copied6CopiedINtNtNtB21_5slice4iter4IterBO_EEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.13: no profile data available for function _RNvMs2_NtCsdBlHRorbRmI_9hashbrown3setINtB5_7HashSetNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE6insertCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVechEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNvNtNtNtNtCsbsWagiDJuys_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvXs2_NtNtNtCsdPxStG2PgiO_4core3ops8function5implsQNCNCNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15all_local_items00INtB7_6FnOnceTRNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdEE9call_onceBW_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir11local_crate Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15external_crates Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir10find_crate Hash = 578412082070520035 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15all_local_items Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir10smir_crate Hash = 737863019117533973 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.9: no profile data available for function _RNvXNtNtCsdPxStG2PgiO_4core5slice3cmpShNtNtB6_3cmp9PartialEq2eqCsaVf5JQ9vlaw_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RINvNtCs9rp5xvMaM5Y_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsaVf5JQ9vlaw_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RINvNvMs_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RINvNvMs_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvMs0_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvMs0_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvMs_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB4_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvMs_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB4_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvXs1_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateENtNtNtCsdPxStG2PgiO_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvXs1_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemENtNtNtCsdPxStG2PgiO_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvXs1_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVechENtNtNtCsdPxStG2PgiO_4core3ops4drop4Drop4dropCsaVf5JQ9vlaw_10rustc_smir Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RINvNtCsjAa8tEeucuA_21rustc_data_structures4sync11assert_syncNtNtNtNtCsbsWagiDJuys_12rustc_middle2ty7context3tls12ImplicitCtxtECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RINvNtNtCsdPxStG2PgiO_4core5alloc6layout10size_alignTNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RINvYINtNtNtCsdPxStG2PgiO_4core5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexENtNtNtNtBa_4iter6traits8iterator8Iterator6copiedBJ_ECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvMs_NtNtNtNtNtCs3hdLqqt4OAL_3std3sys6common12thread_local10fast_local4fastINtB4_3KeyINtNtCsdPxStG2PgiO_4core4cell4CellPuEE13register_dtorCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertNtNtCs9rp5xvMaM5Y_5alloc11collections19TryReserveErrorKindINtB5_4IntoNtBA_15TryReserveErrorE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumINtB5_4IntojE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertjINtB5_4IntoNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXs5_NtCsdPxStG2PgiO_4core7convertjINtB5_7TryFromjE8try_fromCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXsV_NtCsdPxStG2PgiO_4core3fmtRNtNtCshqfWlKPmSMb_10rustc_span6def_id5DefIdNtB5_5Debug3fmtCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXsV_NtCsdPxStG2PgiO_4core3fmtRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvMs5_NtCsdPxStG2PgiO_4core3anyNtB6_6TypeId2ofNtNtCskR5GL0FukAJ_12tracing_core8callsite15DefaultCallsiteECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCsbsWagiDJuys_12rustc_middle9dep_graph8dep_node7DepKindEEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvXNtCsbsWagiDJuys_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECsaVf5JQ9vlaw_10rustc_smir Hash = 269792486063750595 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvXsv_NtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCsdPxStG2PgiO_4core4hash4Hash4hashNtCs4bosUpeLYRb_10rustc_hash8FxHasherECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCsaVf5JQ9vlaw_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RNvNvNtNtNtNtCsbsWagiDJuys_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RNvXNtCsdPxStG2PgiO_4core6borrowNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_6BorrowBu_E6borrowCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB5_4IntoNtNtCsjAa8tEeucuA_21rustc_data_structures9profiling17QueryInvocationIdE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVechEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeRNtNtCshqfWlKPmSMb_10rustc_span6def_id5DefIdECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeRbECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placejECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hf41f9a7061e8477cE Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h6ded8a9427c41adaE Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir10find_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir15all_local_items Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvXs3_NtCsaVf5JQ9vlaw_10rustc_smir10stable_mirNtB5_5CrateNtNtCsdPxStG2PgiO_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvXs9_NtCsaVf5JQ9vlaw_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCsdPxStG2PgiO_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVechEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RINvXs_NtNtNtCsdPxStG2PgiO_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateNCNCNvNtB3x_10rustc_smir10find_crate00E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 23700016549702699 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters5chainINtB2_5ChainINtNtNtB8_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumEBX_E3newCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateuNCNCNvNtB2a_10rustc_smir15external_crates00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCs9rp5xvMaM5Y_5alloc3vecINtB4G_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RINvXs0_NtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsdBlHRorbRmI_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RINvXs0_NtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumENCNCNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15external_crates00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3c_8for_each4callNtNtB2g_10stable_mir5CrateNCINvMsi_NtCs9rp5xvMaM5Y_5alloc3vecINtB4O_3VecB4f_E14extend_trustedBN_E0E0EB2g_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB2_3MapINtNtB4_6copied6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsdBlHRorbRmI_9hashbrown3setINtB2Y_7HashSetB1F_INtNtB8_4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtNtB6_6traits7collect6ExtendB1F_E6extendBT_E0E3newCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB2_3MapINtNtCsjVDvwP8TuwX_8indexmap3set4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdENCNCNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15all_local_items00E3newB2q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB2_3MapINtNtNtB8_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumENCNCNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15external_crates00E3newB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.1: no profile data available for function _RINvCs9k1vx9xyVJd_8smallvec10infallibleuECsaVf5JQ9vlaw_10rustc_smir Hash = 1063705160175073211 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.1: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtCs9k1vx9xyVJd_8smallvec18CollectionAllocErrECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.1: no profile data available for function _RNvMsc_Cs9k1vx9xyVJd_8smallvecINtB5_8SmallVecANtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexj8_E11try_reserveCsaVf5JQ9vlaw_10rustc_smir Hash = 832846567024380499 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvMsH_NtCskR5GL0FukAJ_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsdPxStG2PgiO_4core6option6OptionRDNtB6_5ValueEL_EEj3_ECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeRNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvNtCskR5GL0FukAJ_12tracing_core5field5debugRNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvNtCskR5GL0FukAJ_12tracing_core5field5debugRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvXs_NtNtNtCsdPxStG2PgiO_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCsdBlHRorbRmI_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECsaVf5JQ9vlaw_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters6copiedINtB2_6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEE3newCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RNvXs_NtNtNtCsdPxStG2PgiO_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RNvXsu_NtCskR5GL0FukAJ_12tracing_core5fieldINtB5_10DebugValueRNtNtCs9rp5xvMaM5Y_5alloc6string6StringENtB5_5Value6recordCsaVf5JQ9vlaw_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RNvXsu_NtCskR5GL0FukAJ_12tracing_core5fieldINtB5_10DebugValueRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumENtB5_5Value6recordCsaVf5JQ9vlaw_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc3vec3VecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc3vec3VecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RNvXNtNtCs9rp5xvMaM5Y_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCsdPxStG2PgiO_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumENCNCNvNtBY_10rustc_smir15external_crates00EE9from_iterBY_ Hash = 1096621587988718700 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RNvXNtNtCs9rp5xvMaM5Y_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCsdPxStG2PgiO_4core4iter8adapters3map3MapINtNtCsjVDvwP8TuwX_8indexmap3set4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdENCNCNvNtBY_10rustc_smir15all_local_items00EE9from_iterBY_ Hash = 421769329463800046 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RNvXsn_NtCs9rp5xvMaM5Y_5alloc3vecINtB5_3VechENtNtNtCsdPxStG2PgiO_4core3ops4drop4Drop4dropCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.15: no profile data available for function _RINvNtNtNtCsdBlHRorbRmI_9hashbrown3raw5alloc5inner8do_allocNtNtCs9rp5xvMaM5Y_5alloc5alloc6GlobalECsaVf5JQ9vlaw_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.15: no profile data available for function _RNvMs2_NtCsjVDvwP8TuwX_8indexmap3setINtB5_8IndexSetNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE4iterCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.15: no profile data available for function _RNvXsb_NtCsjVDvwP8TuwX_8indexmap3setINtB5_4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdENtNtNtNtCsdPxStG2PgiO_4core4iter6traits8iterator8Iterator4nextCsaVf5JQ9vlaw_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.15: no profile data available for function _RNvXsb_NtCsjVDvwP8TuwX_8indexmap3setINtB5_4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdENtNtNtNtCsdPxStG2PgiO_4core4iter6traits8iterator8Iterator9size_hintCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvMs2_NtNtCs3hdLqqt4OAL_3std6thread5localINtB6_8LocalKeyINtNtCsdPxStG2PgiO_4core4cell4CellPuEE4withNCNvNtNtNtNtCsbsWagiDJuys_12rustc_middle2ty7context3tls3tlv7get_tlv0B1s_ECsaVf5JQ9vlaw_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVechEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtNtCs3hdLqqt4OAL_3std6thread5local11AccessErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvYINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvMs0_CsjVDvwP8TuwX_8indexmapINtB5_6BucketNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIduE7key_refCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertINtNtNtB7_3ptr6unique6UniquehEINtB5_4IntoINtNtBD_8non_null7NonNullhEE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvXs1_NtNtNtCsdPxStG2PgiO_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateNCNCNvNtB2E_10rustc_smir10find_crate00E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 1147234025963109369 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvXsV_NtCsdPxStG2PgiO_4core3fmtRNtNtCs9rp5xvMaM5Y_5alloc6string6StringNtB5_5Debug3fmtCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvXsV_NtCsdPxStG2PgiO_4core3fmtRbNtB5_5Debug3fmtCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir14rustc_internal11item_def_id Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir14rustc_internal9crate_num Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.5: no profile data available for function _RINvNtCsdBlHRorbRmI_9hashbrown3map9make_hashNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexBG_INtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.5: no profile data available for function _RINvXs1x_NtCsdBlHRorbRmI_9hashbrown3mapINtB7_7HashMapNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECsaVf5JQ9vlaw_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.5: no profile data available for function _RNvMs1_NtCsdBlHRorbRmI_9hashbrown3mapINtB5_7HashMapNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE6insertCsaVf5JQ9vlaw_10rustc_smir Hash = 11922953328495428 up to 0 count discarded
[RUSTC-TIMING] jemalloc_sys test:false 1.123
[RUSTC-TIMING] rustc_smir test:false 9.177
warning: `rustc_smir` (lib) generated 111 warnings
[RUSTC-TIMING] rustc_ty_utils test:false 41.622
---
Preparing cargo-0.60.0
[2023-03-15T16:56:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:56:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:56:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:56:02Z DEBUG collector::execute] cd "/tmp/.tmp5egOoi" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5egOoi#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-03-15T16:56:02Z DEBUG collector::execute] cd "/tmp/.tmpN15wjc" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN15wjc#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-03-15T16:56:02Z DEBUG collector::execute] cd "/tmp/.tmpb9nsTW" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpb9nsTW#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-03-15T16:57:55Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:57:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:57:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:57:56Z DEBUG collector::execute] cd "/tmp/.tmpBtjBge" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBtjBge#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:58:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:58:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:58:03Z DEBUG collector::execute] cd "/tmp/.tmp0ZbFv3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0ZbFv3#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Opt + [Full]
Running cargo-0.60.0: Opt + [Full]
[2023-03-15T16:58:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:58:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T16:58:33Z DEBUG collector::execute] cd "/tmp/.tmpF9drcn" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpF9drcn#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/7)
Preparing clap-3.1.6
[2023-03-15T16:59:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T16:59:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:59:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T16:59:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T16:59:43Z DEBUG collector::execute] cd "/tmp/.tmpH498Fu" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH498Fu#clap@3.1.6" "--" "--skip-this-rustc"
[2023-03-15T16:59:43Z DEBUG collector::execute] cd "/tmp/.tmpARTXpN" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpARTXpN#clap@3.1.6" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T16:59:43Z DEBUG collector::execute] cd "/tmp/.tmp947LhE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp947LhE#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-03-15T16:59:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:59:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:59:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T16:59:56Z DEBUG collector::execute] cd "/tmp/.tmphpyNZ0" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphpyNZ0#clap@3.1.6" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T16:59:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T16:59:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:59:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T16:59:58Z DEBUG collector::execute] cd "/tmp/.tmpFH6Lph" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFH6Lph#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:00:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:00:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T17:00:04Z DEBUG collector::execute] cd "/tmp/.tmpMW6hGX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMW6hGX#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark clap-3.1.6 (2/7)
Finished benchmark clap-3.1.6 (2/7)
Executing benchmark hyper-0.14.18 (3/7)
Preparing hyper-0.14.18
[2023-03-15T17:00:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T17:00:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T17:00:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T17:00:19Z DEBUG collector::execute] cd "/tmp/.tmpDV37Ip" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDV37Ip#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-15T17:00:19Z DEBUG collector::execute] cd "/tmp/.tmpsQwhqL" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsQwhqL#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-15T17:00:19Z DEBUG collector::execute] cd "/tmp/.tmpsFICSc" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsFICSc#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-03-15T17:01:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:01:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:01:08Z DEBUG collector::execute] cd "/tmp/.tmpyu0G3U" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyu0G3U#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Debug + [Full]
Running hyper-0.14.18: Debug + [Full]
[2023-03-15T17:01:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:01:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T17:01:11Z DEBUG collector::execute] cd "/tmp/.tmpQKsMWV" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQKsMWV#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:01:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:01:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T17:01:17Z DEBUG collector::execute] cd "/tmp/.tmp2YCqI4" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2YCqI4#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/7)
Finished benchmark hyper-0.14.18 (3/7)
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2023-03-15T17:01:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T17:01:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T17:01:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T17:01:33Z DEBUG collector::execute] cd "/tmp/.tmp5IQlky" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5IQlky#regex@1.5.5" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T17:01:33Z DEBUG collector::execute] cd "/tmp/.tmptusBzr" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptusBzr#regex@1.5.5" "--" "--skip-this-rustc"
[2023-03-15T17:01:33Z DEBUG collector::execute] cd "/tmp/.tmpCQt0im" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCQt0im#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-03-15T17:01:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:01:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:01:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:01:50Z DEBUG collector::execute] cd "/tmp/.tmprJDSpo" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprJDSpo#regex@1.5.5" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:01:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:01:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T17:01:52Z DEBUG collector::execute] cd "/tmp/.tmp7ZF1og" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7ZF1og#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
Running regex-1.5.5: Opt + [Full]
[2023-03-15T17:01:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:01:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T17:01:57Z DEBUG collector::execute] cd "/tmp/.tmpLzDEbq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLzDEbq#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0 (5/7)
Preparing ripgrep-13.0.0
[2023-03-15T17:02:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T17:02:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T17:02:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T17:02:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T17:02:13Z DEBUG collector::execute] cd "/tmp/.tmpZ4vQWO" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZ4vQWO#ripgrep@13.0.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T17:02:13Z DEBUG collector::execute] cd "/tmp/.tmptcQJPd" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptcQJPd#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-03-15T17:02:13Z DEBUG collector::execute] cd "/tmp/.tmp49BEIb" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp49BEIb#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-03-15T17:03:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:03:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:03:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:03:13Z DEBUG collector::execute] cd "/tmp/.tmpIxUNqh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIxUNqh#ripgrep@13.0.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:03:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:03:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T17:03:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T17:03:14Z DEBUG collector::execute] cd "/tmp/.tmpc0lUHh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc0lUHh#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:03:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:03:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T17:03:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T17:03:21Z DEBUG collector::execute] cd "/tmp/.tmpiuSpdZ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiuSpdZ#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (6/7)
Preparing serde-1.0.136
[2023-03-15T17:03:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T17:03:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T17:03:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T17:03:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T17:03:40Z DEBUG collector::execute] cd "/tmp/.tmpgGGKMv" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgGGKMv#serde@1.0.136" "--" "--skip-this-rustc"
[2023-03-15T17:03:40Z DEBUG collector::execute] cd "/tmp/.tmpSh9blD" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSh9blD#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-03-15T17:03:40Z DEBUG collector::execute] cd "/tmp/.tmpCDQblS" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCDQblS#serde@1.0.136" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T17:03:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:03:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:03:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:03:45Z DEBUG collector::execute] cd "/tmp/.tmpCeqhyY" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCeqhyY#serde@1.0.136" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:03:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:03:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T17:03:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T17:03:47Z DEBUG collector::execute] cd "/tmp/.tmp2LWoe1" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2LWoe1#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:03:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:03:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T17:03:52Z DEBUG collector::execute] cd "/tmp/.tmpD62bkP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpD62bkP#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark serde-1.0.136 (6/7)
Finished benchmark serde-1.0.136 (6/7)
Executing benchmark syn-1.0.89 (7/7)
Preparing syn-1.0.89
[2023-03-15T17:04:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-15T17:04:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-03-15T17:04:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-03-15T17:04:01Z DEBUG collector::execute] cd "/tmp/.tmpQSHWUn" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQSHWUn#syn@1.0.89" "--profile" "check" "--" "--skip-this-rustc"
[2023-03-15T17:04:01Z DEBUG collector::execute] cd "/tmp/.tmpmRkGMK" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmRkGMK#syn@1.0.89" "--" "--skip-this-rustc"
[2023-03-15T17:04:01Z DEBUG collector::execute] cd "/tmp/.tmpgBitcw" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgBitcw#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-03-15T17:04:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:04:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:04:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-15T17:04:15Z DEBUG collector::execute] cd "/tmp/.tmpASzXUS" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpASzXUS#syn@1.0.89" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:04:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:04:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T17:04:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-15T17:04:16Z DEBUG collector::execute] cd "/tmp/.tmpejoCOx" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpejoCOx#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-15T17:04:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-15T17:04:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-15T17:04:21Z DEBUG collector::execute] cd "/tmp/.tmpc2775K" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc2775K#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
---
[RUSTC-TIMING] rustc_incremental test:false 22.210
[RUSTC-TIMING] rustc_query_impl test:false 115.992
[RUSTC-TIMING] rustc_monomorphize test:false 23.700
[RUSTC-TIMING] rustc_ast_lowering test:false 49.211
warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvMs5_NtCsdBlHRorbRmI_9hashbrown3rawINtB6_8RawTableTNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE0ECsaVf5JQ9vlaw_10rustc_smir Hash = 1126541921141000735 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvMs5_NtCsdBlHRorbRmI_9hashbrown3rawINtB6_8RawTableTNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE0ECsaVf5JQ9vlaw_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvMs5_NtCsdBlHRorbRmI_9hashbrown3rawINtB6_8RawTableTNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_BQ_uINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE0ECsaVf5JQ9vlaw_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCsdBlHRorbRmI_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCs9rp5xvMaM5Y_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECsaVf5JQ9vlaw_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.2: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCsdBlHRorbRmI_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCs9rp5xvMaM5Y_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECsaVf5JQ9vlaw_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.13: no profile data available for function _RINvXs8_NtCsdBlHRorbRmI_9hashbrown3setINtB6_7HashSetNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtNtNtB21_4iter6traits7collect6ExtendBO_E6extendINtNtNtB3t_8adapters6copied6CopiedINtNtNtB21_5slice4iter4IterBO_EEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.13: no profile data available for function _RNvMs2_NtCsdBlHRorbRmI_9hashbrown3setINtB5_7HashSetNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE6insertCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVechEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNvNtNtNtNtCsbsWagiDJuys_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvXs2_NtNtNtCsdPxStG2PgiO_4core3ops8function5implsQNCNCNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15all_local_items00INtB7_6FnOnceTRNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdEE9call_onceBW_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir11local_crate Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15external_crates Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir10find_crate Hash = 578412082070520035 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15all_local_items Hash = 18663488239531952 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.4: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir10smir_crate Hash = 737863019117533973 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.9: no profile data available for function _RNvXNtNtCsdPxStG2PgiO_4core5slice3cmpShNtNtB6_3cmp9PartialEq2eqCsaVf5JQ9vlaw_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RINvNtCs9rp5xvMaM5Y_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsaVf5JQ9vlaw_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RINvNvMs_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RINvNvMs_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 1061802868857037553 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvMs0_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvMs0_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemE13needs_to_growBQ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvMs_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB4_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvMs_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB4_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 159676625634057410 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvXs1_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateENtNtNtCsdPxStG2PgiO_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvXs1_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemENtNtNtCsdPxStG2PgiO_4core3ops4drop4Drop4dropBQ_ Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.0: no profile data available for function _RNvXs1_NtCs9rp5xvMaM5Y_5alloc7raw_vecINtB5_6RawVechENtNtNtCsdPxStG2PgiO_4core3ops4drop4Drop4dropCsaVf5JQ9vlaw_10rustc_smir Hash = 536873291871879055 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RINvNtCsjAa8tEeucuA_21rustc_data_structures4sync11assert_syncNtNtNtNtCsbsWagiDJuys_12rustc_middle2ty7context3tls12ImplicitCtxtECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RINvNtNtCsdPxStG2PgiO_4core5alloc6layout10size_alignTNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RINvYINtNtNtCsdPxStG2PgiO_4core5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexENtNtNtNtBa_4iter6traits8iterator8Iterator6copiedBJ_ECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvMs_NtNtNtNtNtCs3hdLqqt4OAL_3std3sys6common12thread_local10fast_local4fastINtB4_3KeyINtNtCsdPxStG2PgiO_4core4cell4CellPuEE13register_dtorCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertNtNtCs9rp5xvMaM5Y_5alloc11collections19TryReserveErrorKindINtB5_4IntoNtBA_15TryReserveErrorE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumINtB5_4IntojE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertjINtB5_4IntoNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXs5_NtCsdPxStG2PgiO_4core7convertjINtB5_7TryFromjE8try_fromCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXsV_NtCsdPxStG2PgiO_4core3fmtRNtNtCshqfWlKPmSMb_10rustc_span6def_id5DefIdNtB5_5Debug3fmtCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.14: no profile data available for function _RNvXsV_NtCsdPxStG2PgiO_4core3fmtRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvMs5_NtCsdPxStG2PgiO_4core3anyNtB6_6TypeId2ofNtNtCskR5GL0FukAJ_12tracing_core8callsite15DefaultCallsiteECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCsbsWagiDJuys_12rustc_middle9dep_graph8dep_node7DepKindEEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvXNtCsbsWagiDJuys_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECsaVf5JQ9vlaw_10rustc_smir Hash = 269792486063750595 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RINvXsv_NtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graphNtB6_12DepNodeIndexNtNtCsdPxStG2PgiO_4core4hash4Hash4hashNtCs4bosUpeLYRb_10rustc_hash8FxHasherECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCsaVf5JQ9vlaw_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RNvNvNtNtNtNtCsbsWagiDJuys_12rustc_middle2ty7context3tls3tlv3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RNvXNtCsdPxStG2PgiO_4core6borrowNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_6BorrowBu_E6borrowCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.12: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB5_4IntoNtNtCsjAa8tEeucuA_21rustc_data_structures9profiling17QueryInvocationIdE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVechEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeRNtNtCshqfWlKPmSMb_10rustc_span6def_id5DefIdECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeRbECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placejECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hf41f9a7061e8477cE Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h6ded8a9427c41adaE Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir10find_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir15all_local_items Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvXs3_NtCsaVf5JQ9vlaw_10rustc_smir10stable_mirNtB5_5CrateNtNtCsdPxStG2PgiO_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.11: no profile data available for function _RNvXs9_NtCsaVf5JQ9vlaw_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCsdPxStG2PgiO_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVechEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RINvXs_NtNtNtCsdPxStG2PgiO_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateNCNCNvNtB3x_10rustc_smir10find_crate00E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 23700016549702699 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.6: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters5chainINtB2_5ChainINtNtNtB8_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumEBX_E3newCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateuNCNCNvNtB2a_10rustc_smir15external_crates00NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCs9rp5xvMaM5Y_5alloc3vecINtB4G_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RINvXs0_NtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsdBlHRorbRmI_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RINvXs0_NtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumENCNCNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15external_crates00ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3c_8for_each4callNtNtB2g_10stable_mir5CrateNCINvMsi_NtCs9rp5xvMaM5Y_5alloc3vecINtB4O_3VecB4f_E14extend_trustedBN_E0E0EB2g_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB2_3MapINtNtB4_6copied6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCsdBlHRorbRmI_9hashbrown3setINtB2Y_7HashSetB1F_INtNtB8_4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtNtB6_6traits7collect6ExtendB1F_E6extendBT_E0E3newCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB2_3MapINtNtCsjVDvwP8TuwX_8indexmap3set4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdENCNCNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15all_local_items00E3newB2q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.7: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters3mapINtB2_3MapINtNtNtB8_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumENCNCNvNtCsaVf5JQ9vlaw_10rustc_smir10rustc_smir15external_crates00E3newB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.1: no profile data available for function _RINvCs9k1vx9xyVJd_8smallvec10infallibleuECsaVf5JQ9vlaw_10rustc_smir Hash = 1063705160175073211 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.1: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtCs9k1vx9xyVJd_8smallvec18CollectionAllocErrECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.1: no profile data available for function _RNvMsc_Cs9k1vx9xyVJd_8smallvecINtB5_8SmallVecANtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexj8_E11try_reserveCsaVf5JQ9vlaw_10rustc_smir Hash = 832846567024380499 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvMsH_NtCskR5GL0FukAJ_12tracing_core5fieldNtB6_8FieldSet9value_setATRNtB6_5FieldINtNtCsdPxStG2PgiO_4core6option6OptionRDNtB6_5ValueEL_EEj3_ECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeRNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvNtCskR5GL0FukAJ_12tracing_core5field5debugRNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvNtCskR5GL0FukAJ_12tracing_core5field5debugRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RINvXs_NtNtNtCsdPxStG2PgiO_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCsdBlHRorbRmI_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECsaVf5JQ9vlaw_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RNvMNtNtNtCsdPxStG2PgiO_4core4iter8adapters6copiedINtB2_6CopiedINtNtNtB8_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEE3newCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RNvXs_NtNtNtCsdPxStG2PgiO_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RNvXsu_NtCskR5GL0FukAJ_12tracing_core5fieldINtB5_10DebugValueRNtNtCs9rp5xvMaM5Y_5alloc6string6StringENtB5_5Value6recordCsaVf5JQ9vlaw_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.8: no profile data available for function _RNvXsu_NtCskR5GL0FukAJ_12tracing_core5fieldINtB5_10DebugValueRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumENtB5_5Value6recordCsaVf5JQ9vlaw_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc3vec3VecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 391331301844825284 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc3vec3VecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RNvXNtNtCs9rp5xvMaM5Y_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCsdPxStG2PgiO_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumENCNCNvNtBY_10rustc_smir15external_crates00EE9from_iterBY_ Hash = 1096621587988718700 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RNvXNtNtCs9rp5xvMaM5Y_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCsdPxStG2PgiO_4core4iter8adapters3map3MapINtNtCsjVDvwP8TuwX_8indexmap3set4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdENCNCNvNtBY_10rustc_smir15all_local_items00EE9from_iterBY_ Hash = 421769329463800046 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.3: no profile data available for function _RNvXsn_NtCs9rp5xvMaM5Y_5alloc3vecINtB5_3VechENtNtNtCsdPxStG2PgiO_4core3ops4drop4Drop4dropCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.15: no profile data available for function _RINvNtNtNtCsdBlHRorbRmI_9hashbrown3raw5alloc5inner8do_allocNtNtCs9rp5xvMaM5Y_5alloc5alloc6GlobalECsaVf5JQ9vlaw_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.15: no profile data available for function _RNvMs2_NtCsjVDvwP8TuwX_8indexmap3setINtB5_8IndexSetNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE4iterCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.15: no profile data available for function _RNvXsb_NtCsjVDvwP8TuwX_8indexmap3setINtB5_4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdENtNtNtNtCsdPxStG2PgiO_4core4iter6traits8iterator8Iterator4nextCsaVf5JQ9vlaw_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.15: no profile data available for function _RNvXsb_NtCsjVDvwP8TuwX_8indexmap3setINtB5_4IterNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIdENtNtNtNtCsdPxStG2PgiO_4core4iter6traits8iterator8Iterator9size_hintCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvMs2_NtNtCs3hdLqqt4OAL_3std6thread5localINtB6_8LocalKeyINtNtCsdPxStG2PgiO_4core4cell4CellPuEE4withNCNvNtNtNtNtCsbsWagiDJuys_12rustc_middle2ty7context3tls3tlv7get_tlv0B1s_ECsaVf5JQ9vlaw_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeINtNtCs9rp5xvMaM5Y_5alloc7raw_vec6RawVechEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtCs9rp5xvMaM5Y_5alloc6string6StringECsaVf5JQ9vlaw_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvNtCsdPxStG2PgiO_4core3ptr13drop_in_placeNtNtNtCs3hdLqqt4OAL_3std6thread5local11AccessErrorECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RINvYINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvMs0_CsjVDvwP8TuwX_8indexmapINtB5_6BucketNtNtCshqfWlKPmSMb_10rustc_span6def_id10LocalDefIduE7key_refCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvXs1_NtCsdPxStG2PgiO_4core7convertINtNtNtB7_3ptr6unique6UniquehEINtB5_4IntoINtNtBD_8non_null7NonNullhEE4intoCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvXs1_NtNtNtCsdPxStG2PgiO_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCshqfWlKPmSMb_10rustc_span6def_id8CrateNumNtNtCsaVf5JQ9vlaw_10rustc_smir10stable_mir5CrateNCNCNvNtB2E_10rustc_smir10find_crate00E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 1147234025963109369 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvXsV_NtCsdPxStG2PgiO_4core3fmtRNtNtCs9rp5xvMaM5Y_5alloc6string6StringNtB5_5Debug3fmtCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvXsV_NtCsdPxStG2PgiO_4core3fmtRbNtB5_5Debug3fmtCsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir14rustc_internal11item_def_id Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.10: no profile data available for function _RNvNtCsaVf5JQ9vlaw_10rustc_smir14rustc_internal9crate_num Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.5: no profile data available for function _RINvNtCsdBlHRorbRmI_9hashbrown3map9make_hashNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexBG_INtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEECsaVf5JQ9vlaw_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.5: no profile data available for function _RINvXs1x_NtCsdBlHRorbRmI_9hashbrown3mapINtB7_7HashMapNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECsaVf5JQ9vlaw_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.63b909ba-cgu.5: no profile data available for function _RNvMs1_NtCsdBlHRorbRmI_9hashbrown3mapINtB5_7HashMapNtNtNtCshydvqb9mfAi_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCsdPxStG2PgiO_4core4hash18BuildHasherDefaultNtCs4bosUpeLYRb_10rustc_hash8FxHasherEE6insertCsaVf5JQ9vlaw_10rustc_smir Hash = 11922953328495428 up to 0 count discarded
[RUSTC-TIMING] rustc_smir test:false 9.177
warning: `rustc_smir` (lib) generated 111 warnings
[RUSTC-TIMING] rustc_transmute test:false 14.429
[RUSTC-TIMING] rustc_metadata test:false 70.723
---
[RUSTC-TIMING] build_script_build test:false 0.142
    Checking home v0.5.4
[RUSTC-TIMING] gix_sec test:false 0.116
   Compiling ucd-trie v0.1.5
error: internal compiler error: /rustc/8ea09f172814926c4c234f649b199f6aa9205307/compiler/rustc_infer/src/infer/outlives/env.rs:145:26: add_outlives_bounds: unexpected regions
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/8ea09f172814926c4c234f649b199f6aa9205307/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7fc55f58750a - std::backtrace_rs::backtrace::libunwind::trace::h6cb6c29587441f8e
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc55f58750a - std::backtrace_rs::backtrace::trace_unsynchronized::h8c5477e383770049
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc55f58750a - std::sys_common::backtrace::_print_fmt::h83f599ed5943481c
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fc55f58750a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8324227ae12c4eff
   4:     0x7fc55f5eab3e - core::fmt::write::h5c56cab042d28558
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/core/src/fmt/mod.rs:1232:17
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/core/src/fmt/mod.rs:1232:17
   5:     0x7fc55f57a375 - std::io::Write::write_fmt::hf4f87c91cf61b6f0
   6:     0x7fc55f5872d5 - std::sys_common::backtrace::_print::h0bdb139796fd67c5
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fc55f5872d5 - std::sys_common::backtrace::print::h4097914cd2148b67
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys_common/backtrace.rs:34:9
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fc55f58a04f - std::panicking::default_hook::{{closure}}::h1d3059499a0be6b5
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/panicking.rs:271:22
   9:     0x7fc55f589d8b - std::panicking::default_hook::h928cd28524aa6750
[RUSTC-TIMING] digest test:false 0.138
    Checking hmac v0.12.1
[RUSTC-TIMING] gix_command test:false 0.054
    Checking sha2 v0.10.6
    Checking sha2 v0.10.6
[RUSTC-TIMING] nom test:false 1.282
    Checking signature v2.0.0
[RUSTC-TIMING] home test:false 0.051
    Checking gix-glob v0.5.5
  10:     0x7fc55e2ae365 - rustc_driver_impl[185cc224cea72581]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fc55f58a88d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h43db6c5edbab66d6
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/alloc/src/boxed.rs:2002:9
  12:     0x7fc55f58a88d - std::panicking::rust_panic_with_hook::h7c9082922d0f7bfc
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/panicking.rs:696:13
  13:     0x7fc55e8134e1 - std[262da2da9c85f821]::panicking::begin_panic::<rustc_errors[3aceb780ccb62908]::ExplicitBug>::{closure#0}
  14:     0x7fc55e80e946 - std[262da2da9c85f821]::sys_common::backtrace::__rust_end_short_backtrace::<std[262da2da9c85f821]::panicking::begin_panic<rustc_errors[3aceb780ccb62908]::ExplicitBug>::{closure#0}, !>
  15:     0x7fc55e80bbf6 - std[262da2da9c85f821]::panicking::begin_panic::<rustc_errors[3aceb780ccb62908]::ExplicitBug>
  16:     0x7fc55e860976 - std[262da2da9c85f821]::panic::panic_any::<rustc_errors[3aceb780ccb62908]::ExplicitBug>
  17:     0x7fc55e85d196 - <rustc_errors[3aceb780ccb62908]::HandlerInner>::bug::<&alloc[6dfa18b13b64e7c4]::string::String>
  18:     0x7fc55e85ce60 - <rustc_errors[3aceb780ccb62908]::Handler>::bug::<&alloc[6dfa18b13b64e7c4]::string::String>
  19:     0x7fc55e84931b - rustc_middle[858f6d7dd951ad9a]::util::bug::opt_span_bug_fmt::<rustc_span[caf13abfe4deac85]::span_encoding::Span>::{closure#0}
  20:     0x7fc55e847c6a - rustc_middle[858f6d7dd951ad9a]::ty::context::tls::with_opt::<rustc_middle[858f6d7dd951ad9a]::util::bug::opt_span_bug_fmt<rustc_span[caf13abfe4deac85]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7fc55e847c36 - rustc_middle[858f6d7dd951ad9a]::ty::context::tls::with_context_opt::<rustc_middle[858f6d7dd951ad9a]::ty::context::tls::with_opt<rustc_middle[858f6d7dd951ad9a]::util::bug::opt_span_bug_fmt<rustc_span[caf13abfe4deac85]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7fc55e849266 - rustc_middle[858f6d7dd951ad9a]::util::bug::opt_span_bug_fmt::<rustc_span[caf13abfe4deac85]::span_encoding::Span>
  23:     0x7fc55c8f2133 - rustc_middle[858f6d7dd951ad9a]::util::bug::bug_fmt
  24:     0x7fc55cac0d45 - <rustc_infer[c3ef9866355676a6]::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core[a11a512dc6886a58]::iter::adapters::flatten::Flatten<core[a11a512dc6886a58]::iter::adapters::map::Map<indexmap[e821fc8f43264d5d]::set::IntoIter<rustc_middle[858f6d7dd951ad9a]::ty::Ty>, <rustc_infer[c3ef9866355676a6]::infer::InferCtxt as rustc_trait_selection[26b52ce1de15822d]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  25:     0x7fc55ce37ce0 - rustc_hir_analysis[d442839d624bc529]::check::compare_impl_item::compare_method_predicate_entailment
  26:     0x7fc55ce31701 - rustc_hir_analysis[d442839d624bc529]::check::compare_impl_item::compare_impl_method
  27:     0x7fc55ce29f3c - rustc_hir_analysis[d442839d624bc529]::check::check::check_impl_items_against_trait
  28:     0x7fc55ce212d8 - rustc_hir_analysis[d442839d624bc529]::check::check::check_mod_item_types
  29:     0x7fc55d8e9a9e - rustc_query_system[cc70169c16cb3288]::query::plumbing::try_execute_query::<rustc_query_impl[7095ed3878df9db6]::queries::check_mod_item_types, rustc_query_impl[7095ed3878df9db6]::plumbing::QueryCtxt>
  30:     0x7fc55d8e9623 - <rustc_query_impl[7095ed3878df9db6]::Queries as rustc_middle[858f6d7dd951ad9a]::ty::query::QueryEngine>::check_mod_item_types
  31:     0x7fc55d95b2dc - <rustc_middle[858f6d7dd951ad9a]::hir::map::Map>::for_each_module::<rustc_hir_analysis[d442839d624bc529]::check_crate::{closure#6}::{closure#0}>
  32:     0x7fc55c5cea78 - rustc_hir_analysis[d442839d624bc529]::check_crate
  33:     0x7fc55c5c6695 - rustc_interface[f786dc321c281ebd]::passes::analysis
  34:     0x7fc55dac25bc - rustc_query_system[cc70169c16cb3288]::query::plumbing::try_execute_query::<rustc_query_impl[7095ed3878df9db6]::queries::analysis, rustc_query_impl[7095ed3878df9db6]::plumbing::QueryCtxt>
  35:     0x7fc55dac22b0 - <rustc_query_impl[7095ed3878df9db6]::Queries as rustc_middle[858f6d7dd951ad9a]::ty::query::QueryEngine>::analysis
  36:     0x7fc55d8ef089 - <rustc_middle[858f6d7dd951ad9a]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>
  37:     0x7fc55d485688 - rustc_span[caf13abfe4deac85]::with_source_map::<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_interface[f786dc321c281ebd]::interface::run_compiler<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7fc55d47d1a0 - <scoped_tls[5aa1769cc2076f03]::ScopedKey<rustc_span[caf13abfe4deac85]::SessionGlobals>>::set::<rustc_interface[f786dc321c281ebd]::interface::run_compiler<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}>::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>
  39:     0x7fc55d47c882 - std[262da2da9c85f821]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f786dc321c281ebd]::util::run_in_thread_pool_with_globals<rustc_interface[f786dc321c281ebd]::interface::run_compiler<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}>::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>
  40:     0x7fc55d47c62a - <<std[262da2da9c85f821]::thread::Builder>::spawn_unchecked_<rustc_interface[f786dc321c281ebd]::util::run_in_thread_pool_with_globals<rustc_interface[f786dc321c281ebd]::interface::run_compiler<core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>, rustc_driver_impl[185cc224cea72581]::run_compiler::{closure#1}>::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a11a512dc6886a58]::result::Result<(), rustc_span[caf13abfe4deac85]::ErrorGuaranteed>>::{closure#1} as core[a11a512dc6886a58]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7fc55f594783 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4e1f4cc398ef09af
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/alloc/src/boxed.rs:1988:9
  42:     0x7fc55f594783 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1cd028674f7cf6f1
  43:     0x7fc55f594783 - std::sys::unix::thread::Thread::new::thread_start::h4af0c18be6dcac5b
                               at /rustc/8ea09f172814926c4c234f649b199f6aa9205307/library/std/src/sys/unix/thread.rs:108:17
  44:     0x7fc55aa23ea5 - start_thread
  45:     0x7fc55a74cb0d - clone
  45:     0x7fc55a74cb0d - clone
  46:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (8ea09f172 2023-03-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [check_mod_item_types] checking item types in module `spki`
#1 [analysis] running analysis passes on this crate
[RUSTC-TIMING] prodash test:false 0.272
[RUSTC-TIMING] ahash test:false 0.177
[RUSTC-TIMING] spki test:false 0.118
error: could not compile `spki`
---
Total duration:                        1h 37m 7s
------------------------------------------------
root INFO: Free disk space: 559.92 GiB out of total 666.61 GiB (16.00% used)
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
