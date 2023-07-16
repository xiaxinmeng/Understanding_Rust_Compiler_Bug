
$ ls /lib/x86_64-linux-gnu/libpthread.so.0
/lib/x86_64-linux-gnu/libpthread.so.0

$ patchelf --debug --add-needed /lib/x86_64-linux-gnu/libpthread.so.0 rustc
patching ELF file `--add-needed'
Kernel page size is 4096 bytes
stat: No such file or directory
