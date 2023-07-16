
#!/bin/sh

./configure \
  --disable-docs \
  --disable-optimize-llvm \
  --disable-jemalloc \
  --prefix=${HOME}/sw/ \
  --local-rust-root=${HOME}/sw/ \
  $@
