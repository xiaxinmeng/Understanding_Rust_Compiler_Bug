
~/Desktop/rust/gbasm $ strace -f -ff ./target/gbasm
execve("./target/gbasm", ["./target/gbasm"], [/* 43 vars */]) = 0
brk(0)                                  = 0xb9173000
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
mmap2(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb7663000
access("/etc/ld.so.preload", R_OK)      = -1 ENOENT (No such file or directory)
open("/etc/ld.so.cache", O_RDONLY)      = 3
fstat64(3, {st_mode=S_IFREG|0644, st_size=121480, ...}) = 0
mmap2(NULL, 121480, PROT_READ, MAP_PRIVATE, 3, 0) = 0xb7645000
close(3)                                = 0
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
open("/lib/tls/i686/cmov/libdl.so.2", O_RDONLY) = 3
read(3, "\177ELF\1\1\1\0\0\0\0\0\0\0\0\0\3\0\3\0\1\0\0\0@\n\0\0004\0\0\0"..., 512) = 512
fstat64(3, {st_mode=S_IFREG|0644, st_size=9736, ...}) = 0
mmap2(NULL, 12408, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0xb7641000
mmap2(0xb7643000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1) = 0xb7643000
close(3)                                = 0
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
open("/lib/tls/i686/cmov/libpthread.so.0", O_RDONLY) = 3
read(3, "\177ELF\1\1\1\0\0\0\0\0\0\0\0\0\3\0\3\0\1\0\0\0 J\0\0004\0\0\0"..., 512) = 512
fstat64(3, {st_mode=S_IFREG|0755, st_size=117086, ...}) = 0
mmap2(NULL, 98792, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0xb7628000
mmap2(0xb763d000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x14) = 0xb763d000
mmap2(0xb763f000, 4584, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0xb763f000
close(3)                                = 0
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
open("/lib/tls/i686/cmov/librt.so.1", O_RDONLY) = 3
read(3, "\177ELF\1\1\1\0\0\0\0\0\0\0\0\0\3\0\3\0\1\0\0\0\300\30\0\0004\0\0\0"..., 512) = 512
fstat64(3, {st_mode=S_IFREG|0644, st_size=30684, ...}) = 0
mmap2(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb7627000
mmap2(NULL, 33364, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0xb761e000
mmap2(0xb7625000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x6) = 0xb7625000
close(3)                                = 0
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
open("/lib/libgcc_s.so.1", O_RDONLY)    = 3
read(3, "\177ELF\1\1\1\0\0\0\0\0\0\0\0\0\3\0\3\0\1\0\0\0p#\0\0004\0\0\0"..., 512) = 512
fstat64(3, {st_mode=S_IFREG|0644, st_size=120368, ...}) = 0
mmap2(NULL, 123432, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0xb75ff000
mmap2(0xb761c000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1c) = 0xb761c000
close(3)                                = 0
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
open("/lib/tls/i686/cmov/libc.so.6", O_RDONLY) = 3
read(3, "\177ELF\1\1\1\0\0\0\0\0\0\0\0\0\3\0\3\0\1\0\0\0Pm\1\0004\0\0\0"..., 512) = 512
fstat64(3, {st_mode=S_IFREG|0755, st_size=1438276, ...}) = 0
mmap2(NULL, 1448360, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0xb749d000
mmap2(0xb75f9000, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x15b) = 0xb75f9000
mmap2(0xb75fc000, 10664, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0xb75fc000
close(3)                                = 0
access("/etc/ld.so.nohwcap", F_OK)      = -1 ENOENT (No such file or directory)
open("/lib/tls/i686/cmov/libm.so.6", O_RDONLY) = 3
read(3, "\177ELF\1\1\1\0\0\0\0\0\0\0\0\0\3\0\3\0\1\0\0\0`4\0\0004\0\0\0"..., 512) = 512
fstat64(3, {st_mode=S_IFREG|0644, st_size=149392, ...}) = 0
mmap2(NULL, 151680, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0xb7477000
mmap2(0xb749b000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x23) = 0xb749b000
close(3)                                = 0
mmap2(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb7476000
mmap2(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb7475000
set_thread_area({entry_number:-1 -> 6, base_addr:0xb7475750, limit:1048575, seg_32bit:1, contents:0, read_exec_only:0, limit_in_pages:1, seg_not_present:0, useable:1}) = 0
mprotect(0xb749b000, 4096, PROT_READ)   = 0
mprotect(0xb75f9000, 8192, PROT_READ)   = 0
mprotect(0xb761c000, 4096, PROT_READ)   = 0
mprotect(0xb7625000, 4096, PROT_READ)   = 0
mprotect(0xb763d000, 4096, PROT_READ)   = 0
mprotect(0xb7643000, 4096, PROT_READ)   = 0
mprotect(0xb7707000, 12288, PROT_READ)  = 0
mprotect(0xb7681000, 4096, PROT_READ)   = 0
munmap(0xb7645000, 121480)              = 0
set_tid_address(0xb74757b8)             = 7979
set_robust_list(0xb74757c0, 0xc)        = 0
futex(0xbffd5c10, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xbffd5c10, FUTEX_WAIT_BITSET_PRIVATE|FUTEX_CLOCK_REALTIME, 1, NULL, bffd5c20) = -1 EAGAIN (Resource temporarily unavailable)
rt_sigaction(SIGRTMIN, {0xb762c410, [], SA_SIGINFO}, NULL, 8) = 0
rt_sigaction(SIGRT_1, {0xb762c8f0, [], SA_RESTART|SA_SIGINFO}, NULL, 8) = 0
rt_sigprocmask(SIG_UNBLOCK, [RTMIN RT_1], NULL, 8) = 0
getrlimit(RLIMIT_STACK, {rlim_cur=8192*1024, rlim_max=RLIM_INFINITY}) = 0
uname({sys="Linux", node="ivo-desktop", ...}) = 0
readlink("/etc/je_malloc.conf", 0xbffd4b80, 4096) = -1 ENOENT (No such file or directory)
brk(0)                                  = 0xb9173000
mmap2(NULL, 4194304, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb7075000
munmap(0xb7075000, 4194304)             = 0
mmap2(NULL, 8384512, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb6c76000
munmap(0xb6c76000, 3710976)             = 0
munmap(0xb7400000, 479232)              = 0
open("/proc/stat", O_RDONLY|O_CLOEXEC)  = 3
read(3, "cpu  459670 4804 172852 2382956 "..., 8192) = 1312
close(3)                                = 0
brk(0xb9194000)                         = 0xb9194000
open("/proc/self/maps", O_RDONLY)       = 3
getrlimit(RLIMIT_STACK, {rlim_cur=8192*1024, rlim_max=RLIM_INFINITY}) = 0
fstat64(3, {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
mmap2(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb7662000
read(3, "b7000000-b7400000 rw-p 00000000 "..., 1024) = 1024
read(3, " 08:07 2366204    /lib/tls/i686/"..., 1024) = 1024
read(3, "gbasm/target/gbasm\nb7707000-b770"..., 1024) = 314
close(3)                                = 0
munmap(0xb7662000, 4096)                = 0
sched_getaffinity(7979, 32,  { f })     = 4
mmap2(0xbf7d7000, 4096, PROT_NONE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0xbf7d7000
rt_sigaction(SIGSEGV, {0xb76b4940, [], SA_STACK|SA_SIGINFO}, NULL, 8) = 0
rt_sigaction(SIGBUS, {0xb76b4940, [], SA_STACK|SA_SIGINFO}, NULL, 8) = 0
mmap2(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb7661000
sigaltstack({ss_sp=0xb7661000, ss_flags=0, ss_size=8192}, NULL) = 0
mmap2(NULL, 4194304, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xb6c00000
rt_sigaction(SIGPIPE, {SIG_IGN, [PIPE], SA_RESTART}, {SIG_DFL, [], 0}, 8) = 0
--- SIGBUS (Bus error) @ 0 (0) ---
rt_sigaction(SIGBUS, {SIG_DFL, [BUS], SA_RESTART}, {0xb76b4940, [], SA_STACK|SA_SIGINFO}, 8) = 0
tgkill(7979, 7979, SIGBUS)              = 0
--- SIGILL (Illegal instruction) @ 0 (0) ---
+++ killed by SIGILL +++
Illegal instruction
