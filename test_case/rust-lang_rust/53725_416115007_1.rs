
[…]
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f072df95000
sigaltstack({ss_sp=0x7f072df95000, ss_flags=0, ss_size=8192}, NULL) = 0
getrandom("", 0, GRND_NONBLOCK)         = 0
getrandom("\x26\x53\x69\xb8\x75\xd4\xe3\x9a\x32\x11\x10\x58\xa7\x8a\x47\x4b", 16, GRND_NONBLOCK) = 16
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x7f072df95000, 8192)            = 0
exit_group(0)                           = ?
+++ exited with 0 +++
