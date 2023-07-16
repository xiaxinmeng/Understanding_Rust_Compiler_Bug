
$ strace -s 0 ./read_to_end large.txt
...
openat(AT_FDCWD, "large.txt", O_RDONLY|O_CLOEXEC) = 3
futex(0x7fb36787f0c8, FUTEX_WAKE_PRIVATE, 2147483647) = 0
statx(0, NULL, AT_STATX_SYNC_AS_STAT, STATX_ALL, NULL) = -1 EFAULT (Bad address)
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|0x1000, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=10100000, ...}) = 0
mmap(NULL, 10100736, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fb366ce4000
read(3, ""..., 10100000)                = 10100000
read(3, "", 32)                         = 0
munmap(0x7fb366ce4000, 10100736)        = 0
close(3)                                = 0
...
