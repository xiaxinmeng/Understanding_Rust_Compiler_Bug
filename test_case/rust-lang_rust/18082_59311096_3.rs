
error: linking with `cc` failed: exit code: 1
note: cc '-m64' '-L' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib' '-o' 'app' 'app.o' '-Wl,--whole-archive' '-lmorestack' '-Wl,--no-whole-archive' '-nodefaultlibs' '-fno-lto' '-Wl,--gc-sections' '-pie' '-Wl,--as-needed' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libnative-4e7c5e5c.rlib' '/home/japaric/tmp/libopaque.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/librand-4e7c5e5c.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libsync-4e7c5e5c.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/librustrt-4e7c5e5c.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-4e7c5e5c.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-4e7c5e5c.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libunicode-4e7c5e5c.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4e7c5e5c.rlib' '/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libcore-4e7c5e5c.rlib' '-L' '.' '-L' '/home/japaric/tmp/.rust' '-L' '/home/japaric/tmp' '-Wl,--whole-archive' '-Wl,-Bstatic' '-Wl,--no-whole-archive' '-Wl,-Bdynamic' '-ldl' '-lpthread' '-lgcc_s' '-lpthread' '-lc' '-lm' '-lcompiler-rt'
note: app.o: In function `main::h7437927d0a861856faa':
app.0.rs:(.text._ZN4main20h7437927d0a861856faaE+0x2c): undefined reference to `inaccessible::Struct::kaboom::hbaedc25ee8398ba6jaa'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
