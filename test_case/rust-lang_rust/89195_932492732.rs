sh
rustup install nightly-2021-09-17
rustup install nightly-2021-09-18

# Get the script.
curl -sSO https://gist.githubusercontent.com/eddyb/b2e2cc5121675da718977383100c5d39/raw/192b00ba2e30e0038b3aef57959edf36009f12f9/is_89195_repro.sh
chmod +x is_89195_repro.sh

# Get the relevant source and set the env var used by the script.
git clone https://github.com/linkerd/linkerd2-proxy
export REDUCE_CRATE=linkerd2-proxy/linkerd/app/inbound

# Run the script on its own once to build deps.
./is_89195_repro.sh $REDUCE_CRATE/src/lib.rs

# Install `rust-reduce` (from my fork that supports Rust 2018) and run it.
cargo install --git https://github.com/eddyb/rust-reduce --branch rustfmt-2018
rust-reduce ./is_89195_repro.sh $REDUCE_CRATE/src/lib.rs

# Over an hour later, we have an output (you can stop it sooner though).
# Make a copy just in case `creduce` does something weird.
cp $REDUCE_CRATE/src/lib.rs checkpoint-1-rust-reduce.rs

# Don't want `creduce` to get its hands on the remaining modules'
# original files, after `rust-reduce` inlined them into `lib.rs`,
# so keep only `lib.rs` (from the copy we just made).
rm $REDUCE_CRATE/src/**.rs
cp checkpoint-1-rust-reduce.rs $REDUCE_CRATE/src/lib.rs

# Finally, run `creduce`, which will (hopefully) do most of the work.
# The `--n 1` flag is important - the script is not parallel-safe!
creduce --not-c --n 1 --timing ./is_89195_repro.sh $REDUCE_CRATE/src/lib.rs
