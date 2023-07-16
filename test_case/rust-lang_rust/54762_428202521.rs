
# prepare
export CARGO_INCREMENTAL=1
git reset --hard HEAD && rm target -rf && cargo +stage2.2 build --release
# bench
patch -p1 < 0-println.patch && perf stat -- cargo +stage2.2 build --release
