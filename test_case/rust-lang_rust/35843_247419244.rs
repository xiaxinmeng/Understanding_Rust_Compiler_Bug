
$ ls /usr/lib/libpthread.so.0
ls: cannot access '/usr/lib/libpthread.so.0': No such file or directory

$ patchelf --debug --add-needed /usr/lib/libpthread.so.0 rustc
patching ELF file `--add-needed'
Kernel page size is 4096 bytes
stat: No such file or directory
