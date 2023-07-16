sh
#!/bin/bash

set -e

mkdir -p build

FLAGS="--edition=2018 --out-dir build  --emit=dep-info,link -L dependency=./build --extern a=./build/liba.rlib  --extern b=./buil\
d/libb.rlib "

# RUSTC=rustc
RUSTC=./rust-61711/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/bin/rustc

echo "Building a.rs"
$RUSTC --crate-name a a.rs --crate-type lib $FLAGS

echo "Building b.rs"
$RUSTC --crate-name b b.rs --crate-type lib $FLAGS

echo "Building c.rs"
gdb --args $RUSTC --crate-name c c.rs --crate-type bin $FLAGS
