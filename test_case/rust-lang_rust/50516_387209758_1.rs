
mkdirat(AT_FDCWD, "hello", 0777)        = 0
mkdirat(AT_FDCWD, "hello", 0777)        = -1 EEXIST (File exists)
write(2, "thread '", 8thread ')                 = 8
write(2, "main", 4main)                     = 4
write(2, "' panicked at '", 15' panicked at ')         = 15
write(2, "2: Os { code: 17, kind: AlreadyE"..., 632: Os { code: 17, kind: AlreadyExists, message: "File exists" }) = 63
write(2, "', ", 3', )                      = 3
write(2, "libcore/result.rs", 17libcore/result.rs)       = 17
write(2, ":", 1:)                        = 1
write(2, "945", 3945)                      = 3
write(2, ":", 1:)                        = 1
write(2, "5", 15)                        = 1
write(2, "\n", 1
)                       = 1
write(2, "note: Run with `RUST_BACKTRACE=1"..., 51note: Run with `RUST_BACKTRACE=1` for a backtrace.
) = 51
futex(0xffffb74cfccc, FUTEX_WAKE_PRIVATE, 2147483647) = 0
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=16384}, NULL) = 0
munmap(0xffffb7534000, 16384)           = 0
exit_group(101)                         = ?
+++ exited with 101 +++
