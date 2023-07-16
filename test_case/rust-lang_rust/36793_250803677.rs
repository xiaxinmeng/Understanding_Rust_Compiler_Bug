
$ rustc +nightly -vV
rustc 1.14.0-nightly (289f3a4ca 2016-09-29)
binary: rustc
commit-hash: 289f3a4ca79916d6445b452fc19a18a1e42a879a
commit-date: 2016-09-29
host: x86_64-unknown-linux-gnu
release: 1.14.0-nightly
$ rustc +nightly foo.rs --target i686-unknown-linux-gnu
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m32" "-L" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib" "foo.0.o" "-o" "foo" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libpanic_unwind-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libunwind-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/librand-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libcollections-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_unicode-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/liballoc-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/liballoc_jemalloc-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/liblibc-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libcore-6eb85298.rlib" "/home/alex/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/libcompiler_builtins-6eb85298.rlib" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "util"
  = note: foo.0.o: In function `main':
foo.cgu-0.rs:(.text.main+0x2d): undefined reference to `std::rt::lang_start::h14cbded5fe3cd915'
collect2: error: ld returned 1 exit status
$ nm -g `rustc +nightly --print sysroot`/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-*.rlib 2>&1 | grep lang_start | c++filt
00000000 T std::rt::lang_start::ha713258a8b5363bc
