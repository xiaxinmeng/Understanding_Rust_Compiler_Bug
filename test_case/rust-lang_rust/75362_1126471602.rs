console
$ cd ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib
$ strings libLLVM-14-rust-1.60.0-stable.so | grep -F '/include' | sort -u
/checkout/src/llvm-project/llvm/include/llvm/ADT/GenericCycleImpl.h
/tmp/gcc-5.5.0/libstdc++-v3/../include
/tmp/gcc-build/gcc/include
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/backward
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/bits
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/debug
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/ext
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/x86_64-unknown-linux-gnu/bits
/usr/include
/usr/include/bits
/usr/include/sys
$ strings librustc_driver-75e5f32fc3580f6c.so | grep -F '/include' | sort -u
/tmp/gcc-5.5.0/libstdc++-v3/../include
/tmp/gcc-build/gcc/include
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/backward
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/bits
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/debug
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/ext
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/parallel
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/tr1
/tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/x86_64-unknown-linux-gnu/bits
/usr/include
/usr/include/bits
/usr/include/sys
$ strings rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-acf5ff6e9595d982.rlib | grep -F '/include' | sort -u
/rustroot/lib/clang/13.0.0/include
