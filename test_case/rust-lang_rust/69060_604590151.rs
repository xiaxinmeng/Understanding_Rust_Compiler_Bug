sh
rm -rf target/release
RUSTFLAGS=-Cmetadata=$N cargo rustc --release -- -Zself-profile
mkdir perf-$N
mv syn-*.{events,string_{data,index}} perf-$N
