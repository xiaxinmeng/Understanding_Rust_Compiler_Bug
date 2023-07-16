
./configure --enable-rustbuild --target=x86_64-unknown-linux-uclibc

# Rinse and repeat this command until the compile of libstd works will likely require
# local modifications to liblibc and libstd
python src/bootstrap/bootstrap.py --stage 1 --step libtest --target x86_64-unknown-linux-uclibc

# Build the standard library for the host
python src/bootstrap/bootstrap.py --stage 1 --step libtest --target x86_64-unknown-linux-gnu

# point Cargo at that compiler
export RUSTC=`pwd`/build/x86_64-unkown-linux-gnu/stage1/bin/rustc

# Get liblibc tests working
cd path/to/libc/libc-test
cargo run --target x86_64-unknown-linux-uclibc
