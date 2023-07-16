
$ strace ./foo | head -n 3
execve("./foo", ["./foo"], [/* 26 vars */]) = 0
brk(0)                                  = 0x228a000
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f6572c6b000
access("/etc/ld.so.preload", R_OK)      = -1 ENOENT (No such file or directory)
open("/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
fstat(3, {st_mode=S_IFREG|0644, st_size=92196, ...}) = 0
mmap(NULL, 92196, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7f6572c54000
close(3)                                = 0
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
open("/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\360\36\2\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=1853400, ...}) = 0
mmap(NULL, 3961912, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f6572683000
mprotect(0x7f6572840000, 2097152, PROT_NONE) = 0
mmap(0x7f6572a40000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1bd000) = 0x7f6572a40000
mmap(0x7f6572a46000, 17464, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7f6572a46000
close(3)                                = 0
mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f6572c53000
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f6572c51000
arch_prctl(ARCH_SET_FS, 0x7f6572c51740) = 0
mprotect(0x7f6572a40000, 16384, PROT_READ) = 0
mprotect(0x600000, 4096, PROT_READ)     = 0
mprotect(0x7f6572c6d000, 4096, PROT_READ) = 0
munmap(0x7f6572c54000, 92196)           = 0
fstat(1, {st_mode=S_IFIFO|0600, st_size=0, ...}) = 0
mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f6572c6a000
write(1, "test\n", 5)                   = 5
write(1, "test\n", 5test
test
)                   = 5
write(1, "test\n", 5test
)                   = 5
write(1, "test\n", 5)                   = -1 EPIPE (Broken pipe)
--- SIGPIPE {si_signo=SIGPIPE, si_code=SI_USER, si_pid=24113, si_uid=1000} ---
+++ killed by SIGPIPE +++
