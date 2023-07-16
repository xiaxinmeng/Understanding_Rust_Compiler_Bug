
./configure --prefix=/foo/bar/rustc32 --disable-valgrind --enable-optimize \
  --enable-optimize-cxx --enable-optimize-llvm --enable-optimize-tests \
  --build=i686-unknown-linux-gnu --host=i586-unknown-linux-gnu \
  --target=i586-unknown-linux-gnu --disable-docs --disable-compiler-docs
