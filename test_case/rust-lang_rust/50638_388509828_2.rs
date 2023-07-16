
openat(AT_FDCWD, "x", O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0666) = 3
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
openat(AT_FDCWD, "y", O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0666) = 4
close(4)                                = 0
close(3)                                = 0
