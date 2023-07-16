
error: linking with `cc` failed: exit code: 1
note: "cc" "-Wl,--as-needed" "-m64" "-L" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "foo.0.o" "-o" "foo" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/andrew/.rust/lib/x86_64-unknown-linux-gnu" "-L" "/home/andrew/lib/x86_64-unknown-linux-gnu" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-8cf6ce90.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-8cf6ce90.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-8cf6ce90.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-8cf6ce90.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-8cf6ce90.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-8cf6ce90.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-8cf6ce90.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-8cf6ce90.rlib" "-l" "dl" "-l" "pthread" "-l" "gcc_s" "-l" "rt" "-l" "pthread" "-l" "c" "-l" "m" "-l" "compiler-rt"
note: foo.0.o: In function `main':
foo.0.rs:(.text.main+0x17): undefined reference to `__tsan_func_entry'
foo.0.rs:(.text.main+0x39): undefined reference to `__tsan_func_exit'
foo.0.o: In function `tsan.module_ctor':
foo.0.rs:(.text.tsan.module_ctor+0x2): undefined reference to `__tsan_init'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
