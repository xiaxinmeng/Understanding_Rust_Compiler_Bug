plain
[2023-05-16T21:52:15Z DEBUG collector::execute] cd "/tmp/.tmpGrQTx7" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGrQTx7#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
Running cargo-0.60.0: Debug + [Full]
[2023-05-16T21:53:01Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:53:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T21:53:01Z DEBUG collector::execute] cd "/tmp/.tmpkiLsiz" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkiLsiz#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T21:53:23Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:53:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T21:53:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T21:53:23Z DEBUG collector::execute] cd "/tmp/.tmpooIY9E" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpooIY9E#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/8)
Preparing clap-3.1.6
[2023-05-16T21:54:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T21:54:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T21:54:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T21:54:16Z DEBUG collector::execute] cd "/tmp/.tmp4vfJyg" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4vfJyg#clap@3.1.6" "--" "--skip-this-rustc"
[2023-05-16T21:54:16Z DEBUG collector::execute] cd "/tmp/.tmpYvOKxq" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYvOKxq#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-05-16T21:54:18Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:54:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T21:54:18Z DEBUG collector::execute] cd "/tmp/.tmpkP8cB6" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkP8cB6#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
Running clap-3.1.6: Opt + [Full]
[2023-05-16T21:54:21Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:54:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T21:54:21Z DEBUG collector::execute] cd "/tmp/.tmpnGuTtb" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnGuTtb#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-05-16T21:54:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T21:54:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T21:54:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T21:54:27Z DEBUG collector::execute] cd "/tmp/.tmprC2UwJ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprC2UwJ#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-16T21:54:27Z DEBUG collector::execute] cd "/tmp/.tmp8BKBme" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8BKBme#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
Running hyper-0.14.18: Debug + [Full]
[2023-05-16T21:54:48Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:54:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T21:54:48Z DEBUG collector::execute] cd "/tmp/.tmpsxTE4p" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsxTE4p#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T21:54:50Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:54:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T21:54:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T21:54:50Z DEBUG collector::execute] cd "/tmp/.tmpGqiWaQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGqiWaQ#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-05-16T21:54:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T21:54:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
Finished benchmark ripgrep-13.0.0 (5/8)
Executing benchmark ripgrep-13.0.0-tiny (6/8)
Preparing ripgrep-13.0.0-tiny
[2023-05-16T21:55:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T21:55:48Z DEBUG collector::execute] cd "/tmp/.tmpMdSsjR" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMdSsjR#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-05-16T21:55:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:55:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T21:55:59Z DEBUG collector::execute] cd "/tmp/.tmp17hJv3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp17hJv3#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark ripgrep-13.0.0-tiny (6/8)
Finished benchmark ripgrep-13.0.0-tiny (6/8)
Executing benchmark serde-1.0.136 (7/8)
Preparing serde-1.0.136
[2023-05-16T21:56:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T21:56:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T21:56:34Z DEBUG collector::execute] cd "/tmp/.tmpJekLHN" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJekLHN#serde@1.0.136" "--" "--skip-this-rustc"
[2023-05-16T21:56:34Z DEBUG collector::execute] cd "/tmp/.tmpA5RSjm" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA5RSjm#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-05-16T21:56:35Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:56:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T21:56:35Z DEBUG collector::execute] cd "/tmp/.tmpBjUrQq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBjUrQq#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
Running serde-1.0.136: Opt + [Full]
[2023-05-16T21:56:37Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:56:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T21:56:37Z DEBUG collector::execute] cd "/tmp/.tmpOjcDuw" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOjcDuw#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (8/8)
Preparing syn-1.0.89
[2023-05-16T21:56:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T21:56:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-05-16T21:56:43Z DEBUG collector::execute] cd "/tmp/.tmp9yGb85" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9yGb85#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
[2023-05-16T21:56:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T21:56:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T21:56:45Z DEBUG collector::execute] cd "/tmp/.tmp7zQwNv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7zQwNv#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging LLVM PGO profiles to /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata
stage-build INFO: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata /tmp/tmp-multistage/opt-artifacts/llvm-pgo`
stage-build INFO: LLVM PGO statistics
---
Preparing bitmaps-3.1.0
[2023-05-16T22:08:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:08:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:08:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:08:56Z DEBUG collector::execute] cd "/tmp/.tmpUO6xME" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUO6xME#bitmaps@3.1.0" "--" "--skip-this-rustc"
[2023-05-16T22:08:56Z DEBUG collector::execute] cd "/tmp/.tmpesw5Y0" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpesw5Y0#bitmaps@3.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:08:56Z DEBUG collector::execute] cd "/tmp/.tmpR4L2Sa" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR4L2Sa#bitmaps@3.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:08:56Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:08:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:08:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:08:56Z DEBUG collector::execute] cd "/tmp/.tmpEQzfdp" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEQzfdp#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:08:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:08:57Z DEBUG collector::execute] cd "/tmp/.tmpEQzfdp" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEQzfdp#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEQzfdp/incremental-state"
[2023-05-16T22:08:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:08:59Z DEBUG collector::execute] cd "/tmp/.tmpEQzfdp" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEQzfdp#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEQzfdp/incremental-state"
[2023-05-16T22:08:59Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpEQzfdp"
[2023-05-16T22:08:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:08:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:08:59Z DEBUG collector::execute] cd "/tmp/.tmpEQzfdp" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEQzfdp#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEQzfdp/incremental-state"
[2023-05-16T22:09:00Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:09:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:09:00Z DEBUG collector::execute] cd "/tmp/.tmpSt0B27" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSt0B27#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:09:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-05-16T22:09:03Z DEBUG collector::execute] cd "/tmp/.tmpSt0B27" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSt0B27#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSt0B27/incremental-state"
Running bitmaps-3.1.0: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-05-16T22:09:03Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:09:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:09:03Z DEBUG collector::execute] cd "/tmp/.tmpCEbuzm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCEbuzm#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:09:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-16T22:09:05Z DEBUG collector::execute] cd "/tmp/.tmpCEbuzm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCEbuzm#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCEbuzm/incremental-state"
[2023-05-16T22:09:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:09:07Z DEBUG collector::execute] cd "/tmp/.tmpCEbuzm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCEbuzm#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCEbuzm/incremental-state"
[2023-05-16T22:09:07Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpCEbuzm"
[2023-05-16T22:09:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:09:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:09:07Z DEBUG collector::execute] cd "/tmp/.tmpCEbuzm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCEbuzm#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpCEbuzm/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-05-16T22:09:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:09:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:09:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:09:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:09:08Z DEBUG collector::execute] cd "/tmp/.tmpJ8g6do" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJ8g6do#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-05-16T22:09:08Z DEBUG collector::execute] cd "/tmp/.tmpZMuK50" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZMuK50#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-05-16T22:09:08Z DEBUG collector::execute] cd "/tmp/.tmpxbJe4D" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxbJe4D#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-05-16T22:10:04Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:10:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:10:04Z DEBUG collector::execute] cd "/tmp/.tmpU8e4Ve" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU8e4Ve#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:10:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:10:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:10:16Z DEBUG collector::execute] cd "/tmp/.tmpU8e4Ve" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU8e4Ve#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpU8e4Ve/incremental-state"
[2023-05-16T22:10:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:10:30Z DEBUG collector::execute] cd "/tmp/.tmpU8e4Ve" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU8e4Ve#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpU8e4Ve/incremental-state"
[2023-05-16T22:10:32Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpU8e4Ve"
[2023-05-16T22:10:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:10:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:10:32Z DEBUG collector::execute] cd "/tmp/.tmpU8e4Ve" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU8e4Ve#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpU8e4Ve/incremental-state"
[2023-05-16T22:10:35Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:10:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:10:36Z DEBUG collector::execute] cd "/tmp/.tmp4XdECh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4XdECh#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:11:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-16T22:11:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-16T22:11:05Z DEBUG collector::execute] cd "/tmp/.tmp4XdECh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4XdECh#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4XdECh/incremental-state"
[2023-05-16T22:11:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:11:40Z DEBUG collector::execute] cd "/tmp/.tmp4XdECh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4XdECh#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4XdECh/incremental-state"
[2023-05-16T22:11:46Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmp4XdECh"
[2023-05-16T22:11:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:11:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:11:46Z DEBUG collector::execute] cd "/tmp/.tmp4XdECh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4XdECh#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmp4XdECh/incremental-state"
[2023-05-16T22:11:52Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:11:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:11:52Z DEBUG collector::execute] cd "/tmp/.tmpj5yGLW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj5yGLW#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:12:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-16T22:12:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-16T22:12:33Z DEBUG collector::execute] cd "/tmp/.tmpj5yGLW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj5yGLW#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpj5yGLW/incremental-state"
[2023-05-16T22:13:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:13:14Z DEBUG collector::execute] cd "/tmp/.tmpj5yGLW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj5yGLW#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpj5yGLW/incremental-state"
[2023-05-16T22:13:19Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpj5yGLW"
[2023-05-16T22:13:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:13:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:13:19Z DEBUG collector::execute] cd "/tmp/.tmpj5yGLW" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj5yGLW#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpj5yGLW/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2023-05-16T22:13:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:13:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:13:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:13:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:13:30Z DEBUG collector::execute] cd "/tmp/.tmpLywraQ" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLywraQ#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:13:30Z DEBUG collector::execute] cd "/tmp/.tmpTAuydc" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTAuydc#ctfe-stress-5@0.1.0" "--" "--skip-this-rustc"
[2023-05-16T22:13:30Z DEBUG collector::execute] cd "/tmp/.tmpDNpJSt" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDNpJSt#ctfe-stress-5@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:13:30Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:13:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:13:30Z DEBUG collector::execute] cd "/tmp/.tmpzANPk6" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzANPk6#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:13:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:13:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:13:36Z DEBUG collector::execute] cd "/tmp/.tmpzANPk6" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzANPk6#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzANPk6/incremental-state"
[2023-05-16T22:13:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:13:44Z DEBUG collector::execute] cd "/tmp/.tmpzANPk6" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzANPk6#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzANPk6/incremental-state"
[2023-05-16T22:13:44Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:13:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:13:44Z DEBUG collector::execute] cd "/tmp/.tmp9W7jC3" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9W7jC3#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:13:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-05-16T22:13:57Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:13:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:13:57Z DEBUG collector::execute] cd "/tmp/.tmpeNXofg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeNXofg#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:14:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-16T22:14:03Z DEBUG collector::execute] cd "/tmp/.tmpeNXofg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeNXofg#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeNXofg/incremental-state"
[2023-05-16T22:14:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:14:10Z DEBUG collector::execute] cd "/tmp/.tmpeNXofg" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeNXofg#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpeNXofg/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2023-05-16T22:14:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:14:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:14:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:14:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:14:10Z DEBUG collector::execute] cd "/tmp/.tmplmDhkl" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplmDhkl#diesel@1.4.8" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:14:10Z DEBUG collector::execute] cd "/tmp/.tmpUIppIt" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUIppIt#diesel@1.4.8" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:14:10Z DEBUG collector::execute] cd "/tmp/.tmpjlrf9Z" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjlrf9Z#diesel@1.4.8" "--" "--skip-this-rustc"
[2023-05-16T22:14:21Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:14:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:14:21Z DEBUG collector::execute] cd "/tmp/.tmpqf4gr0" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqf4gr0#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:14:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2023-05-16T22:14:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:14:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:14:45Z DEBUG collector::execute] cd "/tmp/.tmpA3v9ur" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA3v9ur#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:14:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-16T22:14:55Z DEBUG collector::execute] cd "/tmp/.tmpA3v9ur" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA3v9ur#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpA3v9ur/incremental-state"
[2023-05-16T22:15:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:08Z DEBUG collector::execute] cd "/tmp/.tmpA3v9ur" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA3v9ur#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpA3v9ur/incremental-state"
[2023-05-16T22:15:10Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpA3v9ur"
[2023-05-16T22:15:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:15:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:15:10Z DEBUG collector::execute] cd "/tmp/.tmpA3v9ur" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA3v9ur#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpA3v9ur/incremental-state"
[2023-05-16T22:15:13Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:15:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:15:13Z DEBUG collector::execute] cd "/tmp/.tmpKQQCtQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKQQCtQ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:24Z DEBUG collector::execute] cd "/tmp/.tmpKQQCtQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKQQCtQ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKQQCtQ/incremental-state"
[2023-05-16T22:15:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:36Z DEBUG collector::execute] cd "/tmp/.tmpKQQCtQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKQQCtQ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKQQCtQ/incremental-state"
[2023-05-16T22:15:38Z DEBUG collector::benchmark::patch] applying println to "/tmp/.tmpKQQCtQ"
[2023-05-16T22:15:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:15:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-05-16T22:15:38Z DEBUG collector::execute] cd "/tmp/.tmpKQQCtQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKQQCtQ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKQQCtQ/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-05-16T22:15:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:15:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:15:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:15:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:15:41Z DEBUG collector::execute] cd "/tmp/.tmpx1hY1T" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpx1hY1T#externs@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:15:41Z DEBUG collector::execute] cd "/tmp/.tmpuOsrIK" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuOsrIK#externs@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:15:41Z DEBUG collector::execute] cd "/tmp/.tmpjfSTOX" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjfSTOX#externs@0.1.0" "--" "--skip-this-rustc"
[2023-05-16T22:15:41Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:15:41Z DEBUG collector::execute] cd "/tmp/.tmpfODr1V" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfODr1V#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:41Z DEBUG collector::execute] cd "/tmp/.tmpfODr1V" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfODr1V#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfODr1V/incremental-state"
[2023-05-16T22:15:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:42Z DEBUG collector::execute] cd "/tmp/.tmpfODr1V" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfODr1V#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfODr1V/incremental-state"
[2023-05-16T22:15:42Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:15:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:15:42Z DEBUG collector::execute] cd "/tmp/.tmpLrADOH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLrADOH#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:42Z DEBUG collector::execute] cd "/tmp/.tmpLrADOH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLrADOH#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLrADOH/incremental-state"
[2023-05-16T22:15:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:43Z DEBUG collector::execute] cd "/tmp/.tmpLrADOH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLrADOH#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLrADOH/incremental-state"
[2023-05-16T22:15:43Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:15:43Z DEBUG collector::execute] cd "/tmp/.tmpRHaV91" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRHaV91#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Preparing match-stress
[2023-05-16T22:15:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:15:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:15:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:15:45Z DEBUG collector::execute] cd "/tmp/.tmpAKrHnL" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAKrHnL#match-stress@0.1.0" "--" "--skip-this-rustc"
[2023-05-16T22:15:45Z DEBUG collector::execute] cd "/tmp/.tmp6lejVu" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6lejVu#match-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:15:45Z DEBUG collector::execute] cd "/tmp/.tmpUGvzRM" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUGvzRM#match-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:15:45Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:15:45Z DEBUG collector::execute] cd "/tmp/.tmpsj6k7I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsj6k7I#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:46Z DEBUG collector::execute] cd "/tmp/.tmpsj6k7I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsj6k7I#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsj6k7I/incremental-state"
[2023-05-16T22:15:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:48Z DEBUG collector::execute] cd "/tmp/.tmpsj6k7I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsj6k7I#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpsj6k7I/incremental-state"
Running match-stress: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-05-16T22:15:48Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:15:48Z DEBUG collector::execute] cd "/tmp/.tmpxUAIHa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxUAIHa#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:50Z DEBUG collector::execute] cd "/tmp/.tmpxUAIHa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxUAIHa#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxUAIHa/incremental-state"
[2023-05-16T22:15:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:52Z DEBUG collector::execute] cd "/tmp/.tmpxUAIHa" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxUAIHa#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxUAIHa/incremental-state"
[2023-05-16T22:15:52Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:15:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:15:52Z DEBUG collector::execute] cd "/tmp/.tmppRdQaP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppRdQaP#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:54Z DEBUG collector::execute] cd "/tmp/.tmppRdQaP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppRdQaP#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppRdQaP/incremental-state"
[2023-05-16T22:15:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:56Z DEBUG collector::execute] cd "/tmp/.tmppRdQaP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppRdQaP#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppRdQaP/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-05-16T22:15:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:15:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-05-16T22:15:57Z DEBUG collector::execute] cd "/tmp/.tmpX63t0B" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpX63t0B#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpX63t0B/incremental-state"
Running token-stream-stress: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-05-16T22:15:57Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:15:57Z DEBUG collector::execute] cd "/tmp/.tmpohCowq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpohCowq#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:57Z DEBUG collector::execute] cd "/tmp/.tmpohCowq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpohCowq#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpohCowq/incremental-state"
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:58Z DEBUG collector::execute] cd "/tmp/.tmpohCowq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpohCowq#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpohCowq/incremental-state"
[2023-05-16T22:15:58Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:15:58Z DEBUG collector::execute] cd "/tmp/.tmpUMPO5E" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUMPO5E#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-16T22:15:58Z DEBUG collector::execute] cd "/tmp/.tmpUMPO5E" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUMPO5E#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUMPO5E/incremental-state"
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:15:58Z DEBUG collector::execute] cd "/tmp/.tmpUMPO5E" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUMPO5E#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpUMPO5E/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:15:58Z DEBUG collector::execute] cd "/tmp/.tmpYPsDxU" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYPsDxU#tuple-stress@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:15:58Z DEBUG collector::execute] cd "/tmp/.tmpI8o8nL" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpI8o8nL#tuple-stress@0.1.0" "--" "--skip-this-rustc"
[2023-05-16T22:15:58Z DEBUG collector::execute] cd "/tmp/.tmpVTZzPL" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVTZzPL#tuple-stress@0.1.0" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:15:58Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:15:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:15:58Z DEBUG collector::execute] cd "/tmp/.tmpI8IHcQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpI8IHcQ#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:16:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
---
[2023-05-16T22:16:05Z DEBUG collector::execute] cd "/tmp/.tmpI8IHcQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpI8IHcQ#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpI8IHcQ/incremental-state"
Running tuple-stress: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-05-16T22:16:08Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:16:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:16:08Z DEBUG collector::execute] cd "/tmp/.tmpJNUR7t" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJNUR7t#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:16:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-05-16T22:16:11Z DEBUG collector::execute] cd "/tmp/.tmpJNUR7t" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJNUR7t#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJNUR7t/incremental-state"
[2023-05-16T22:16:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:16:14Z DEBUG collector::execute] cd "/tmp/.tmpJNUR7t" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJNUR7t#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJNUR7t/incremental-state"
[2023-05-16T22:16:15Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmpJNUR7t"
[2023-05-16T22:16:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-05-16T22:16:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-05-16T22:16:15Z DEBUG collector::execute] cd "/tmp/.tmpJNUR7t" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJNUR7t#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpJNUR7t/incremental-state"
[2023-05-16T22:16:18Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:16:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:16:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:16:18Z DEBUG collector::execute] cd "/tmp/.tmpoUtyVq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoUtyVq#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:16:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-05-16T22:16:20Z DEBUG collector::execute] cd "/tmp/.tmpoUtyVq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoUtyVq#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoUtyVq/incremental-state"
[2023-05-16T22:16:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-05-16T22:16:24Z DEBUG collector::execute] cd "/tmp/.tmpoUtyVq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoUtyVq#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoUtyVq/incremental-state"
[2023-05-16T22:16:24Z DEBUG collector::benchmark::patch] applying new row to "/tmp/.tmpoUtyVq"
[2023-05-16T22:16:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-05-16T22:16:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-05-16T22:16:24Z DEBUG collector::execute] cd "/tmp/.tmpoUtyVq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoUtyVq#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpoUtyVq/incremental-state"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging Rustc PGO profiles to /tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata
stage-build INFO: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/rustc-pgo.profdata /tmp/tmp-multistage/opt-artifacts/rustc-pgo`
stage-build INFO: Rustc PGO statistics
---
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs9J0sh8Xtzxx_10rustc_span6symbol6SymbolEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtCs1SMo5IIIYUS_9rustc_abi8FieldIdxECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtCs1SMo5IIIYUS_9rustc_abi10VariantIdxECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCs8RWOfvZhALO_12rustc_middle3mir5LocalECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRbECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeyECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRINtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 974670608791895679 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RNvXsm_NtCs45KzRInYOiI_4core3fmtSINtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h11fcf0e7ff41ce1bE Hash = 1124680650125156080 up to 0 count discarded


