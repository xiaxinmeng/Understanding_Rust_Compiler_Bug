
$ rustc --version
rustc 1.20.0-nightly (37849a002 2017-06-30)

$ rustc --crate-name=unwind --target arm-unknown-linux-musleabihf --test -C linker=arm-linux-gnueabihf-gcc $(rustc --print sysroot)/lib/rustlib/src/rust/src/libunwind/lib.rs

$ file unwind
unwind: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, BuildID[sha1]=05aec867a6e0d108a33c209fb6f8bf92ccdb069e, not stripped
