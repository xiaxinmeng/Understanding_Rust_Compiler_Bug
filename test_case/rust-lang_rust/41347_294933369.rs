
thread 'main' panicked at 'couldn't open /var/log/vmkernel.log: other os error None', src/main.rs:58
note: Run with `RUST_BACKTRACE=1` for a backtrace.
[root@clx-vcl-25:/tmp] strace ./logs_tmp
execve("./logs_tmp", ["./logs_tmp"], [/* 18 vars */]) = 0
mmap(NULL, 592, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x66bf50a000
arch_prctl(ARCH_SET_FS, 0x66bf50a100)   = 0
set_tid_address(0x66bf50a138)           = 542517
readlink("/etc/malloc.conf", 0x3165e4e1ca0, 4096) = -1 ENOENT (No such file or directory)
brk(0)                                  = 0x667f50a000
mmap(NULL, 2097152, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x66bf50b000
munmap(0x66bf50b000, 2097152)           = 0
mmap(NULL, 4190208, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x66bf70b000
munmap(0x66bf70b000, 1003520)           = 0
munmap(0x66bfa00000, 1089536)           = 0
sched_getaffinity(0, 128, {ffffffffffff, 0, 0, 0, 0, 0, 0, 0, 0}) = 72
mmap(NULL, 2097152, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x66bfa00000
rt_sigaction(SIGPIPE, {SIG_IGN, [], SA_RESTORER|SA_RESTART, 0x43b913}, {SIG_DFL, [], 0}, 8) = 0
rt_sigprocmask(SIG_UNBLOCK, [RT_1 RT_2], NULL, 8) = 0
rt_sigaction(SIGSEGV, {0x408d20, [], SA_RESTORER|SA_STACK|SA_SIGINFO, 0x43b913}, NULL, 8) = 0
rt_sigaction(SIGBUS, {0x408d20, [], SA_RESTORER|SA_STACK|SA_SIGINFO, 0x43b913}, NULL, 8) = 0
sigaltstack(NULL, {ss_sp=0, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x66bfc00000
sigaltstack({ss_sp=0x66bfc00000, ss_flags=0, ss_size=8192}, NULL) = 0
open("/var/log/vmkernel.log", O_RDONLY|O_CLOEXEC) = 3
fcntl(3, F_SETFD, FD_CLOEXEC)           = 0
ioctl(3, FIOCLEX)                       = -1 ENOSYS (Function not implemented)
close(3)                                = 0
write(2, "thread '", 8thread ')                 = 8
write(2, "main", 4main)                     = 4
write(2, "' panicked at '", 15' panicked at ')         = 15
write(2, "couldn't open /var/log/vmkernel."..., 56couldn't open /var/log/vmkernel.log: other os error None) = 56
write(2, "', ", 3', )                      = 3
write(2, "src/main.rs", 11src/main.rs)             = 11
write(2, ":", 1:)                        = 1
write(2, "58", 258)                       = 2
write(2, "\n", 1
)                       = 1
write(2, "note: Run with `RUST_BACKTRACE=1"..., 51note: Run with `RUST_BACKTRACE=1` for a backtrace.
) = 51
sigaltstack({ss_sp=0, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x66bfc00000, 8192)              = 0
exit_group(101)                         = ?
