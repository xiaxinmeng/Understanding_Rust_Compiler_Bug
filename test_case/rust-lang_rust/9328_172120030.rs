
/tmp/foo $ cargo new --bin foobar

/tmp/foo $ cd foobar 

/tmp/foo/foobar $ cargo build
   Compiling foobar v0.1.0 (file:///tmp/foo/foobar)

/tmp/foo/foobar $ cargo build --target=arm-unknown-linux-gnueabihf
   Compiling foobar v0.1.0 (file:///tmp/foo/foobar)
error: linking with `cc` failed: exit code: 1
note: "cc" "-Wl,--as-needed" "-L" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o" "-o" "/tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug" "-L" "/tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/deps" "-L" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-L" "/tmp/foo/foobar/.rust/lib/arm-unknown-linux-gnueabihf" "-L" "/tmp/foo/foobar/lib/arm-unknown-linux-gnueabihf" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libstd-35c36e89.rlib" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcollections-35c36e89.rlib" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib/librustc_unicode-35c36e89.rlib" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib/librand-35c36e89.rlib" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib/liballoc-35c36e89.rlib" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib/liballoc_jemalloc-35c36e89.rlib" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib/liblibc-35c36e89.rlib" "/home/wilfred/.multirust/toolchains/stable/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcore-35c36e89.rlib" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "rt" "-l" "pthread" "-l" "c" "-l" "m" "-l" "compiler-rt"
note: /usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/usr/bin/ld: /tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: Relocations in generic ELF (EM: 40)
/tmp/foo/foobar/target/arm-unknown-linux-gnueabihf/debug/foobar.0.o: error adding symbols: File in wrong format
collect2: error: ld returned 1 exit status

error: aborting due to previous error
Could not compile `foobar`.

To learn more, run the command again with --verbose.
