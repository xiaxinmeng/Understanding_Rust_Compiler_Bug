 shell
wget  https://static.rust-lang.org/dist/rustc-beta-src.tar.gz
tar xvf rustc-beta-src.tar.gz
cd rustc-beta
./configure --enable-optimize --disable-jemalloc --llvm-root=/usr --disable-docs --enable-local-rust --local-rust-root=/tmp --prefix=~/rust-beta-1.10 --build=armv7-unknown-linux-gnueabihf
make -j2
make install
