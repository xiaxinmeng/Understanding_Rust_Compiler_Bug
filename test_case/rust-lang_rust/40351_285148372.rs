
   configure --host=x86_64-pc-linux-gnu --target=x86_64-pc-linux-gnu
             --with-auto-load-dir=$debugdir:$datadir/auto-load
             --with-auto-load-safe-path=$debugdir:$datadir/auto-load
             --with-expat
             --with-gdb-datadir=/usr/local/share/gdb (relocatable)
             --with-jit-reader-dir=/usr/local/lib/gdb (relocatable)
             --without-libunwind-ia64
             --with-lzma
             --with-python=/usr
             --without-guile
             --with-separate-debug-dir=/usr/local/lib/debug (relocatable)
             --with-babeltrace
