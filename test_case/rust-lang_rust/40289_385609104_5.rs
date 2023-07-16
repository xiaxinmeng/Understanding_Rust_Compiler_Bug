 console
$ cargo rustc --target thumbv7m-none-eabi -- -C linker=arm-none-eabi-ld -Z linker-flavor=ld -C link-arg=-Tlink.x

$ # note exported symbol: "R"
$ arm-none-eabi-nm -C target/thumbv7m-none-eabi/debug/deps/libfoo-739fb2098c2fa823.rlib
foo-739fb2098c2fa823.18cd33xhzunvqw3h.rcgu.o:
00000000 N
00000041 N
00000050 N
00000066 N
0000006a N
00000073 N
00000077 N
00000080 N
00000000 R STATIC

$ arm-none-eabi-objdump -Cd -j.static target/thumbv7m-none-eabi/debug/bar

target/thumbv7m-none-eabi/debug/bar:     file format elf32-littlearm


Disassembly of section .static:

00000000 <STATIC>:
   0:   01 00 00 00 01 00 00 00 01 00 00 00 01 00 00 00     ................
  10:   01 00 00 00 01 00 00 00 01 00 00 00 01 00 00 00     ................
  20:   01 00 00 00 01 00 00 00                             ........
