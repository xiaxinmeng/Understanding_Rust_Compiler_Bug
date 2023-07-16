console
$ readelf -x .debug_gdb_scripts /tmp/bin-out/main /tmp/lib-out/liblib.so

File: /tmp/bin-out/main

Hex dump of section '.debug_gdb_scripts':
  0x00002000 01676462 5f6c6f61 645f7275 73745f70 .gdb_load_rust_p
  0x00002010 72657474 795f7072 696e7465 72732e70 retty_printers.p
  0x00002020 7900                                y.


File: /tmp/lib-out/liblib.so
readelf: Warning: Section '.debug_gdb_scripts' was not dumped because it does not exist
