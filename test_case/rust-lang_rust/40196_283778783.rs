
sigaltstack({ss_sp=0x7fe1f6861000, ss_flags=0, ss_size=8192}, NULL) = 0
open("/dev/net/tun", O_WRONLY|O_CLOEXEC) = 4
ioctl(4, FIOCLEX)                       = 0
ioctl(4, TUNSETIFF, 0x7ffdc5395d90)     = 0
write(1, "0\n", 20
)                      = 2
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x7fe1f6861000, 8192)            = 0
exit_group(0)                           = ?
+++ exited with 0 +++
