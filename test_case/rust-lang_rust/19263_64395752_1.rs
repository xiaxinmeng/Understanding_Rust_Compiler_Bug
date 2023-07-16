
#! /bin/sh

basedir=${0%/*}
rustdir=`ls -td $basedir/rust-* | head -n 1` # the simplest logic to choose one
if [ -z "$rustdir" ]; then echo "no installation found"; exit 10; fi
bindir=$rustdir/bin
libdir=$rustdir/lib

LD_LIBRARY_PATH=$libdir $bindir/rustc "$@"
