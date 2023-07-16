
execve("/tmp/logs", ["/tmp/logs"], [/* 17 vars */]) = 0
mmap(NULL, 592, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x9d22d01000
arch_prctl(ARCH_SET_FS, 0x9d22d01100)   = 0
set_tid_address(0x9d22d01138)           = 542215
readlink("/etc/malloc.conf", 0x3d49bd45cb0, 4096) = -1 ENOENT (No such file or directory)
brk(0)                                  = 0x9ce2d01000
mmap(NULL, 2097152, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x9d22d02000
munmap(0x9d22d02000, 2097152)           = 0
mmap(NULL, 4190208, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x9d22f02000
munmap(0x9d22f02000, 1040384)           = 0
munmap(0x9d23200000, 1052672)           = 0
sched_getaffinity(0, 128, {ffffffffffff, 0, 0, 0, 0, 0, 0, 0, 0}) = 72
mmap(NULL, 2097152, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x9d23200000
rt_sigaction(SIGPIPE, {SIG_IGN, [], SA_RESTORER|SA_RESTART, 0x43cbb3}, {SIG_DFL, [], 0}, 8) = 0
rt_sigprocmask(SIG_UNBLOCK, [RT_1 RT_2], NULL, 8) = 0
rt_sigaction(SIGSEGV, {0x409c70, [], SA_RESTORER|SA_STACK|SA_SIGINFO, 0x43cbb3}, NULL, 8) = 0
rt_sigaction(SIGBUS, {0x409c70, [], SA_RESTORER|SA_STACK|SA_SIGINFO, 0x43cbb3}, NULL, 8) = 0
sigaltstack(NULL, {ss_sp=0, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x9d23400000
sigaltstack({ss_sp=0x9d23400000, ss_flags=0, ss_size=8192}, NULL) = 0
open("/var/log/vmkernel.log", O_RDONLY) = 3
open("/var/log/vmkernel.log", O_RDONLY) = 4
readv(4, [{"", 0}, {"2017-04-17T20:00:17.365Z cpu8:66"..., 1024}], 2) = 1024
ioctl(1, TIOCGWINSZ, {ws_row=62, ws_col=237, ws_xpixel=0, ws_ypixel=0}) = 0
writev(1, [{"", 0}, {"2017-04-17T20:00:17.365Z cpu8:66"..., 102}], 22017-04-17T20:00:17.365Z cpu8:66686)WARNING: DVFilter: 1199: Couldn't enable keepalive: Not supported
) = 102
close(3)                                = 0
open("./zzz", O_RDONLY|O_CLOEXEC)       = -1 ENOENT (No such file or directory)
write(2, "thread '", 8thread ')                 = 8
write(2, "main", 4main)                     = 4
write(2, "' panicked at '", 15' panicked at ')         = 15
write(2, "called `Result::unwrap()` on an "..., 113called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 2, message: "No such file or directory" } }) = 113
write(2, "', ", 3', )                      = 3
write(2, "/buildslave/rust-buildbot/slave/"..., 88/buildslave/rust-buildbot/slave/stable-dist-rustc-musl-linux/build/src/libcore/result.rs) = 88
write(2, ":", 1:)                        = 1
write(2, "868", 3868)                      = 3
write(2, "\n", 1
)                       = 1
write(2, "note: Run with `RUST_BACKTRACE=1"..., 51note: Run with `RUST_BACKTRACE=1` for a backtrace.
) = 51
sigaltstack({ss_sp=0, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(0x9d23400000, 8192)              = 0
lseek(4, -922, SEEK_CUR)                = 102
exit_group(101)                         = ?
