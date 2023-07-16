
$ cargo rustc --release --target arm-unknown-linux-gnueabihf -- -C lto -C linker=arm-linux-gnueabihf-gcc
error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
note: /tmp/rustc.UWbdzurzi6jF/liblibrocksdb_sys-4bd5278da99c30b0.rlib: error adding symbols: File format not recognized
