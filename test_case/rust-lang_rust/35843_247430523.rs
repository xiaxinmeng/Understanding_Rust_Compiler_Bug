
$ strace -e stat patchelf --add-needed libpthread.so.0 rustc
stat("--add-needed", 0x7fffbcb94610)    = -1 ENOENT (No such file or directory)
stat: No such file or directory
+++ exited with 1 +++
