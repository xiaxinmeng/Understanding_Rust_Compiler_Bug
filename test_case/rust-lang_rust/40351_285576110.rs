diff
$ diff -u <(/usr/bin/gdb --configuration) <(/usr/local/bin/gdb --configuration)                                                           [6/1883]
--- /dev/fd/63  2017-03-10 15:21:43.586582443 +1100
+++ /dev/fd/62  2017-03-10 15:21:43.586582443 +1100
@@ -1,16 +1,15 @@
 This GDB was configured as follows:
-   configure --host=x86_64-linux-gnu --target=x86_64-linux-gnu
+   configure --host=x86_64-pc-linux-gnu --target=x86_64-pc-linux-gnu
              --with-auto-load-dir=$debugdir:$datadir/auto-load
              --with-auto-load-safe-path=$debugdir:$datadir/auto-load
              --with-expat
-             --with-gdb-datadir=/usr/share/gdb (relocatable)
-             --with-jit-reader-dir=/usr/lib/gdb (relocatable)
+             --with-gdb-datadir=/usr/local/share/gdb (relocatable)
+             --with-jit-reader-dir=/usr/local/lib/gdb (relocatable)
              --without-libunwind-ia64
              --with-lzma
-             --with-python=/usr (relocatable)
+             --with-python=/usr
              --without-guile
-             --with-separate-debug-dir=/usr/lib/debug (relocatable)
-             --with-system-gdbinit=/etc/gdb/gdbinit
+             --with-separate-debug-dir=/usr/local/lib/debug (relocatable)
              --with-babeltrace
