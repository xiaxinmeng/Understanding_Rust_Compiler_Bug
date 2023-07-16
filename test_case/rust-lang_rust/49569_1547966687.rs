sh
dd if=/dev/zero of=output.dat bs=1M count=1536
1536+0 records in
1536+0 records out
1610612736 bytes (1.6 GB, 1.5 GiB) copied, 2.18666 s, 737 MB/s
michal@vm01:~/projects/tests/test_playground$ cargo run --release
   Compiling test_playground v0.1.0 (/home/michal/projects/tests/test_playground)
error: could not compile `test_playground` (bin "test_playground")

Caused by:
  process didn't exit successfully: `/home/michal/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name test_playground --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=157 --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto=thin -C metadata=4ccb91c3f98e9471 -C extra-filename=-4ccb91c3f98e9471 --out-dir /home/michal/projects/tests/test_playground/target/release/deps -L dependency=/home/michal/projects/tests/test_playground/target/release/deps` (signal: 9, SIGKILL: kill)
