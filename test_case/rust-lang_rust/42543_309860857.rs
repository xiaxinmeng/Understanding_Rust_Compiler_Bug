
./configure \
  --prefix=/usr \
  --release-channel=stable \
  --llvm-root=/usr \
  --enable-llvm-link-shared \
  --disable-codegen-tests \
  --jemalloc-root=/usr/lib \
  --enable-local-rust
python2 ./x.py build --verbose
