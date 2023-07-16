
root@generic_x86:/data/local # strace ./android-socket 
[..]
write(1, "Trying to bind to 0.0.0.0:8080\n", 31Trying to bind to 0.0.0.0:8080
) = 31
socket(PF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_IP) = 3
bind(3, {sa_family=AF_INET, sin_port=htons(8080), sin_addr=inet_addr("0.0.0.0")}, 16) = 0
listen(3, 128)                          = 0
syscall_364(0x3, 0xbf85b6c0, 0xbf85b698, 0x80000, 0x80, 0x10) = -1 (errno 38)
write(1, "error on accept: 38\n", 20error on accept: 38
)   = 20
futex(0xb771722c, FUTEX_WAKE_PRIVATE, 2147483647) = 0
mprotect(0xb771a000, 4096, PROT_READ|PROT_WRITE) = 0
mprotect(0xb771a000, 4096, PROT_READ)   = 0
close(0)                                = 0
close(1)                                = 0
close(2)                                = 0
futex(0xb770f544, FUTEX_WAKE_PRIVATE, 2147483647) = 0
mprotect(0xb771a000, 4096, PROT_READ|PROT_WRITE) = 0
mprotect(0xb771a000, 4096, PROT_READ)   = 0
mprotect(0xb771a000, 4096, PROT_READ|PROT_WRITE) = 0
mprotect(0xb771a000, 4096, PROT_READ)   = 0
munmap(0xb771a000, 4096)                = 0
exit_group(0)                           = ?
+++ exited with 0 +++
