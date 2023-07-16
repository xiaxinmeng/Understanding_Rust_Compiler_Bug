
$ rustc --version
rustc 1.12.0-nightly (28ce3e8a5 2016-08-01)
$ RUST_LOG=debug OUT_DIR=$(pwd)/target/debug/build/ayzim-62448a58f04ea64b/out rustc src/lib.rs --crate-name ayzim --crate-type lib -g --out-dir /home/aidanhs/Desktop/rust/ayzim-opt/target/debug/deps --emit=dep-info,link -L dependency=/home/aidanhs/Desktop/rust/ayzim-opt/target/debug/deps [...] --verbose -Z verbose -Z time--passes
time: 0.025; rss: 67MB  parsing
time: 0.004; rss: 71MB  configuration
[...]
time: 0.020; rss: 143MB wf checking
time: 0.015; rss: 145MB item-types checking
