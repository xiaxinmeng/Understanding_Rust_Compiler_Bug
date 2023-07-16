
open("/dev/net/tun", O_WRONLY|O_CLOEXEC) = 4
ioctl(4, FIOCLEX)                       = 0
write(1, "One day I'll catch you!\n", 24One day I'll catch you!
) = 24
ioctl(4, TUNSETIFF, 0)                  = -1 EFAULT (Bad address)
write(1, "-1\n", 3-1
)                     = 3
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x7fd76aa0f000, 8192)            = 0
exit_group(0)                           = ?
+++ exited with 0 +++