warning: rustc_smir.5131fa049a35b038-cgu.14: no profile data available for function _RNvMs2_NtCsj9NxzrUAAYd_8indexmap3setINtB5_8IndexSetNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE4iterCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.14: no profile data available for function _RNvXsb_NtCsj9NxzrUAAYd_8indexmap3setINtB5_4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdENtNtNtNtCs45KzRInYOiI_4core4iter6traits8iterator8Iterator4nextCs6YcpMVo69Tg_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.14: no profile data available for function _RNvXsb_NtCsj9NxzrUAAYd_8indexmap3setINtB5_4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdENtNtNtNtCs45KzRInYOiI_4core4iter6traits8iterator8Iterator9size_hintCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeQNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCs9J0sh8Xtzxx_10rustc_span6symbol6SymbolECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRjECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvMNtNtNtCs45KzRInYOiI_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCs6YcpMVo69Tg_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXNtCs45KzRInYOiI_4core3fmtQNtNtCsbMYL7gbozeQ_5alloc6string6StringNtB2_5Write10write_charCs6YcpMVo69Tg_10rustc_smir Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXNtCs45KzRInYOiI_4core3fmtQNtNtCsbMYL7gbozeQ_5alloc6string6StringNtB2_5Write9write_fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXNtCs45KzRInYOiI_4core3fmtQNtNtCsbMYL7gbozeQ_5alloc6string6StringNtB2_5Write9write_strCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsM_NtCs45KzRInYOiI_4core6optionINtB5_6OptionNtNtCs9J0sh8Xtzxx_10rustc_span6symbol6SymbolENtNtB7_3fmt5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsM_NtCs45KzRInYOiI_4core6optionINtB5_6OptionjENtNtB7_3fmt5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCsbMYL7gbozeQ_5alloc6string6StringNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyNtB5_5Debug3fmtBB_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsm_NtCs45KzRInYOiI_4core3fmtSNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyNtB5_5Debug3fmtBB_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvYNtNtCsbMYL7gbozeQ_5alloc6string6StringNtNtCs45KzRInYOiI_4core3fmt5Write9write_fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvMNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2tyNtB2_2Ty4kind Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXs1_NtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2tyNtB5_2TyNtNtCs45KzRInYOiI_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNtCsbMYL7gbozeQ_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECs6YcpMVo69Tg_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyNtNtB9_5alloc6GlobalEB1s_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handlehNtNtB9_5alloc6GlobalECs6YcpMVo69Tg_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs0_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVechE14grow_amortizedCs6YcpMVo69Tg_10rustc_smir Hash = 515675264369571770 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyE16reserve_for_pushCs6YcpMVo69Tg_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdE16reserve_for_pushCs6YcpMVo69Tg_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyE11allocate_inBR_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVechE11allocate_inCs6YcpMVo69Tg_10rustc_smir Hash = 1096621587513427694 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVechE16reserve_for_pushCs6YcpMVo69Tg_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBQ_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBQ_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBS_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVechENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropCs6YcpMVo69Tg_10rustc_smir Hash = 134732430909126014 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtNtCsh3nXoKZtS7v_3std11collections4hash3map7HashMapNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdTINtNtNtCs8RWOfvZhALO_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtB4_4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEEEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtB4_3fmt5ErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtNtCs8RWOfvZhALO_12rustc_middle2ty5query12query_get_atINtNtNtCs42j8y7zXj0q_18rustc_query_system5query6caches11SingleCacheINtNtNtB6_5query5erase6ErasedAhj10_EEECs6YcpMVo69Tg_10rustc_smir Hash = 903233818342430900 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXs2_NtNtNtCs45KzRInYOiI_4core3ops8function5implsQNCNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir30rustc_terminator_to_terminator0INtB7_6FnOnceTToNtNtCs8RWOfvZhALO_12rustc_middle3mir10BasicBlockEEE9call_onceBU_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXs2_NtNtNtCs45KzRInYOiI_4core3ops8function5implsQNCNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtBT_6TablesNtNtBV_10stable_mir7Context15all_local_items0INtB7_6FnOnceTRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdEE9call_onceBV_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17ha8a93a57159c00d8E Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17hd2923901f29878fcE Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvMNtCs6YcpMVo69Tg_10rustc_smir14rustc_internalNtNtB4_10rustc_smir6Tables11item_def_id Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvMNtCs6YcpMVo69Tg_10rustc_smir14rustc_internalNtNtB4_10rustc_smir6Tables10crate_item Hash = 11922954461437676 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context10find_crate Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context15all_local_items Hash = 995382920162496885 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context8entry_fn Hash = 18663487788228108 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context8mir_body Hash = 668285657691964329 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context12rustc_tables Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context7ty_kind Hash = 1117982120138886448 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvMs_NtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB4_6Tables9intern_ty Hash = 287486625014882487 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir10smir_crate Hash = 828027342165498051 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir28rustc_statement_to_statement Hash = 90653978435007901 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir14rustc_op_to_op Hash = 650973719845048549 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir22rustc_bin_op_to_bin_op Hash = 393568427803840120 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir30rustc_terminator_to_terminator Hash = 471086160746802464 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvMs5_NtCs2oODm8Op2Ir_9hashbrown3rawINtB6_8RawTableTNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_uINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE0ECs6YcpMVo69Tg_10rustc_smir Hash = 654994090400369401 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvMs5_NtCs2oODm8Op2Ir_9hashbrown3rawINtB6_8RawTableTNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_uINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE0ECs6YcpMVo69Tg_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvMs5_NtCs2oODm8Op2Ir_9hashbrown3rawINtB6_8RawTableTNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_uINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE0ECs6YcpMVo69Tg_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCs2oODm8Op2Ir_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCsbMYL7gbozeQ_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECs6YcpMVo69Tg_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCs2oODm8Op2Ir_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCsbMYL7gbozeQ_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECs6YcpMVo69Tg_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEEB1j_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEBO_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 567991000227110385 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body13AssertMessageEBO_ Hash = 647472252099359144 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body6RvalueEBO_ Hash = 1096621588281264899 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementEBO_ Hash = 844982797524240697 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvMs0_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VechE17extend_from_sliceCs6YcpMVo69Tg_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumENCNvXNtBY_10rustc_smirNtB49_6TablesNtBW_7Context15external_crates0EE9from_iterBY_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtCsj9NxzrUAAYd_8indexmap3set4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdENCNvXNtBY_10rustc_smirNtB4q_6TablesNtBW_7Context15all_local_items0EE9from_iterBY_ Hash = 303685787496864384 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtB2b_6copied6CopiedINtNtNtB2f_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyEENCNvMs_NtB10_10rustc_smirNtB4t_6Tables14rustc_ty_to_ty0EE9from_iterB10_ Hash = 414921810136842226 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2f_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir9LocalDeclENCNvXNtB10_10rustc_smirNtB4b_6TablesNtBY_7Context8mir_bodys_0EE9from_iterB10_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2w_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir14BasicBlockDataENCNvXNtB12_10rustc_smirNtB4y_6TablesNtB10_7Context8mir_body0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapNtNtNtCs8RWOfvZhALO_12rustc_middle3mir10terminator17SwitchTargetsIterNCNvNtB12_10rustc_smir30rustc_terminator_to_terminator0EE9from_iterB12_ Hash = 141929913395094594 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2s_5slice4iter4IterNtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax7OperandENCNvNtB12_10rustc_smir30rustc_terminator_to_terminators_0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2u_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir9StatementENvNtB12_10rustc_smir28rustc_statement_to_statementEE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyENtB5_5Debug3fmtB18_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBJ_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBJ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBL_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VechENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsp_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtCs45KzRInYOiI_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsp_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtCs45KzRInYOiI_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsp_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandENtNtCs45KzRInYOiI_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsp_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementENtNtCs45KzRInYOiI_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvMs2_NtNtCsh3nXoKZtS7v_3std6thread5localINtB6_8LocalKeyINtNtCs45KzRInYOiI_4core4cell4CellOuEE15initialize_withNCNvMs3_B6_BF_3set0uECs6YcpMVo69Tg_10rustc_smir Hash = 811594942473831488 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCs8RWOfvZhALO_12rustc_middle9dep_graph8dep_node7DepKindEEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtCsh3nXoKZtS7v_3std6thread5local11AccessErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtNtNtCs2oODm8Op2Ir_9hashbrown3raw5alloc5inner8do_allocNtNtCsbMYL7gbozeQ_5alloc5alloc6GlobalECs6YcpMVo69Tg_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvXNtCs8RWOfvZhALO_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECs6YcpMVo69Tg_10rustc_smir Hash = 269792483359258493 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvXs8_NtCs9J0sh8Xtzxx_10rustc_span6def_idNtB6_5DefIdNtNtCs45KzRInYOiI_4core4hash4Hash4hashNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvMs0_Csj9NxzrUAAYd_8indexmapINtB5_6BucketNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIduE7key_refCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvMs3_NtNtCsh3nXoKZtS7v_3std6thread5localINtB5_8LocalKeyINtNtCs45KzRInYOiI_4core4cell4CellOuEE7replaceCs6YcpMVo69Tg_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXCs2oODm8Op2Ir_9hashbrownNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdINtB2_10EquivalentBq_E10equivalentCs6YcpMVo69Tg_10rustc_smir Hash = 382993475055910911 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXs1_NtNtNtCs45KzRInYOiI_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateNCNvXNtB2E_10rustc_smirNtB3r_6TablesNtB2C_7Context10find_crate0E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 505312707234972298 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtCs1SMo5IIIYUS_9rustc_abi10VariantIdxNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCs8RWOfvZhALO_12rustc_middle3mir5LocalNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCs9J0sh8Xtzxx_10rustc_span6symbol6SymbolNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRbNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRjNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsu_NtCsapyAhDdZmw1_12tracing_core5fieldINtB5_10DebugValueRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumENtB5_5Value6recordCs6YcpMVo69Tg_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsu_NtCsapyAhDdZmw1_12tracing_core5fieldINtB5_10DebugValueRNtNtCsbMYL7gbozeQ_5alloc6string6StringENtB5_5Value6recordCs6YcpMVo69Tg_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsx_NtCsbMYL7gbozeQ_5alloc5boxedINtB5_3BoxNtNtCs8RWOfvZhALO_12rustc_middle3mir8ConstantENtNtCs45KzRInYOiI_4core3fmt7Display3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.9: no profile data available for function _RINvMsz_NtCs2oODm8Op2Ir_9hashbrown3mapINtB6_15RawEntryBuilderNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdTINtNtNtCs8RWOfvZhALO_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE6searchNCINvB6_10equivalentBX_BX_E0ECs6YcpMVo69Tg_10rustc_smir Hash = 910108394528351925 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.9: no profile data available for function _RINvXs1x_NtCs2oODm8Op2Ir_9hashbrown3mapINtB7_7HashMapNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECs6YcpMVo69Tg_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.9: no profile data available for function _RNvMs1_NtCs2oODm8Op2Ir_9hashbrown3mapINtB5_7HashMapNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE6insertCs6YcpMVo69Tg_10rustc_smir Hash = 11922956408974369 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB16_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEEB1j_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEEB1q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtBP_10stable_mir9CrateItemNCNvBN_10crate_item0E00EBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdNCNvBN_11item_def_id0E00EBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty6TyKindEBM_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body4BodyEBO_ Hash = 1096621589765811986 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRbECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRjECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placejECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNSNvYNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtBd_10stable_mir9CrateItemNCNvBb_10crate_item0E00INtNtNtCs45KzRInYOiI_4core3ops8function6FnOnceTQNtNtBd_10rustc_smir6TablesEE9call_once6vtableBd_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNSNvYNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdNCNvBb_11item_def_id0E00INtNtNtCs45KzRInYOiI_4core3ops8function6FnOnceTQNtNtBd_10rustc_smir6TablesEE9call_once6vtableBd_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb19bfaeaf1194325E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h65a613c241761a63E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtB8_10stable_mir9CrateItemNCNvB6_10crate_item0E00B8_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdNCNvB6_11item_def_id0E00B8_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvMNtCs6YcpMVo69Tg_10rustc_smir10stable_mirNtB2_9CrateItem4body Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir8entry_fn Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir11local_crate Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir10find_crate Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir15external_crates Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir15all_local_items Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir4withNtNtB2_2ty6TyKindNCNvMBN_NtBN_2Ty4kind0EB4_ Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir4withuNCINvNtB4_14rustc_internal11with_tablesNtB2_9CrateItemNCNvBR_10crate_item0E0EB4_ Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir4withuNCINvNtB4_14rustc_internal11with_tablesNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdNCNvBR_11item_def_id0E0EB4_ Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvXs4_NtCs6YcpMVo69Tg_10rustc_smir10stable_mirNtB5_5CrateNtNtCs45KzRInYOiI_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvXsa_NtCs6YcpMVo69Tg_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCs45KzRInYOiI_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNvNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3TLV7___getit7destroy Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs8RWOfvZhALO_12rustc_middle3mir14BasicBlockDataNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockuNCNvXNtB2k_10rustc_smirNtB3n_6TablesNtB2i_7Context8mir_body0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2c_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5m_3VecB2c_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3i_EE0E0E0EB2k_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs8RWOfvZhALO_12rustc_middle3mir9LocalDeclNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyuNCNvXNtB2c_10rustc_smirNtB30_6TablesNtB2a_7Context8mir_bodys_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB51_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2V_EE0E0E0EB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs8RWOfvZhALO_12rustc_middle3mir9StatementNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementuNvNtB2e_10rustc_smir28rustc_statement_to_statementNCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB54_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3a_EE0E0E0EB2e_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateuNCNvXNtB2a_10rustc_smirNtB2Y_6TablesNtB28_7Context15external_crates0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB55_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax7OperandNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperanduNCNvNtB2l_10rustc_smir30rustc_terminator_to_terminators_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2d_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5g_3VecB2d_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3f_EE0E0E0EB2l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 567991000227110385 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body13AssertMessageEBO_ Hash = 647472252099359144 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyEENCNvMs_NtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2u_6Tables14rustc_ty_to_ty0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3D_8for_each4callNtNtNtB2w_10stable_mir2ty2TyNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5h_3VecB4G_E14extend_trustedBN_E0E0EB2w_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCs2oODm8Op2Ir_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir14BasicBlockDataENCNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2j_6TablesNtNtB2l_10stable_mir7Context8mir_body0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3N_8for_each4callNtNtNtB3c_3mir4body10BasicBlockNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5u_3VecB4Q_E14extend_trustedBN_E0E0EB2l_ Hash = 650973720849546769 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir9LocalDeclENCNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2d_6TablesNtNtB2f_10stable_mir7Context8mir_bodys_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3J_8for_each4callNtNtB36_2ty2TyNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB59_3VecB4M_E14extend_trustedBN_E0E0EB2f_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir9StatementENvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir28rustc_statement_to_statementENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3j_8for_each4callNtNtNtNtB2c_10stable_mir3mir4body9StatementNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5c_3VecB4m_E14extend_trustedBN_E0E0EB2c_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumENCNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2d_6TablesNtNtB2f_10stable_mir7Context15external_crates0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3P_8for_each4callNtB36_5CrateNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5d_3VecB4S_E14extend_trustedBN_E0E0EB2f_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax7OperandENCNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir30rustc_terminator_to_terminators_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3x_8for_each4callNtNtNtNtB2l_10stable_mir3mir4body7OperandNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5o_3VecB4A_E14extend_trustedBN_E0E0EB2l_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters6copied9copy_foldNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyuNCINvNtBN_3map8map_foldB1p_NtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyuNCNvMs_NtB2z_10rustc_smirNtB3p_6Tables14rustc_ty_to_ty0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2t_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5h_3VecB2t_E14extend_trustedINtB27_3MapINtBL_6CopiedINtNtNtB4_5slice4iter4IterB1p_EEB3i_EE0E0E0E0EB2z_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RINvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_NtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyuNCNvMs_NtB3h_10rustc_smirNtB47_6Tables14rustc_ty_to_ty0NCINvNvB26_8for_each4callB3b_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5v_3VecB3b_E14extend_trustedINtB2P_3MapBP_B40_EE0E0E0EB3h_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RINvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCs2oODm8Op2Ir_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECs6YcpMVo69Tg_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvXs3_NtNtCs45KzRInYOiI_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6YcpMVo69Tg_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyEENtNtNtB8_6traits8iterator8Iterator9size_hintCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11item_def_id Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal10crate_item Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal9crate_num Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateNCNvXNtB3x_10rustc_smirNtB4k_6TablesNtB3v_7Context10find_crate0E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 629592967359852085 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvYINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RNvXCs2oODm8Op2Ir_9hashbrownNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_10EquivalentBq_E10equivalentCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRINtNtNtCs8RWOfvZhALO_12rustc_middle2ty4list4ListINtNtNtBC_3mir6syntax14ProjectionElemNtB1m_5LocalNtBA_2TyEENtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRINtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax14ProjectionElemNtB19_5LocalNtNtB1b_2ty2TyEINtNtNtBa_5slice4iter4IterB14_EECs6YcpMVo69Tg_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyINtNtNtBa_5slice4iter4IterB14_EEB1a_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRINtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax14ProjectionElemNtBM_5LocalNtNtBO_2ty2TyEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEBN_ Hash = 742261418966908927 up to 0 count discarded

