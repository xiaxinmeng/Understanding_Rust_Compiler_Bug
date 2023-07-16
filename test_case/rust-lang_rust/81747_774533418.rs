bash
env RUSTFLAGS="-Csave-temps" ./x.py clean
env RUSTFLAGS="-Csave-temps" ./x.py build --stage 0 library/std
for f in ./build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/*.no-opt.bc; do
  ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-dis "$f";
done
cargo-llvm-lines llvm-lines --files ./build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/*.ll
