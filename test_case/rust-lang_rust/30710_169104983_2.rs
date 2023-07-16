 sh
$ ./configure --target=arm-unknown-linux-gnueabi --prefix=$HOME/rust-cross --disable-jemalloc
$ make
$ make install
$ LD_LIBRARY_PATH=$HOME/rust-cross/lib $HOME/rust-cross/bin/rustc --version
rustc 1.7.0-dev (4c111e990 2016-01-05)
