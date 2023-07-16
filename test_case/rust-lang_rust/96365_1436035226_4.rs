
$ cargo rustc --lib
   Compiling issue-96365-repro v0.1.0 (/home/martin/src/issue-96365-repro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
$ readelf -x .debug_gdb_scripts ./target/debug/libissue_96365_repro.so
readelf: Warning: Section '.debug_gdb_scripts' was not dumped because it does not exist
$ cargo rustc --lib -- -C link-args=-Wl,--no-gc-sections
   Compiling issue-96365-repro v0.1.0 (/home/martin/src/issue-96365-repro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
$ readelf -x .debug_gdb_scripts ./target/debug/libissue_96365_repro.so

Hex dump of section '.debug_gdb_scripts':
  0x000f2d68 01676462 5f6c6f61 645f7275 73745f70 .gdb_load_rust_p
  0x000f2d78 72657474 795f7072 696e7465 72732e70 retty_printers.p
  0x000f2d88 7900                                y.
