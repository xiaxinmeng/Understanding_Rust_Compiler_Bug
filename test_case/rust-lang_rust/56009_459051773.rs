
rustup install nightly-2018-11-10
rustup install nightly-2018-11-11
rustup install nightly-2019-01-29
git clone https://github.com/mohrezaei/num-integer
cd num-integer
git checkout 5be6ea1
set your RUSTFLAGS="-C target-cpu=native"
cargo +nightly-2018-11-10 bench benchp10_u64_10000_random
cargo +nightly-2018-11-11 bench benchp10_u64_10000_random
cargo +nightly-2019-01-29 bench benchp10_u64_10000_random
