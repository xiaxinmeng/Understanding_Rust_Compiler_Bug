
cargo install cargo-llvm-lines
# check out this PR's branch
RUSTFLAGS="--emit=llvm-ir" CARGOFLAGS="-Ztimings" ./x.py clean
RUSTFLAGS="--emit=llvm-ir" CARGOFLAGS="-Ztimings" ./x.py build --stage 0 library/std
cargo llvm-lines --files ./build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/*.ll > llvm-lines-std-before.txt

# Check out the commit before that
RUSTFLAGS="--emit=llvm-ir" CARGOFLAGS="-Ztimings" ./x.py clean
RUSTFLAGS="--emit=llvm-ir" CARGOFLAGS="-Ztimings" ./x.py build --stage 0 library/std
cargo llvm-lines --files ./build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/*.ll > llvm-lines-std-after.txt
