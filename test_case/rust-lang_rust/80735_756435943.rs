bash
$ strace -ff clang-11 -target riscv64-unknown-linux-gnu -B$RISCV --sysroot=$RISCV/sysroot hello.c -static -lpthread -o hello -v 2>&1 | grep crt1
access("/home/addisoncrump/.local/riscv/crt1.o", F_OK) = -1 ENOENT (No such file or directory)
access("/usr/lib/llvm-11/lib/clang/11.0.0/crt1.o", F_OK) = -1 ENOENT (No such file or directory)
access("/usr/lib/llvm-11/lib/clang/11.0.0/lib/linux/crt1.o", F_OK) = -1 ENOENT (No such file or directory)
access("/usr/lib/llvm-11/bin/../crt1.o", F_OK) = -1 ENOENT (No such file or directory)
access("/home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0/crt1.o", F_OK) = -1 ENOENT (No such file or directory)
access("/home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0/../../../../riscv64-unknown-linux-gnu/lib/crt1.o", F_OK) = -1 ENOENT (No such file or directory)
access("/home/addisoncrump/.local/riscv/sysroot/lib/crt1.o", F_OK) = -1 ENOENT (No such file or directory)
access("/home/addisoncrump/.local/riscv/sysroot/usr/lib/crt1.o", F_OK) = 0
write(2, "/home/addisoncrump/.local/riscv/"..., 54/home/addisoncrump/.local/riscv/sysroot/usr/lib/crt1.o) = 54
[pid 81384] openat(AT_FDCWD, "/home/addisoncrump/.local/riscv/sysroot/usr/lib/crt1.o", O_RDONLY) = 4
