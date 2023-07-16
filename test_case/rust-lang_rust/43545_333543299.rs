
$ make install DESTDIR=/dev/shm/bee-root/rustc/rustc-1.19.0-0/image
[…]
   Compiling miniz-sys v0.1.9
   Compiling lzma-sys v0.1.4
   Compiling backtrace-sys v0.1.10
   Compiling flate2 v0.2.19
   Compiling clap v2.19.3
   Compiling error-chain v0.10.0
error: could not find native static library `lzma`, perhaps an -L flag is missing?

error: Could not compile `lzma-sys`.
Build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/dev/shm/bee-root/rustc/rustc-1.19.0-0/build/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "64" "--target" "x86_64-unknown-linux-gnu" "--release" "--frozen" "--manifest-path" "/dev/shm/bee-root/rustc/rustc-1.19.0-0/source/src/tools/rust-installer/Cargo.toml"
expected success, got: exit code: 101
$ file /usr/lib/liblzma.a
/usr/lib/liblzma.a: current ar archive
