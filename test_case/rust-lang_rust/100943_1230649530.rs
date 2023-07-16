plain
[2022-08-29T16:03:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T16:03:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T16:03:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7hDk4K#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark clap-3.1.6 (2/7)
collector error: Failed to profile 'clap-3.1.6' with Eprintln, recorded: expected success, got exit status: 101
