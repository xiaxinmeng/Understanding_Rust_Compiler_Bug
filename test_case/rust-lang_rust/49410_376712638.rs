`
git clone https://github.com/sharkdp/fd
cd fd
RUSTFLAGS="-Z pgo-gen=/tmp/fd.pgo -C target-cpu=native" cargo build --release
./target/release/fd
~/LLVM/LLVM6/stage_2/build/bin/llvm-profdata merge -o fd.profdata /tmp/fd.pgo
RUSTFLAGS="-Z pgo-use=/tmp/fd.pgo -C target-cpu=native" cargo build --release
