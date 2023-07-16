
execve("./a", ["./a"], [/* 33 vars */]) = 0
brk(NULL)                               = 0x55618643c000
access("/etc/ld.so.preload", R_OK)      = -1 ENOENT (No such file or directory)
open("/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
fstat(3, {st_mode=S_IFREG|0644, st_size=314472, ...}) = 0
mmap(NULL, 314472, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7f4f9f7a5000
close(3)                                = 0
open("/usr/lib/libdl.so.2", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\240\r\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=14608, ...}) = 0
mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4f9f7a4000
mmap(NULL, 2109680, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f4f9f3cb000
mprotect(0x7f4f9f3cd000, 2097152, PROT_NONE) = 0
mmap(0x7f4f9f5cd000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x2000) = 0x7f4f9f5cd000
close(3)                                = 0
open("/usr/lib/libpthread.so.0", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\0a\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=143584, ...}) = 0
mmap(NULL, 2212880, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f4f9f1ae000
mprotect(0x7f4f9f1c6000, 2093056, PROT_NONE) = 0
mmap(0x7f4f9f3c5000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x17000) = 0x7f4f9f3c5000
mmap(0x7f4f9f3c7000, 13328, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7f4f9f3c7000
close(3)                                = 0
open("/usr/lib/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0p*\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0644, st_size=725296, ...}) = 0
mmap(NULL, 2185552, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f4f9ef98000
mprotect(0x7f4f9efae000, 2093056, PROT_NONE) = 0
mmap(0x7f4f9f1ad000, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x15000) = 0x7f4f9f1ad000
close(3)                                = 0
open("/usr/lib/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0`\10\2\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=1960968, ...}) = 0
mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4f9f7a3000
mmap(NULL, 3803440, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f4f9ebf7000
mprotect(0x7f4f9ed8e000, 2097152, PROT_NONE) = 0
mmap(0x7f4f9ef8e000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x197000) = 0x7f4f9ef8e000
mmap(0x7f4f9ef94000, 14640, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7f4f9ef94000
close(3)                                = 0
open("/usr/lib/libm.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0pU\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=1063296, ...}) = 0
mmap(NULL, 3158248, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f4f9e8f3000
mprotect(0x7f4f9e9f6000, 2093056, PROT_NONE) = 0
mmap(0x7f4f9ebf5000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x102000) = 0x7f4f9ebf5000
close(3)                                = 0
mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4f9f7a2000
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4f9f7a0000
arch_prctl(ARCH_SET_FS, 0x7f4f9f7a07c0) = 0
mprotect(0x7f4f9ef8e000, 16384, PROT_READ) = 0
mprotect(0x7f4f9ebf5000, 4096, PROT_READ) = 0
mprotect(0x7f4f9f3c5000, 4096, PROT_READ) = 0
mprotect(0x7f4f9f5cd000, 4096, PROT_READ) = 0
mprotect(0x7f4f9f7f2000, 4096, PROT_READ) = 0
munmap(0x7f4f9f7a5000, 314472)          = 0
set_tid_address(0x7f4f9f7a0a90)         = 18917
set_robust_list(0x7f4f9f7a0aa0, 24)     = 0
rt_sigaction(SIGRTMIN, {0x7f4f9f1b3ba0, [], SA_RESTORER|SA_SIGINFO, 0x7f4f9f1bef00}, NULL, 8) = 0
rt_sigaction(SIGRT_1, {0x7f4f9f1b3c30, [], SA_RESTORER|SA_RESTART|SA_SIGINFO, 0x7f4f9f1bef00}, NULL, 8) = 0
rt_sigprocmask(SIG_UNBLOCK, [RTMIN RT_1], NULL, 8) = 0
getrlimit(RLIMIT_STACK, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
readlink("/etc/malloc.conf", 0x7ffda7c2d4c0, 4096) = -1 ENOENT (No such file or directory)
brk(NULL)                               = 0x55618643c000
mmap(NULL, 2097152, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4f9e6f3000
munmap(0x7f4f9e6f3000, 2097152)         = 0
mmap(NULL, 4190208, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4f9e4f4000
munmap(0x7f4f9e4f4000, 1097728)         = 0
munmap(0x7f4f9e800000, 995328)          = 0
open("/sys/devices/system/cpu/online", O_RDONLY|O_CLOEXEC) = 3
read(3, "0-1\n", 8192)                  = 4
close(3)                                = 0
rt_sigaction(SIGPIPE, {SIG_IGN, [PIPE], SA_RESTORER|SA_RESTART, 0x7f4f9ec2a310}, {SIG_DFL, [], 0}, 8) = 0
mmap(NULL, 2097152, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4f9e400000
open("/proc/self/maps", O_RDONLY|O_CLOEXEC) = 3
getrlimit(RLIMIT_STACK, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
fstat(3, {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
read(3, "55618585e000-5561858b1000 r-xp 0"..., 1024) = 1024
read(3, "  /usr/lib/libc-2.23.so\n7f4f9ef9"..., 1024) = 1024
read(3, "f5ce000 r--p 00002000 fe:00 9210"..., 1024) = 876
close(3)                                = 0
sched_getaffinity(18917, 32, [0 1])     = 16
mmap(0x7ffda742f000, 4096, PROT_NONE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7ffda742f000
rt_sigaction(SIGSEGV, {0x556185868600, [], SA_RESTORER|SA_STACK|SA_SIGINFO, 0x7f4f9f1bef00}, NULL, 8) = 0
rt_sigaction(SIGBUS, {0x556185868600, [], SA_RESTORER|SA_STACK|SA_SIGINFO, 0x7f4f9f1bef00}, NULL, 8) = 0
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4f9f7f0000
sigaltstack({ss_sp=0x7f4f9f7f0000, ss_flags=0, ss_size=8192}, NULL) = 0
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x7f4f9f7f0000, 8192)            = 0
exit_group(15731617)                    = ?
+++ exited with 161 +++
