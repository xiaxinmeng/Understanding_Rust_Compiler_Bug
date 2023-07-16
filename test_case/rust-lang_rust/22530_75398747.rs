
/home/rprichard/android-ndk-standalone-19/bin/arm-linux-androideabi-gcc
    -Wl,--as-needed
    -Wl,--allow-multiple-definition
    -L /home/rprichard/work/rust-android-local/lib/rustlib/arm-linux-androideabi/lib
    -o sepcomp-lib-lto
    sepcomp-lib-lto.o
    -Wl,--whole-archive -lmorestack -Wl,--no-whole-archive
    -Wl,--gc-sections
    -nodefaultlibs
    /tmp/rustc.2wZNcIqEwb2J/libstd-4e7c5e5c.rlib
    /tmp/rustc.2wZNcIqEwb2J/liballoc-4e7c5e5c.rlib
    -L .
    -L /home/rprichard/work/rust-android-local/lib/rustlib/arm-linux-androideabi/lib
    -L /home/rprichard/mess/android-test/.rust/lib/arm-linux-androideabi
    -L /home/rprichard/mess/android-test/lib/arm-linux-androideabi
    -Wl,--whole-archive
    -Wl,-Bstatic
    -Wl,--no-whole-archive
    -Wl,-Bdynamic
    -ldl
    -llog
    -lgcc
    -lc
    -lm
    -Wl,-Map=output.map
    -Wl,--cref
    -lcompiler-rt
