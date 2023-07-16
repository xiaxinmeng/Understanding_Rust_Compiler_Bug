
[…]
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fa8d3862000
sigaltstack({ss_sp=0x7fa8d3862000, ss_flags=0, ss_size=8192}, NULL) = 0
getrandom("\x05\x08\x66\x91\x2a\xf0\x05\xf9\x44\x34\x7c\x27\xf3\xfe\x06\x4c", 16, GRND_NONBLOCK) = 16
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x7fa8d3862000, 8192)            = 0
exit_group(0)                           = ?
+++ exited with 0 +++
