
andrewrj@andrewrj-guest:~/rust-debug$ grep enable-debug config.mk                                                                                                       [6/1804]
CFG_CONFIGURE_ARGS   := --enable-debug
andrewrj@andrewrj-guest:~/rust-debug$ objdump -j .debug_gdb_scripts -s build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.stage1-x86_64-unknown-linux-gnu

build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.stage1-x86_64-unknown-linux-gnu:     file format elf64-x86-64

objdump: section '.debug_gdb_scripts' mentioned in a -j option, but not found in any input file
andrewrj@andrewrj-guest:~/rust-debug$ ldd build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.stage1-x86_64-unknown-linux-gnu
        linux-vdso.so.1 =>  (0x00007ffe65d09000)
        libstd-1ad65c1c514eed75.so => /home/andrewrj/rust-debug/build/x86_64-unknown-linux-gnu/test/debuginfo/../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-1ad65
c1c514eed75.so (0x00007fd83d1cf000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007fd83cfaf000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fd83cbe8000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007fd83c9e4000)
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007fd83c7da000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007fd83c5bc000)
        /lib64/ld-linux-x86-64.so.2 (0x0000564293e40000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fd83c2b3000)
andrewrj@andrewrj-guest:~/rust-debug$ objdump -j .debug_gdb_scripts -s /home/andrewrj/rust-debug/build/x86_64-unknown-linux-gnu/test/debuginfo/../../stage1/lib/rustlib/x86_64-u
nknown-linux-gnu/lib/libstd-1ad65c1c514eed75.so

/home/andrewrj/rust-debug/build/x86_64-unknown-linux-gnu/test/debuginfo/../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-1ad65c1c514eed75.so:     file format elf64-
x86-64

Contents of section .debug_gdb_scripts:
 4790a3 01676462 5f6c6f61 645f7275 73745f70  .gdb_load_rust_p
 4790b3 72657474 795f7072 696e7465 72732e70  retty_printers.p
 4790c3 79000167 64625f6c 6f61645f 72757374  y..gdb_load_rust
 4790d3 5f707265 7474795f 7072696e 74657273  _pretty_printers
 4790e3 2e707900 01676462 5f6c6f61 645f7275  .py..gdb_load_ru
 4790f3 73745f70 72657474 795f7072 696e7465  st_pretty_printe
 479103 72732e70 79000167 64625f6c 6f61645f  rs.py..gdb_load_
 479113 72757374 5f707265 7474795f 7072696e  rust_pretty_prin
 479123 74657273 2e707900 01676462 5f6c6f61  ters.py..gdb_loa
 479133 645f7275 73745f70 72657474 795f7072  d_rust_pretty_pr
 479143 696e7465 72732e70 79000167 64625f6c  inters.py..gdb_l
 479153 6f61645f 72757374 5f707265 7474795f  oad_rust_pretty_
 479163 7072696e 74657273 2e707900 01676462  printers.py..gdb
 479173 5f6c6f61 645f7275 73745f70 72657474  _load_rust_prett
 479183 795f7072 696e7465 72732e70 79000167  y_printers.py..g
 479193 64625f6c 6f61645f 72757374 5f707265  db_load_rust_pre
 4791a3 7474795f 7072696e 74657273 2e707900  tty_printers.py.
 4791b3 01676462 5f6c6f61 645f7275 73745f70  .gdb_load_rust_p
 4791c3 72657474 795f7072 696e7465 72732e70  retty_printers.p
 4791d3 79000167 64625f6c 6f61645f 72757374  y..gdb_load_rust
 4791e3 5f707265 7474795f 7072696e 74657273  _pretty_printers
 4791f3 2e707900 
