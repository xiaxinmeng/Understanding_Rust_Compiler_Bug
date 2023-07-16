
rustc --crate-type=cdylib src/lib.rs 
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "lib.0.o" "-o" "liblib.so" "-Wl,--version-script=/tmp/rustc.AxbnbIEKM7Z3/list" "-Wl,--gc-sections" "-nodefaultlibs" "-L" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b4054fae3db32020.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-1c6ed188684e7d33.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-63f7707126c5a809.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_unicode-a9711770523833d4.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-d2ecc8049920bea8.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-5837d7d3490e00c5.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-0720511b45a7223a.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-34e7f110f175a258.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-ab203041f1ec5313.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-93f19628b61beb76.rlib" "/home/m4b/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-35d2bc471c7ce467.rlib" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "util" "-shared"
  = note: /usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\363' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\240' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\206' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\267' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\342' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\210' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\200' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\360' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\237' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\242' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: ignoring invalid character `\253' in script
/usr/bin/ld:/tmp/rustc.AxbnbIEKM7Z3/list:3: syntax error in VERSION script
collect2: error: ld returned 1 exit status
