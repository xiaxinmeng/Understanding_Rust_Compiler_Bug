 diff
 "ld.lld" \
     "--as-needed" \
     "-z" \
     "noexecstack" \
     "-L" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" \
     "hello.o" \
     "-o" \
     "hello" \
     "--gc-sections" \
-    "-pie" \
     "-L" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" \
     "--Bstatic" \
     "--Bdynamic" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-411f48d3.rlib" \
     "$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-411f48d3.rlib" \
     "-l" \
     "dl" \
     "-l" \
     "pthread" \
     "-l" \
     "gcc_s" \
     "-l" \
     "pthread" \
     "-l" \
     "c" \
     "-l" \
     "m" \
     "-l" \
     "rt" \
     "-l" \
     "util" \
     "-l" \
-    "compiler-rt"
+    "compiler-rt" \
+    -dynamic-linker \
+    /lib64/ld-linux-x86-64.so.2 \
+    -L/usr/lib/gcc/x86_64-linux-gnu/5 \
+    -L/usr/lib/gcc/x86_64-linux-gnu/5/../../../x86_64-linux-gnu \
+    /usr/lib/gcc/x86_64-linux-gnu/5/../../../x86_64-linux-gnu/crt1.o \
+    /usr/lib/gcc/x86_64-linux-gnu/5/../../../x86_64-linux-gnu/crti.o \
+    /usr/lib/gcc/x86_64-linux-gnu/5/../../../x86_64-linux-gnu/crtn.o
