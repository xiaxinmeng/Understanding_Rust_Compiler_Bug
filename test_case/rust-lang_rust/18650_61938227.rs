
error: linking with `cc` failed: exit code: 1
note: cc '-m64' '-L' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib' '-o' 'rstest' 'rstest.o' '-Wl,-force_load,/usr/local/lib/rustlib/x86_64-apple-darwin/lib/libmorestack.a' '-nodefaultlibs' '-fno-lto' '-Wl,-dead_strip' '/Users/benanderson/Desktop/rstest/librstestlib.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/libnative-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/libstd-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/librand-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/libsync-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/librustrt-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/libcollections-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/libunicode-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/liballoc-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/liblibc-4e7c5e5c.rlib' '/usr/local/lib/rustlib/x86_64-apple-darwin/lib/libcore-4e7c5e5c.rlib' '-L' '.' '-L' '/Users/benanderson/Desktop/rstest/.rust' '-L' '/Users/benanderson/Desktop/rstest' '-lSystem' '-lpthread' '-lc' '-lm' '-lcompiler-rt'
note: ld: warning: directory not found for option '-L/Users/benanderson/Desktop/rstest/.rust'
ld: archive has no table of contents file '/Users/benanderson/Desktop/rstest/librstestlib.rlib' for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error
