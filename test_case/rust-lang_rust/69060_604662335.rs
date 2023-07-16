sh
mkdir perf-{1,2}
rm perf-{1,2}/*
rm -rf target/release
CARGO_INCREMENTAL=1 cargo +rust-3-stage1 rustc --release -- -Zself-profile=perf-1
rm -rf target/release
CARGO_INCREMENTAL=1 cargo +rust-4-stage1 rustc --release -- -Zself-profile=perf-2
