bash
#!/usr/bin/env bash

set -o errexit

RUSTC=./build/x86_64-unknown-linux-gnu/stage1/bin/rustc
TEST=test.rs
FLAGS="-Copt-level=3 -Ccodegen-units=1 -C save-temps"
COUNT=100000000

rm -rf *.profraw *.profdata

$RUSTC $FLAGS -Cpgo-gen=default.profraw $TEST -o test-pgo-gen
./test-pgo-gen $COUNT

./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -output=pgo.profdata default.profraw

$RUSTC $FLAGS -Cpgo-use=pgo.profdata $TEST -o test-pgo-use
$RUSTC $FLAGS $TEST -o test-no-pgo

time ./test-no-pgo $COUNT
time ./test-pgo-use $COUNT
