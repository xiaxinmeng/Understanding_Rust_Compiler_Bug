 text
error: linking with `cc` failed: exit code: 1
note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/steve/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib" "bar.0.o" "-o" "bar" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "." "-L" "/home/steve/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "-L" "/home/steve/tmp" "-l" "foo" "-L" "/home/steve/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "std-fd663c41" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "compiler-rt"
note: bar.0.o: In function `main::h7afac322a1b32d81faa':
bar.0.rs:(.text._ZN4main20h7afac322a1b32d81faaE+0x2): undefined reference to `foo::hfe26232f3a6cc9a1eaa'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
