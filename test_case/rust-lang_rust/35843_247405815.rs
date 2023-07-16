
$ file rustc
rustc: ELF 32-bit LSB shared object, ARM, EABI5 version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-armhf.so.3, for GNU/Linux 3.2.72, not stripped

$ file $(which patchelf)
/usr/bin/patchelf: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 2.6.32, BuildID[sha1]=bba1e8ce02627bc87d0b313ba0a9d7ed1bfda81c, stripped

$ patchelf --add-needed libpthread.so.0 rustc
stat: No such file or directory
