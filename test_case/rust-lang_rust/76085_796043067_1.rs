sh
#!/bin/sh

export CXX=clang++-11
export CC=clang-11
export PROFILE=1
export CARGO_INCREMENTAL=0
export RUSTFLAGS='-A warnings -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort'
export RUSTDOCFLAGS='-Cpanic=abort'
export RUST_TEST_THREADS=2
export TMPDIR=/dev/shm/ 

make -j9 all test profclean

cd test
timeout 30 ./test '[this]'