---
Preparing cargo-0.60.0
[2023-05-16T22:45:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:45:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:45:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:45:42Z DEBUG collector::execute] cd "/tmp/.tmpRAa3Oq" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRAa3Oq#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-05-16T22:45:42Z DEBUG collector::execute] cd "/tmp/.tmpneDAJd" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpneDAJd#cargo@0.60.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2023-05-16T22:45:42Z DEBUG collector::execute] cd "/tmp/.tmpFFUwsm" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFFUwsm#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-05-16T22:46:48Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:46:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:46:48Z DEBUG collector::execute] cd "/tmp/.tmpZAo4TX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZAo4TX#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Debug + [Full]
Running cargo-0.60.0: Debug + [Full]
[2023-05-16T22:46:53Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:46:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:46:53Z DEBUG collector::execute] cd "/tmp/.tmpA2rE8p" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA2rE8p#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Running cargo-0.60.0: Opt + [Full]
[2023-05-16T22:47:10Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:47:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:47:10Z DEBUG collector::execute] cd "/tmp/.tmpZsxMjF" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZsxMjF#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/8)
Preparing clap-3.1.6
[2023-05-16T22:47:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:47:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:47:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:47:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:47:48Z DEBUG collector::execute] cd "/tmp/.tmpBALNHc" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBALNHc#clap@3.1.6" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:47:48Z DEBUG collector::execute] cd "/tmp/.tmpG4jvXp" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG4jvXp#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:47:48Z DEBUG collector::execute] cd "/tmp/.tmpGoJ8vw" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpGoJ8vw#clap@3.1.6" "--" "--skip-this-rustc"
[2023-05-16T22:47:53Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:47:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:47:53Z DEBUG collector::execute] cd "/tmp/.tmpk5tRX5" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpk5tRX5#clap@3.1.6" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Debug + [Full]
Running clap-3.1.6: Debug + [Full]
[2023-05-16T22:47:54Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:47:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:47:54Z DEBUG collector::execute] cd "/tmp/.tmpEC56TL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEC56TL#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
[2023-05-16T22:47:57Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:47:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:47:57Z DEBUG collector::execute] cd "/tmp/.tmprshrjD" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprshrjD#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-05-16T22:48:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:48:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:48:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:48:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:48:04Z DEBUG collector::execute] cd "/tmp/.tmpgLLoRC" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgLLoRC#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-16T22:48:04Z DEBUG collector::execute] cd "/tmp/.tmpSZ9ZdB" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSZ9ZdB#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-16T22:48:04Z DEBUG collector::execute] cd "/tmp/.tmptUWWsO" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptUWWsO#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-16T22:48:28Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:48:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:48:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:48:29Z DEBUG collector::execute] cd "/tmp/.tmpvWg8lA" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvWg8lA#hyper@0.14.18" "--profile" "check" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:48:30Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:48:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:48:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:48:30Z DEBUG collector::execute] cd "/tmp/.tmpYMNHAH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYMNHAH#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:48:33Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:48:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-16T22:48:33Z DEBUG collector::execute] cd "/tmp/.tmp03UxN5" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp03UxN5#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/8)
---
Preparing ripgrep-13.0.0
[2023-05-16T22:48:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:48:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:48:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:48:57Z DEBUG collector::execute] cd "/tmp/.tmpsQAeYF" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsQAeYF#ripgrep@13.0.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:48:57Z DEBUG collector::execute] cd "/tmp/.tmpBxAZQv" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBxAZQv#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-05-16T22:48:57Z DEBUG collector::execute] cd "/tmp/.tmpmikXpC" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmikXpC#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:49:29Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:49:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:49:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:49:30Z DEBUG collector::execute] cd "/tmp/.tmpaewgot" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaewgot#ripgrep@13.0.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:49:30Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:49:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:49:30Z DEBUG collector::execute] cd "/tmp/.tmpjlEF1y" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjlEF1y#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
---
Preparing serde-1.0.136
[2023-05-16T22:50:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:50:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:50:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:50:26Z DEBUG collector::execute] cd "/tmp/.tmp7GVLtR" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7GVLtR#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:50:26Z DEBUG collector::execute] cd "/tmp/.tmpFkvLRH" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFkvLRH#serde@1.0.136" "--" "--skip-this-rustc"
[2023-05-16T22:50:26Z DEBUG collector::execute] cd "/tmp/.tmpKBQN8S" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKBQN8S#serde@1.0.136" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:50:27Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:50:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:50:27Z DEBUG collector::execute] cd "/tmp/.tmpYPhQwJ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYPhQwJ#serde@1.0.136" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Debug + [Full]
---
Preparing syn-1.0.89
[2023-05-16T22:50:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-05-16T22:50:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-16T22:50:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-16T22:50:35Z DEBUG collector::execute] cd "/tmp/.tmplBCnFe" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplBCnFe#syn@1.0.89" "--" "--skip-this-rustc"
[2023-05-16T22:50:35Z DEBUG collector::execute] cd "/tmp/.tmp8YxcgE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8YxcgE#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-05-16T22:50:35Z DEBUG collector::execute] cd "/tmp/.tmpOseOJW" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOseOJW#syn@1.0.89" "--profile" "check" "--" "--skip-this-rustc"
[2023-05-16T22:50:39Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:50:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:50:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-05-16T22:50:39Z DEBUG collector::execute] cd "/tmp/.tmpSBBJGy" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSBBJGy#syn@1.0.89" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-16T22:50:40Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-16T22:50:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-16T22:50:40Z DEBUG collector::execute] cd "/tmp/.tmpWaace9" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWaace9#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
---
[RUSTC-TIMING] rustc_symbol_mangling test:false 5.377
[RUSTC-TIMING] rustc_ast_lowering test:false 23.813
[RUSTC-TIMING] rustc_monomorphize test:false 9.940
[RUSTC-TIMING] rustc_query_impl test:false 72.571
warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs9J0sh8Xtzxx_10rustc_span6symbol6SymbolEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtCs1SMo5IIIYUS_9rustc_abi8FieldIdxECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtCs1SMo5IIIYUS_9rustc_abi10VariantIdxECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCs8RWOfvZhALO_12rustc_middle3mir5LocalECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRbECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeyECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRINtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 974670608791895679 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _RNvXsm_NtCs45KzRInYOiI_4core3fmtSINtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax14ProjectionElemNtBA_5LocalNtNtBC_2ty2TyENtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.5131fa049a35b038-cgu.10: no profile data available for function _ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17h11fcf0e7ff41ce1bE Hash = 1124680650125156080 up to 0 count discarded


