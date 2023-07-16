
$ rm main main.bc main.o main.s
$ rustc src/main.rs --target=armv5te-unknown-linux-musl --emit llvm-bc
$ llc main.bc -o main.s -march=arm -float-abi=soft -mattr=+v5te,+soft-float
$ arm-linux-as main.s -o main.o
$ $HOME/arm-unknown-linux-musl-sysroot "-Wl,--as-needed" "-L" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib" "main.o" "-o" "main" "-Wl,--gc-sections" "-fPIC" "-Wl,-O1" "-nodefaultlibs" "-L" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib/libstd-18402db3.rlib" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib/libcollections-18402db3.rlib" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib/librustc_unicode-18402db3.rlib" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib/librand-18402db3.rlib" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib/liballoc-18402db3.rlib" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib/liballoc_system-18402db3.rlib" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib/liblibc-18402db3.rlib" "$LD_LIBRARY_PATH/rustlib/armv5te-unknown-linux-musl/lib/libcore-18402db3.rlib" "-l" "gcc_s" "-l" "c"
$ file main
main: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-arm.so.1, not stripped
