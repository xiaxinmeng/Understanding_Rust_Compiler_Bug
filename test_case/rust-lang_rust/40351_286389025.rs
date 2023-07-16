
andrewrj@andrewrj-guest:~/rust$ grep enable-debug config.mk
andrewrj@andrewrj-guest:~/rust$ echo $?
1
andrewrj@andrewrj-guest:~/rust$ objdump -j .debug_gdb_scripts -s build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.stage1-x86_64-unknown-linux-gnu

build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.stage1-x86_64-unknown-linux-gnu:     file format elf64-x86-64

objdump: section '.debug_gdb_scripts' mentioned in a -j option, but not found in any input file
andrewrj@andrewrj-guest:~/rust$ ldd build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.stage1-x86_64-unknown-linux-gnu
        linux-vdso.so.1 =>  (0x00007fff07de1000)
        libstd-dc2a73ed68c282a1.so => /home/andrewrj/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-dc2a73ed68c282a1.so (0x00007f0cdd26f000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f0cdd04f000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f0cdcc88000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f0cdca84000)
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007f0cdc87a000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f0cdc65c000)
        /lib64/ld-linux-x86-64.so.2 (0x000056343c52e000)
andrewrj@andrewrj-guest:~/rust$ objdump -j .debug_gdb_scripts -s /home/andrewrj/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-dc2a73ed68c282a1.so

/home/andrewrj/rust/build/x86_64-unknown-linux-gnu/test/debuginfo/../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-dc2a73ed68c282a1.so:     file format elf64-x86-64

objdump: section '.debug_gdb_scripts' mentioned in a -j option, but not found in any input file
