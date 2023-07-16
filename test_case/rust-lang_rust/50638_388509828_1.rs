
openat(AT_FDCWD, "x", O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0666) = 3
ioctl(3, FIOCLEX)                       = 0
openat(AT_FDCWD, "y", O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0666) = 4
ioctl(4, FIOCLEX)                       = 0
close(4)                                = 0
close(3)                                = 0
