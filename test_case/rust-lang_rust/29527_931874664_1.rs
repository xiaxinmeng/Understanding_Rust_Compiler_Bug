
$ cat linkerdriver.wrap 
#!/bin/bash

for arg do
  shift
  case $arg in
    (-lgcc_s) : ;;
       (*) set -- "$@" "$arg" ;;
  esac
done

# replace 'cc' with the linker driver you actually want to invoke (most likely $CC)
exec cc "$@" -lgcc_eh -lc

$ # Change linker env var according to your target
$ CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=$PWD/linkerdriver.wrap cargo build --verbose --package <my-package>

$ ldd target/debug/my-binary
	linux-vdso.so.1 (0x00007ffcd291f000)
	libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f81c9656000)
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f81c9507000)
	libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f81c9501000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f81c930f000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f81ca741000)