warning: rustc_smir.5131fa049a35b038-cgu.14: no profile data available for function _RNvMs2_NtCsj9NxzrUAAYd_8indexmap3setINtB5_8IndexSetNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE4iterCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.14: no profile data available for function _RNvXsb_NtCsj9NxzrUAAYd_8indexmap3setINtB5_4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdENtNtNtNtCs45KzRInYOiI_4core4iter6traits8iterator8Iterator4nextCs6YcpMVo69Tg_10rustc_smir Hash = 238984482353853237 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.14: no profile data available for function _RNvXsb_NtCsj9NxzrUAAYd_8indexmap3setINtB5_4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdENtNtNtNtCs45KzRInYOiI_4core4iter6traits8iterator8Iterator9size_hintCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeQNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCs9J0sh8Xtzxx_10rustc_span6symbol6SymbolECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRjECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvMNtNtNtCs45KzRInYOiI_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangejEE3newCs6YcpMVo69Tg_10rustc_smir Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXNtCs45KzRInYOiI_4core3fmtQNtNtCsbMYL7gbozeQ_5alloc6string6StringNtB2_5Write10write_charCs6YcpMVo69Tg_10rustc_smir Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXNtCs45KzRInYOiI_4core3fmtQNtNtCsbMYL7gbozeQ_5alloc6string6StringNtB2_5Write9write_fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXNtCs45KzRInYOiI_4core3fmtQNtNtCsbMYL7gbozeQ_5alloc6string6StringNtB2_5Write9write_strCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsM_NtCs45KzRInYOiI_4core6optionINtB5_6OptionNtNtCs9J0sh8Xtzxx_10rustc_span6symbol6SymbolENtNtB7_3fmt5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsM_NtCs45KzRInYOiI_4core6optionINtB5_6OptionjENtNtB7_3fmt5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCsbMYL7gbozeQ_5alloc6string6StringNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyNtB5_5Debug3fmtBB_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXsm_NtCs45KzRInYOiI_4core3fmtSNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyNtB5_5Debug3fmtBB_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvYNtNtCsbMYL7gbozeQ_5alloc6string6StringNtNtCs45KzRInYOiI_4core3fmt5Write9write_fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvMNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2tyNtB2_2Ty4kind Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.13: no profile data available for function _RNvXs1_NtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2tyNtB5_2TyNtNtCs45KzRInYOiI_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNtCsbMYL7gbozeQ_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECs6YcpMVo69Tg_10rustc_smir Hash = 599005163245366147 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemNtNtB9_5alloc6GlobalEB1q_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyNtNtB9_5alloc6GlobalEB1s_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handleNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementNtNtB9_5alloc6GlobalEB1u_ Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RINvNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handlehNtNtB9_5alloc6GlobalECs6YcpMVo69Tg_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs0_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVechE14grow_amortizedCs6YcpMVo69Tg_10rustc_smir Hash = 515675264369571770 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemE11allocate_inBP_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyE16reserve_for_pushCs6YcpMVo69Tg_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdE16reserve_for_pushCs6YcpMVo69Tg_10rustc_smir Hash = 117630494571946737 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyE11allocate_inBR_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementE11allocate_inBT_ Hash = 783548776065062695 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVechE11allocate_inCs6YcpMVo69Tg_10rustc_smir Hash = 1096621587513427694 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvMs_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB4_6RawVechE16reserve_for_pushCs6YcpMVo69Tg_10rustc_smir Hash = 134732430337390523 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBQ_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBQ_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBS_ Hash = 167471838997577442 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBU_ Hash = 959085192517320418 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.0: no profile data available for function _RNvXs1_NtCsbMYL7gbozeQ_5alloc7raw_vecINtB5_6RawVechENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropCs6YcpMVo69Tg_10rustc_smir Hash = 134732430909126014 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtNtCsh3nXoKZtS7v_3std11collections4hash3map7HashMapNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdTINtNtNtCs8RWOfvZhALO_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtB4_4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEEEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtB4_3fmt5ErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RINvNtNtCs8RWOfvZhALO_12rustc_middle2ty5query12query_get_atINtNtNtCs42j8y7zXj0q_18rustc_query_system5query6caches11SingleCacheINtNtNtB6_5query5erase6ErasedAhj10_EEECs6YcpMVo69Tg_10rustc_smir Hash = 903233818342430900 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXs2_NtNtNtCs45KzRInYOiI_4core3ops8function5implsQNCNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir30rustc_terminator_to_terminator0INtB7_6FnOnceTToNtNtCs8RWOfvZhALO_12rustc_middle3mir10BasicBlockEEE9call_onceBU_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXs2_NtNtNtCs45KzRInYOiI_4core3ops8function5implsQNCNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtBT_6TablesNtNtBV_10stable_mir7Context15all_local_items0INtB7_6FnOnceTRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdEE9call_onceBV_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17ha8a93a57159c00d8E Hash = 5832819706010142 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17hd2923901f29878fcE Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvMNtCs6YcpMVo69Tg_10rustc_smir14rustc_internalNtNtB4_10rustc_smir6Tables11item_def_id Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvMNtCs6YcpMVo69Tg_10rustc_smir14rustc_internalNtNtB4_10rustc_smir6Tables10crate_item Hash = 11922954461437676 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context11local_crate Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context15external_crates Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context10find_crate Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context15all_local_items Hash = 995382920162496885 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context8entry_fn Hash = 18663487788228108 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context8mir_body Hash = 668285657691964329 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context12rustc_tables Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2_6TablesNtNtB4_10stable_mir7Context7ty_kind Hash = 1117982120138886448 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvMs_NtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB4_6Tables9intern_ty Hash = 287486625014882487 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir10smir_crate Hash = 828027342165498051 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir28rustc_statement_to_statement Hash = 90653978435007901 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir14rustc_op_to_op Hash = 650973719845048549 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir22rustc_bin_op_to_bin_op Hash = 393568427803840120 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.2: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir30rustc_terminator_to_terminator Hash = 471086160746802464 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvMs5_NtCs2oODm8Op2Ir_9hashbrown3rawINtB6_8RawTableTNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_uINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE0ECs6YcpMVo69Tg_10rustc_smir Hash = 654994090400369401 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvMs5_NtCs2oODm8Op2Ir_9hashbrown3rawINtB6_8RawTableTNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE6insertNCINvNtB8_3map11make_hasherBQ_uINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE0ECs6YcpMVo69Tg_10rustc_smir Hash = 1011877329365145017 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvMs5_NtCs2oODm8Op2Ir_9hashbrown3rawINtB6_8RawTableTNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuEE7reserveNCINvNtB8_3map11make_hasherBQ_uINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE0ECs6YcpMVo69Tg_10rustc_smir Hash = 784007059655560962 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCs2oODm8Op2Ir_9hashbrown10scopeguard10ScopeGuardINtNtBL_3raw13RawTableInnerNtNtCsbMYL7gbozeQ_5alloc5alloc6GlobalENCNvMs9_B1A_B1x_14prepare_resize0EECs6YcpMVo69Tg_10rustc_smir Hash = 1096621590593879847 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.5: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCs2oODm8Op2Ir_9hashbrown10scopeguard10ScopeGuardQINtNtBL_3raw13RawTableInnerNtNtCsbMYL7gbozeQ_5alloc5alloc6GlobalENCNvMs9_B1B_B1y_15rehash_in_place0EECs6YcpMVo69Tg_10rustc_smir Hash = 915934320037209430 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEEB1j_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEBO_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 567991000227110385 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body13AssertMessageEBO_ Hash = 647472252099359144 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body6RvalueEBO_ Hash = 1096621588281264899 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementEBO_ Hash = 844982797524240697 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvMs0_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VechE17extend_from_sliceCs6YcpMVo69Tg_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2d_5slice4iter4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumENCNvXNtBY_10rustc_smirNtB49_6TablesNtBW_7Context15external_crates0EE9from_iterBY_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtCsj9NxzrUAAYd_8indexmap3set4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIdENCNvXNtBY_10rustc_smirNtB4q_6TablesNtBW_7Context15all_local_items0EE9from_iterBY_ Hash = 303685787496864384 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtB2b_6copied6CopiedINtNtNtB2f_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyEENCNvMs_NtB10_10rustc_smirNtB4t_6Tables14rustc_ty_to_ty0EE9from_iterB10_ Hash = 414921810136842226 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2f_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir9LocalDeclENCNvXNtB10_10rustc_smirNtB4b_6TablesNtBY_7Context8mir_bodys_0EE9from_iterB10_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2w_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir14BasicBlockDataENCNvXNtB12_10rustc_smirNtB4y_6TablesNtB10_7Context8mir_body0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapNtNtNtCs8RWOfvZhALO_12rustc_middle3mir10terminator17SwitchTargetsIterNCNvNtB12_10rustc_smir30rustc_terminator_to_terminator0EE9from_iterB12_ Hash = 141929913395094594 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2s_5slice4iter4IterNtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax7OperandENCNvNtB12_10rustc_smir30rustc_terminator_to_terminators_0EE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXNtNtCsbMYL7gbozeQ_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementEINtB2_12SpecFromIterBU_INtNtNtNtCs45KzRInYOiI_4core4iter8adapters3map3MapINtNtNtB2u_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir9StatementENvNtB12_10rustc_smir28rustc_statement_to_statementEE9from_iterB12_ Hash = 798733567901971782 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyENtB5_5Debug3fmtB18_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBJ_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBJ_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBL_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropBN_ Hash = 1096621590113966630 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsn_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VechENtNtNtCs45KzRInYOiI_4core3ops4drop4Drop4dropCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsp_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockENtNtCs45KzRInYOiI_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsp_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetENtNtCs45KzRInYOiI_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsp_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandENtNtCs45KzRInYOiI_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.1: no profile data available for function _RNvXsp_NtCsbMYL7gbozeQ_5alloc3vecINtB5_3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementENtNtCs45KzRInYOiI_4core3fmt5Debug3fmtBN_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvMs2_NtNtCsh3nXoKZtS7v_3std6thread5localINtB6_8LocalKeyINtNtCs45KzRInYOiI_4core4cell4CellOuEE15initialize_withNCNvMs3_B6_BF_3set0uECs6YcpMVo69Tg_10rustc_smir Hash = 811594942473831488 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtB4_4cell6RefMutINtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph8TaskDepsNtNtNtCs8RWOfvZhALO_12rustc_middle9dep_graph8dep_node7DepKindEEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtCsh3nXoKZtS7v_3std6thread5local11AccessErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvNtNtNtCs2oODm8Op2Ir_9hashbrown3raw5alloc5inner8do_allocNtNtCsbMYL7gbozeQ_5alloc5alloc6GlobalECs6YcpMVo69Tg_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvXNtCs8RWOfvZhALO_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph7DepKind9read_depsNCNvMs3_NtB17_5graphINtB2h_8DepGraphBH_E10read_index0ECs6YcpMVo69Tg_10rustc_smir Hash = 269792483359258493 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RINvXs8_NtCs9J0sh8Xtzxx_10rustc_span6def_idNtB6_5DefIdNtNtCs45KzRInYOiI_4core4hash4Hash4hashNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvMs0_Csj9NxzrUAAYd_8indexmapINtB5_6BucketNtNtCs9J0sh8Xtzxx_10rustc_span6def_id10LocalDefIduE7key_refCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvMs3_NtNtCsh3nXoKZtS7v_3std6thread5localINtB5_8LocalKeyINtNtCs45KzRInYOiI_4core4cell4CellOuEE7replaceCs6YcpMVo69Tg_10rustc_smir Hash = 811594940139423323 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXCs2oODm8Op2Ir_9hashbrownNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdINtB2_10EquivalentBq_E10equivalentCs6YcpMVo69Tg_10rustc_smir Hash = 382993475055910911 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXs1_NtNtNtCs45KzRInYOiI_4core3ops8function5implsQNCINvNvNtNtNtNtBb_4iter6traits8iterator8Iterator8find_map5checkRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateNCNvXNtB2E_10rustc_smirNtB3r_6TablesNtB2C_7Context10find_crate0E0INtB7_5FnMutTuB1P_EE8call_mutB2E_ Hash = 505312707234972298 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtCs1SMo5IIIYUS_9rustc_abi10VariantIdxNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCs8RWOfvZhALO_12rustc_middle3mir5LocalNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRNtNtCs9J0sh8Xtzxx_10rustc_span6symbol6SymbolNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRbNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRjNtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsu_NtCsapyAhDdZmw1_12tracing_core5fieldINtB5_10DebugValueRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumENtB5_5Value6recordCs6YcpMVo69Tg_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsu_NtCsapyAhDdZmw1_12tracing_core5fieldINtB5_10DebugValueRNtNtCsbMYL7gbozeQ_5alloc6string6StringENtB5_5Value6recordCs6YcpMVo69Tg_10rustc_smir Hash = 170957022131388415 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.15: no profile data available for function _RNvXsx_NtCsbMYL7gbozeQ_5alloc5boxedINtB5_3BoxNtNtCs8RWOfvZhALO_12rustc_middle3mir8ConstantENtNtCs45KzRInYOiI_4core3fmt7Display3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.9: no profile data available for function _RINvMsz_NtCs2oODm8Op2Ir_9hashbrown3mapINtB6_15RawEntryBuilderNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdTINtNtNtCs8RWOfvZhALO_12rustc_middle5query5erase6ErasedAhj8_ENtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE6searchNCINvB6_10equivalentBX_BX_E0ECs6YcpMVo69Tg_10rustc_smir Hash = 910108394528351925 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.9: no profile data available for function _RINvXs1x_NtCs2oODm8Op2Ir_9hashbrown3mapINtB7_7HashMapNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEEINtNtNtNtB23_4iter6traits7collect6ExtendTBP_uEE6extendINtNtNtB3v_8adapters3map3MapINtNtB4l_6copied6CopiedINtNtNtB23_5slice4iter4IterBP_EENCINvXs8_NtB9_3setINtB5K_7HashSetBP_B1Y_EIB3p_BP_E6extendB4I_E0EECs6YcpMVo69Tg_10rustc_smir Hash = 146835647075900052 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.9: no profile data available for function _RNvMs1_NtCs2oODm8Op2Ir_9hashbrown3mapINtB5_7HashMapNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexuINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEE6insertCs6YcpMVo69Tg_10rustc_smir Hash = 11922956408974369 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB16_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemEEB1h_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEEB1j_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir9CrateItemEEB1o_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEEB1q_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtBP_10stable_mir9CrateItemNCNvBN_10crate_item0E00EBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdNCNvBN_11item_def_id0E00EBP_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateEBK_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty6TyKindEBM_ Hash = 784007056844089447 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body4BodyEBO_ Hash = 1096621589765811986 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRbECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRjECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placejECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNSNvYNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtBd_10stable_mir9CrateItemNCNvBb_10crate_item0E00INtNtNtCs45KzRInYOiI_4core3ops8function6FnOnceTQNtNtBd_10rustc_smir6TablesEE9call_once6vtableBd_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNSNvYNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdNCNvBb_11item_def_id0E00INtNtNtCs45KzRInYOiI_4core3ops8function6FnOnceTQNtNtBd_10rustc_smir6TablesEE9call_once6vtableBd_ Hash = 742261418966908927 up to 0 count discarded
warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb19bfaeaf1194325E Hash = 1124680650125156080 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h65a613c241761a63E Hash = 742261418966908927 up to 0 count discarded


warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtB8_10stable_mir9CrateItemNCNvB6_10crate_item0E00B8_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNCNCINvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11with_tablesNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdNCNvB6_11item_def_id0E00B8_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvMNtCs6YcpMVo69Tg_10rustc_smir10stable_mirNtB2_9CrateItem4body Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir8entry_fn Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir11local_crate Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir10find_crate Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir15external_crates Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir15all_local_items Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir4withNtNtB2_2ty6TyKindNCNvMBN_NtBN_2Ty4kind0EB4_ Hash = 264495435228823219 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir4withuNCINvNtB4_14rustc_internal11with_tablesNtB2_9CrateItemNCNvBR_10crate_item0E0EB4_ Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RINvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir4withuNCINvNtB4_14rustc_internal11with_tablesNtNtCs9J0sh8Xtzxx_10rustc_span6def_id5DefIdNCNvBR_11item_def_id0E0EB4_ Hash = 212526878233036805 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvXs4_NtCs6YcpMVo69Tg_10rustc_smir10stable_mirNtB5_5CrateNtNtCs45KzRInYOiI_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvXsa_NtCs6YcpMVo69Tg_10rustc_smir10stable_mirNtB5_9CrateItemNtNtCs45KzRInYOiI_4core3fmt5Debug3fmt Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3TLV7___getit Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.6: no profile data available for function _RNvNvNvNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3TLV7___getit7destroy Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc3vec3VecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1l_ Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVecNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEEB1s_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs8RWOfvZhALO_12rustc_middle3mir14BasicBlockDataNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockuNCNvXNtB2k_10rustc_smirNtB3n_6TablesNtB2i_7Context8mir_body0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2c_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5m_3VecB2c_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3i_EE0E0E0EB2k_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs8RWOfvZhALO_12rustc_middle3mir9LocalDeclNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyuNCNvXNtB2c_10rustc_smirNtB30_6TablesNtB2a_7Context8mir_bodys_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB51_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2V_EE0E0E0EB2c_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs8RWOfvZhALO_12rustc_middle3mir9StatementNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementuNvNtB2e_10rustc_smir28rustc_statement_to_statementNCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB54_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3a_EE0E0E0EB2e_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateuNCNvXNtB2a_10rustc_smirNtB2Y_6TablesNtB28_7Context15external_crates0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB26_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB55_3VecB26_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB2T_EE0E0E0EB2a_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters3map8map_foldRNtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax7OperandNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperanduNCNvNtB2l_10rustc_smir30rustc_terminator_to_terminators_0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2d_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5g_3VecB2d_E14extend_trustedINtBL_3MapINtNtNtB4_5slice4iter4IterB1m_EB3f_EE0E0E0EB2l_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10TerminatorEBO_ Hash = 567991000227110385 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body13AssertMessageEBO_ Hash = 647472252099359144 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body5PlaceEBO_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandEBO_ Hash = 238984481002779099 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyEENCNvMs_NtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2u_6Tables14rustc_ty_to_ty0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3D_8for_each4callNtNtNtB2w_10stable_mir2ty2TyNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5h_3VecB4G_E14extend_trustedBN_E0E0EB2w_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEENCINvXs8_NtCs2oODm8Op2Ir_9hashbrown3setINtB32_7HashSetB1J_INtNtBc_4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEEINtNtNtBa_6traits7collect6ExtendB1J_E6extendBX_E0ENtNtB54_8iterator8Iterator4folduNCINvNvB5N_8for_each4callTB1J_uENCINvXs1x_NtB34_3mapINtB6Z_7HashMapB1J_uB3P_EIB50_B6I_E6extendBN_E0E0ECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir14BasicBlockDataENCNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2j_6TablesNtNtB2l_10stable_mir7Context8mir_body0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3N_8for_each4callNtNtNtB3c_3mir4body10BasicBlockNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5u_3VecB4Q_E14extend_trustedBN_E0E0EB2l_ Hash = 650973720849546769 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir9LocalDeclENCNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2d_6TablesNtNtB2f_10stable_mir7Context8mir_bodys_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3J_8for_each4callNtNtB36_2ty2TyNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB59_3VecB4M_E14extend_trustedBN_E0E0EB2f_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle3mir9StatementENvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir28rustc_statement_to_statementENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3j_8for_each4callNtNtNtNtB2c_10stable_mir3mir4body9StatementNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5c_3VecB4m_E14extend_trustedBN_E0E0EB2c_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumENCNvXNtCs6YcpMVo69Tg_10rustc_smir10rustc_smirNtB2d_6TablesNtNtB2f_10stable_mir7Context15external_crates0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3P_8for_each4callNtB36_5CrateNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5d_3VecB4S_E14extend_trustedBN_E0E0EB2f_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.12: no profile data available for function _RINvXs0_NtNtNtCs45KzRInYOiI_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax7OperandENCNvNtCs6YcpMVo69Tg_10rustc_smir10rustc_smir30rustc_terminator_to_terminators_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3x_8for_each4callNtNtNtNtB2l_10stable_mir3mir4body7OperandNCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5o_3VecB4A_E14extend_trustedBN_E0E0EB2l_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNCINvNtNtNtB4_4iter8adapters6copied9copy_foldNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyuNCINvNtBN_3map8map_foldB1p_NtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyuNCNvMs_NtB2z_10rustc_smirNtB3p_6Tables14rustc_ty_to_ty0NCINvNvNtNtNtBP_6traits8iterator8Iterator8for_each4callB2t_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5h_3VecB2t_E14extend_trustedINtB27_3MapINtBL_6CopiedINtNtNtB4_5slice4iter4IterB1p_EEB3i_EE0E0E0E0EB2z_ Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RINvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_NtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyuNCNvMs_NtB3h_10rustc_smirNtB47_6Tables14rustc_ty_to_ty0NCINvNvB26_8for_each4callB3b_NCINvMsi_NtCsbMYL7gbozeQ_5alloc3vecINtB5v_3VecB3b_E14extend_trustedINtB2P_3MapBP_B40_EE0E0E0EB3h_ Hash = 798733564678499214 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RINvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldB1s_TB1s_uEuNCINvXs8_NtCs2oODm8Op2Ir_9hashbrown3setINtB3Y_7HashSetB1s_INtNtBb_4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherEEINtNtB2G_7collect6ExtendB1s_E6extendBP_E0NCINvNvB2C_8for_each4callB3H_NCINvXs1x_NtB40_3mapINtB7d_7HashMapB1s_uB4L_EIB5W_B3H_E6extendINtB3l_3MapBP_B3P_EE0E0E0ECs6YcpMVo69Tg_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvXs3_NtNtCs45KzRInYOiI_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6YcpMVo69Tg_10rustc_smir Hash = 2202714313002754 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtCs8RWOfvZhALO_12rustc_middle2ty2TyEENtNtNtB8_6traits8iterator8Iterator9size_hintCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexEENtNtNtB8_6traits8iterator8Iterator9size_hintCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal11item_def_id Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal10crate_item Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.8: no profile data available for function _RNvNtCs6YcpMVo69Tg_10rustc_smir14rustc_internal9crate_num Hash = 784007058953177093 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeINtNtCsbMYL7gbozeQ_5alloc7raw_vec6RawVechEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtB4_4cell14BorrowMutErrorECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeNtNtCsbMYL7gbozeQ_5alloc6string6StringECs6YcpMVo69Tg_10rustc_smir Hash = 238984481941143025 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvXs_NtNtNtCs45KzRInYOiI_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_5slice4iter4IterNtNtCs9J0sh8Xtzxx_10rustc_span6def_id8CrateNumEB10_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2g_8find_map5checkRB1q_NtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir5CrateNCNvXNtB3x_10rustc_smirNtB4k_6TablesNtB3v_7Context10find_crate0E0INtNtNtBb_3ops12control_flow11ControlFlowB3t_EEB3x_ Hash = 629592967359852085 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RINvYINtNtCs45KzRInYOiI_4core4hash18BuildHasherDefaultNtCsjIf1Vgyp9mf_10rustc_hash8FxHasherENtB6_11BuildHasher8hash_oneRNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RNvXCs2oODm8Op2Ir_9hashbrownNtNtNtCs42j8y7zXj0q_18rustc_query_system9dep_graph5graph12DepNodeIndexINtB2_10EquivalentBq_E10equivalentCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.11: no profile data available for function _RNvXsP_NtCs45KzRInYOiI_4core3fmtRINtNtNtCs8RWOfvZhALO_12rustc_middle2ty4list4ListINtNtNtBC_3mir6syntax14ProjectionElemNtB1m_5LocalNtBA_2TyEENtB5_5Debug3fmtCs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRINtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax14ProjectionElemNtB19_5LocalNtNtB1b_2ty2TyEINtNtNtBa_5slice4iter4IterB14_EECs6YcpMVo69Tg_10rustc_smir Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyINtNtNtBa_5slice4iter4IterB14_EEB1a_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body10BasicBlockINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body12SwitchTargetINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body7OperandINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvMs5_NtNtCs45KzRInYOiI_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir3mir4body9StatementINtNtNtBa_5slice4iter4IterB14_EEB1c_ Hash = 146835646621254984 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRINtNtNtCs8RWOfvZhALO_12rustc_middle3mir6syntax14ProjectionElemNtBM_5LocalNtNtBO_2ty2TyEECs6YcpMVo69Tg_10rustc_smir Hash = 742261418966908927 up to 0 count discarded

warning: rustc_smir.5131fa049a35b038-cgu.7: no profile data available for function _RINvNtCs45KzRInYOiI_4core3ptr13drop_in_placeRNtNtNtCs6YcpMVo69Tg_10rustc_smir10stable_mir2ty2TyEBN_ Hash = 742261418966908927 up to 0 count discarded

---
Total duration:                       1h 35m 18s
------------------------------------------------
root INFO: Free disk space: 460.93 GiB out of total 581.32 GiB (20.71% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 901, in <module>
    raise e
  File "../src/ci/stage-build.py", line 898, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 880, in execute_build_pipeline
    pipeline.run_tests()
  File "../src/ci/stage-build.py", line 166, in run_tests
    rustc_dir = extract_dist_directory(dist_dir, f"rustc-1*-dev-{PGO_HOST}")
  File "../src/ci/stage-build.py", line 258, in extract_dist_directory
    dist_archive = Path(glob.glob(str(glob_path))[0])
