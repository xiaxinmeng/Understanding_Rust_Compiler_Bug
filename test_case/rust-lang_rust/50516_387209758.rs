
mkdirat(AT_FDCWD, "hello", 0777)        = 0
mkdirat(AT_FDCWD, "hello", 0777)        = -1 EEXIST (File exists)
newfstatat(AT_FDCWD, "hello", {st_mode=S_IFDIR|0775, st_size=4096, ...}, 0) = 0
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=16384}, NULL) = 0
munmap(0xffffb154a000, 16384)           = 0
exit_group(0)                           = ?
+++ exited with 0 +++
