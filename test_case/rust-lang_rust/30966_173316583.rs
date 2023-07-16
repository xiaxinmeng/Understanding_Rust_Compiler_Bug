
rustc --target=mips-unknown-linux-gnu  hello.rs -C linker=mips-openwrt-linux-uclibc-gcc
objdump -p hello | grep NEEDED
  NEEDED               libdl.so.0
  NEEDED               libpthread.so.0
  NEEDED               libgcc_s.so.1
  NEEDED               libc.so.0
  NEEDED               ld-uClibc.so.0
  NEEDED               libm.so.0
