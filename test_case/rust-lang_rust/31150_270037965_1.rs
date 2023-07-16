
$ rustc hello.rs
rustc hello.rs --verbose
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m32" "-L" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib" "hello.0.o" "-o" "hello" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libpanic_unwind-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libunwind-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/librand-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libcollections-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_unicode-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/liballoc-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/liballoc_jemalloc-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/liblibc-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libcore-f5a209a9.rlib" "/home/tmbb/.rustup/toolchains/stable-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libcompiler_builtins-f5a209a9.rlib" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "util"
  = note: /usr/bin/ld: cannot find Scrt1.o: No such file or directory
collect2: error: ld returned 1 exit status


error: aborting due to previous error
